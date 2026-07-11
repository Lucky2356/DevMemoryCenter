import { lstat, readFile, readlink } from "node:fs/promises";
import { spawnSync } from "node:child_process";
import { isAbsolute, relative, resolve, sep } from "node:path";
import { pathToFileURL } from "node:url";

export const DEFAULT_LIMITS = Object.freeze({
  maxFiles: 20_000,
  maxHistoryBlobs: 50_000,
  maxFileBytes: 1024 * 1024,
  maxWorkingTreeBytes: 64 * 1024 * 1024,
  maxHistoryBytes: 256 * 1024 * 1024,
  maxGitOutputBytes: 16 * 1024 * 1024,
});

const RULES = Object.freeze([
  {
    id: "private-key",
    expression: /-----BEGIN (?:RSA |EC |OPENSSH |DSA )?PRIVATE KEY-----/u,
  },
  {
    id: "github-token",
    expression:
      /\b(?:gh[pousr]_[A-Za-z0-9]{36,255}|github_pat_[A-Za-z0-9_]{40,255})\b/u,
  },
  {
    id: "gitlab-token",
    expression: /\bglpat-[A-Za-z0-9_-]{20,255}\b/u,
  },
  {
    id: "aws-access-key",
    expression: /\b(?:AKIA|ASIA)[A-Z0-9]{16}\b/u,
  },
  {
    id: "slack-token",
    expression: /\bxox[baprs]-[A-Za-z0-9-]{10,255}\b/u,
  },
  {
    id: "jwt",
    expression:
      /\beyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\b/u,
  },
  {
    id: "basic-auth-url",
    expression: /\b[a-z][a-z0-9+.-]*:\/\/[^\s/:@]{1,128}:[^\s/@]{8,256}@/iu,
  },
  {
    id: "assigned-secret",
    expression:
      /\b(?:api[_-]?key|access[_-]?key|client[_-]?secret|password|passwd|secret|token)\s*[=:]\s*["']?(?!<REDACTED>|example|changeme|placeholder|\$\{|%[A-Z_]+%)[A-Za-z0-9+/_=-]{12,512}/iu,
  },
]);

export class SecretScanError extends Error {
  constructor(code, message) {
    super(message);
    this.name = "SecretScanError";
    this.code = code;
  }
}

function checkedLimits(overrides = {}) {
  const limits = { ...DEFAULT_LIMITS, ...overrides };
  for (const [name, value] of Object.entries(limits)) {
    if (!Number.isSafeInteger(value) || value <= 0) {
      throw new SecretScanError(
        "invalid_limits",
        `Invalid scanner limit: ${name}.`,
      );
    }
  }
  return limits;
}

function runGit(cwd, args, input, maxBuffer) {
  const result = spawnSync("git", args, {
    cwd,
    input,
    encoding: null,
    maxBuffer,
    windowsHide: true,
  });
  if (result.error) {
    throw new SecretScanError(
      "git_unavailable",
      "Git could not complete the repository scan.",
    );
  }
  if (result.status !== 0) {
    throw new SecretScanError(
      "git_failed",
      "Git rejected a repository scan operation.",
    );
  }
  return result.stdout;
}

export function scanBytes(bytes) {
  const text = bytes.toString("latin1");
  return RULES.filter(({ expression }) => expression.test(text)).map(
    ({ id }) => id,
  );
}

function parseNullSeparated(output) {
  if (output.length === 0) {
    return [];
  }
  return output
    .toString("utf8")
    .split("\0")
    .filter((entry) => entry.length > 0);
}

function parseObjectIds(output) {
  const objectIds = new Set();
  for (const line of output.toString("ascii").split("\n")) {
    const separator = line.indexOf(" ");
    const objectId = separator === -1 ? line : line.slice(0, separator);
    if (/^[0-9a-f]{40,64}$/u.test(objectId)) {
      objectIds.add(objectId);
    }
  }
  return [...objectIds];
}

function parseObjectMetadata(output) {
  return output
    .toString("ascii")
    .trim()
    .split("\n")
    .filter(Boolean)
    .map((line) => {
      const [objectId, type, sizeText] = line.split(" ");
      const size = Number(sizeText);
      if (
        !/^[0-9a-f]{40,64}$/u.test(objectId) ||
        !Number.isSafeInteger(size) ||
        size < 0
      ) {
        throw new SecretScanError(
          "invalid_git_output",
          "Git returned invalid object metadata.",
        );
      }
      return { objectId, type, size };
    });
}

function scanBatchBlobs(output, expectedBlobs) {
  const findings = [];
  let offset = 0;
  for (const expected of expectedBlobs) {
    const headerEnd = output.indexOf(0x0a, offset);
    if (headerEnd === -1) {
      throw new SecretScanError(
        "invalid_git_output",
        "Git returned truncated blob metadata.",
      );
    }
    const [objectId, type, sizeText] = output
      .subarray(offset, headerEnd)
      .toString("ascii")
      .split(" ");
    const size = Number(sizeText);
    if (
      objectId !== expected.objectId ||
      type !== "blob" ||
      size !== expected.size ||
      !Number.isSafeInteger(size) ||
      size < 0
    ) {
      throw new SecretScanError(
        "invalid_git_output",
        "Git returned unexpected blob metadata.",
      );
    }

    const contentStart = headerEnd + 1;
    const contentEnd = contentStart + size;
    if (contentEnd >= output.length || output[contentEnd] !== 0x0a) {
      throw new SecretScanError(
        "invalid_git_output",
        "Git returned truncated blob content.",
      );
    }
    for (const rule of scanBytes(output.subarray(contentStart, contentEnd))) {
      findings.push({ source: "history", rule, objectId });
    }
    offset = contentEnd + 1;
  }
  if (offset !== output.length) {
    throw new SecretScanError(
      "invalid_git_output",
      "Git returned excess blob content.",
    );
  }
  return findings;
}

async function readWorkingTreeEntry(root, relativePath, limits) {
  const absolutePath = resolve(root, relativePath);
  const containment = relative(root, absolutePath);
  if (
    containment === ".." ||
    containment.startsWith(`..${sep}`) ||
    isAbsolute(containment)
  ) {
    throw new SecretScanError(
      "path_escape",
      "A repository path escaped the work tree.",
    );
  }
  const metadata = await lstat(absolutePath);
  if (metadata.isSymbolicLink()) {
    return Buffer.from(await readlink(absolutePath), "utf8");
  }
  if (!metadata.isFile()) {
    throw new SecretScanError(
      "unsupported_entry",
      "A repository entry is not a regular file.",
    );
  }
  if (metadata.size > limits.maxFileBytes) {
    throw new SecretScanError(
      "file_limit",
      "A repository file exceeds the scan size limit.",
    );
  }
  return readFile(absolutePath);
}

async function scanWorkingTree(root, limits) {
  const output = runGit(
    root,
    ["ls-files", "--cached", "--others", "--exclude-standard", "-z"],
    undefined,
    limits.maxGitOutputBytes,
  );
  const paths = parseNullSeparated(output);
  if (paths.length > limits.maxFiles) {
    throw new SecretScanError(
      "file_count_limit",
      "The repository contains too many files to scan safely.",
    );
  }

  const findings = [];
  let totalBytes = 0;
  for (const relativePath of paths) {
    const bytes = await readWorkingTreeEntry(root, relativePath, limits);
    totalBytes += bytes.length;
    if (totalBytes > limits.maxWorkingTreeBytes) {
      throw new SecretScanError(
        "working_tree_limit",
        "The working tree exceeds the total scan limit.",
      );
    }
    for (const rule of scanBytes(bytes)) {
      findings.push({ source: "working-tree", rule });
    }
  }
  return { findings, filesScanned: paths.length, bytesScanned: totalBytes };
}

function scanHistory(root, limits) {
  const objectsOutput = runGit(
    root,
    ["rev-list", "--objects", "--all"],
    undefined,
    limits.maxGitOutputBytes,
  );
  const objectIds = parseObjectIds(objectsOutput);
  const metadataOutput = runGit(
    root,
    ["cat-file", "--batch-check=%(objectname) %(objecttype) %(objectsize)"],
    Buffer.from(`${objectIds.join("\n")}\n`, "ascii"),
    limits.maxGitOutputBytes,
  );
  const blobs = parseObjectMetadata(metadataOutput).filter(
    ({ type }) => type === "blob",
  );
  if (blobs.length > limits.maxHistoryBlobs) {
    throw new SecretScanError(
      "history_count_limit",
      "Git history contains too many blobs to scan safely.",
    );
  }

  let totalBytes = 0;
  for (const { size } of blobs) {
    if (size > limits.maxFileBytes) {
      throw new SecretScanError(
        "history_blob_limit",
        "A historical blob exceeds the scan size limit.",
      );
    }
    totalBytes += size;
    if (totalBytes > limits.maxHistoryBytes) {
      throw new SecretScanError(
        "history_size_limit",
        "Git history exceeds the total scan limit.",
      );
    }
  }
  if (blobs.length === 0) {
    return { findings: [], blobsScanned: 0, bytesScanned: 0 };
  }
  const batchOutput = runGit(
    root,
    ["cat-file", "--batch"],
    Buffer.from(
      `${blobs.map(({ objectId }) => objectId).join("\n")}\n`,
      "ascii",
    ),
    limits.maxHistoryBytes + limits.maxHistoryBlobs * 100,
  );
  const findings = scanBatchBlobs(batchOutput, blobs);
  return { findings, blobsScanned: blobs.length, bytesScanned: totalBytes };
}

export async function scanRepository({
  cwd = process.cwd(),
  limits: limitOverrides,
} = {}) {
  const limits = checkedLimits(limitOverrides);
  const rootOutput = runGit(
    cwd,
    ["rev-parse", "--show-toplevel"],
    undefined,
    limits.maxGitOutputBytes,
  );
  const rootText = rootOutput.toString("utf8").trim();
  if (!rootText) {
    throw new SecretScanError(
      "invalid_repository",
      "Git did not return a repository root.",
    );
  }
  const root = resolve(rootText);

  const workingTree = await scanWorkingTree(root, limits);
  const history = scanHistory(root, limits);
  return {
    findings: [...workingTree.findings, ...history.findings],
    filesScanned: workingTree.filesScanned,
    workingTreeBytes: workingTree.bytesScanned,
    historyBlobsScanned: history.blobsScanned,
    historyBytes: history.bytesScanned,
  };
}

async function main() {
  try {
    const result = await scanRepository();
    if (result.findings.length > 0) {
      const summaries = new Set(
        result.findings.map(({ source, rule, objectId }) =>
          objectId
            ? `${source}:${rule}:${objectId.slice(0, 12)}`
            : `${source}:${rule}`,
        ),
      );
      console.error(
        `Secret scan rejected ${summaries.size} candidate(s). Values and paths are suppressed.`,
      );
      for (const summary of summaries) {
        console.error(`- ${summary}`);
      }
      process.exitCode = 1;
      return;
    }
    console.log(
      `Secret scan passed: ${result.filesScanned} files and ${result.historyBlobsScanned} historical blobs checked within configured bounds.`,
    );
  } catch (error) {
    const code =
      error instanceof SecretScanError ? error.code : "unexpected_error";
    console.error(
      `Secret scan failed closed (${code}). No repository content was printed.`,
    );
    process.exitCode = 2;
  }
}

if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
) {
  await main();
}
