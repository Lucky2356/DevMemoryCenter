# Contributing

Read `AGENTS.md`, `SPEC.md`, `TODO.md`, and `PROJECT_STATE.md` before changing the repository. Work on one small, testable task from the current phase; preserve unrelated changes and never use destructive Git cleanup.

Production code uses Safe Rust and strict TypeScript. Do not add generic shell/filesystem/SQL IPC, remote content, telemetry, AI, cloud infrastructure, or elevated privileges. Treat all external and stored values as untrusted.

Before a commit, run the configured format, lint, type, test, build, and security checks; inspect `git diff`; update `TODO.md` and `PROJECT_STATE.md`; and document any unavailable check and residual risk. Run `npm run security:secrets` before every push; it checks current repository files and all reachable Git blobs without printing matched values or paths. Do not bypass a finding or add a broad allowlist.

Pull requests should contain one logical change, use synthetic test data, and complete the repository checklist. Report suspected vulnerabilities through GitHub's private security-advisory flow rather than a public issue.
