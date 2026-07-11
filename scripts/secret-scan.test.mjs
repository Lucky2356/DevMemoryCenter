import assert from "node:assert/strict";
import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import test from "node:test";

import { SecretScanError, scanBytes, scanRepository } from "./secret-scan.mjs";

function git(cwd, ...args) {
  const result = spawnSync("git", args, {
    cwd,
    encoding: "utf8",
    windowsHide: true,
  });
  assert.equal(result.status, 0, "synthetic Git fixture should be created");
}

async function withRepository(run) {
  const directory = await mkdtemp(join(tmpdir(), "dev-recall-secret-scan-"));
  try {
    git(directory, "init", "--quiet");
    git(directory, "config", "user.name", "Fixture Author");
    git(directory, "config", "user.email", "fixture@example.invalid");
    await run(directory);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
}

test("detects representative fake credentials without returning their values", () => {
  const fakeGitHubToken = ["ghp_", "A".repeat(36)].join("");
  const fakePrivateKey = ["-----BEGIN ", "PRIVATE KEY-----"].join("");
  assert.deepEqual(scanBytes(Buffer.from(fakeGitHubToken)), ["github-token"]);
  assert.deepEqual(scanBytes(Buffer.from(fakePrivateKey)), ["private-key"]);
});

test("allows documented placeholders and redacted values", () => {
  const safeExamples = Buffer.from(
    "token=<REDACTED>\npassword=${PASSWORD}\napi_key=placeholder",
  );
  assert.deepEqual(scanBytes(safeExamples), []);
});

test("finds a fake credential that remains only in reachable Git history", async () => {
  await withRepository(async (directory) => {
    const fixture = join(directory, "fixture.txt");
    await writeFile(fixture, ["glpat-", "B".repeat(24)].join(""));
    git(directory, "add", "fixture.txt");
    git(directory, "commit", "--quiet", "-m", "add fixture");
    await rm(fixture);
    git(directory, "add", "fixture.txt");
    git(directory, "commit", "--quiet", "-m", "remove fixture");

    const result = await scanRepository({ cwd: directory });
    assert.equal(
      result.findings.some(
        ({ source, rule }) => source === "history" && rule === "gitlab-token",
      ),
      true,
    );
    assert.equal(
      result.findings.some(({ source }) => source === "working-tree"),
      false,
    );
  });
});

test("fails closed when a working-tree file exceeds its configured bound", async () => {
  await withRepository(async (directory) => {
    await writeFile(join(directory, "oversized.txt"), "x".repeat(33));
    await assert.rejects(
      scanRepository({ cwd: directory, limits: { maxFileBytes: 32 } }),
      (error) =>
        error instanceof SecretScanError && error.code === "file_limit",
    );
  });
});
