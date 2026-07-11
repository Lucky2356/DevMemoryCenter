# Contributing

Read `AGENTS.md`, `SPEC.md`, `TODO.md`, and `PROJECT_STATE.md` before changing the repository. Work on one small, testable task from the current phase; preserve unrelated changes and never use destructive Git cleanup.

Production code uses Safe Rust and strict TypeScript. Do not add generic shell/filesystem/SQL IPC, remote content, telemetry, AI, cloud infrastructure, or elevated privileges. Treat all external and stored values as untrusted.

Before a commit, run the configured format, lint, type, test, build, and security checks; inspect `git diff`; update `TODO.md` and `PROJECT_STATE.md`; and document any unavailable check and residual risk.
