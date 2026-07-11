# Repository secret scanning

Dev Recall scans repository files and every blob reachable from local Git refs before publication and in CI:

```text
npm run security:secrets
```

The scanner uses the Node.js standard library and the installed Git executable. It performs no network requests, follows no working-tree symlinks, prints neither matched values nor paths, and exits non-zero on a candidate or an incomplete scan.

## Bounds

- 20,000 working-tree files;
- 50,000 reachable historical blobs;
- 1 MiB per file or blob;
- 64 MiB total working-tree content;
- 256 MiB total historical blob content;
- 16 MiB maximum output from Git enumeration commands.

Exceeding a limit fails closed instead of skipping content. The CI checkout uses full reachable history. Ignored build output and dependency caches are not repository content and are not scanned.

## Detection scope

The initial rules detect private-key headers, major GitHub/GitLab/AWS/Slack token forms, JWT-like values, credentials embedded in URLs, and likely assigned secrets. Tests construct obviously synthetic values at runtime so credential-shaped fixtures are not committed verbatim.

This scanner is a preventive gate, not proof that a repository contains no secret. Unknown formats, encoded values, split values, and high-entropy credentials without recognizable context can evade heuristics. GitHub push protection is an independent second layer when the hosting plan supports it.

## Handling a finding

Do not print or paste the suspected value. Identify the reported rule and, for history findings, the abbreviated object ID locally. Revoke any real credential first. Remove it from the current tree and reachable history using a separately reviewed recovery procedure; rewriting shared history requires owner coordination. Never add a broad allowlist to make CI pass. A narrowly justified synthetic fixture should be assembled at test runtime.
