# Project State

## Current phase

Phase 1 — Application foundation.

## Last completed task

Added Windows/Linux continuous integration with minimal repository permissions, full-SHA-pinned actions, locked dependency installation, frontend/Rust checks, npm audit, and Tauri debug builds.

## Work in progress

None. The CI task is complete; the next Phase 1 task has not started.

## Completed

- Fully read `AGENTS.md` and `SPEC.md`.
- Inspected the initial two-file directory and confirmed it was not a Git repository.
- Verified local Windows toolchain prerequisites.
- Defined exact MVP boundaries and incremental phases.
- Added nine ADRs, including encryption/key storage.
- Added a localized React/Vite shell with strict TypeScript and CSS Modules.
- Added a Safe Rust Tauri composition root with no application IPC commands, no plugins, empty capabilities, strict CSP, no collection, and no persistence.
- Generated local placeholder application icons required by the Windows resource build.
- Added `.github/workflows/ci.yml` for fixed Ubuntu 24.04 and Windows 2025 runners with no publishing or artifact upload.
- Added a visible CI status badge and documented CI security/update policy.

## Tests passed

- `npm run test`: 3 localization tests passed.
- `cargo test --workspace --all-features`: 1 Rust unit test passed; doc tests passed.
- `npm ci --ignore-scripts`: locked dependency installation completed without lifecycle scripts.

## Checks not run

- `cargo audit`: not run because `cargo-audit` is not installed. Install deliberately, then run `cargo audit`; residual risk is an unverified Rust advisory database scan.
- `cargo deny check`: not run because `cargo-deny` is not installed and policy configuration is a Phase 1 task. Residual risk is unverified Rust license/advisory/duplicate policy.
- Linux build and Tauri prerequisites: not run from the Windows host. Verify in Phase 1 CI or a representative Linux environment.
- The GitHub-hosted workflow itself has not run because no push was authorized; workflow success on both runner images remains externally unverified.

## Security checks passed

- Manual threat/privacy review completed and security invariants documented.
- `npm audit --audit-level=high`: zero known vulnerabilities.
- Source review found no `unsafe`, shell plugin, generic IPC command, remote content, user-data persistence, or unsafe React HTML in the skeleton.
- CI workflow review confirmed top-level `contents: read`, non-persisted checkout credentials, full 40-character action SHAs, no cache, no secrets, no artifact upload, and no publish/release step.

## Performance measurements

- Frontend production build completed in approximately 0.2 seconds after dependency installation; output was 192.04 kB JavaScript (60.83 kB gzip) and 1.18 kB CSS (0.59 kB gzip).
- No runtime/idle measurement was taken; the skeleton is not an MVP and initial acceptance budgets remain in `docs/performance/PERFORMANCE_BUDGETS.md`.

## Known issues

- Global `cargo-tauri` is not installed; the verified repository-local npm CLI is used.
- Linux prerequisites and builds remain unverified from this Windows host.
- `cargo-audit` and `cargo-deny` are unavailable and were not installed automatically.
- The application currently has only the foundation status screen; all MVP features remain planned.
- CI remains unverified on GitHub until an explicitly authorized push triggers it.

## Security findings

No concrete vulnerability found in the initial documentation-only repository.

## Decisions required

- Repository license selection.
- Clarify whether the GitHub publication restriction permits the `FUTURE_AI.md` product-boundary document required by `SPEC.md`.

Both decisions are documented in `NEEDS_USER_INPUT.md` and do not block local development. No push or publication is authorized.

## Next task

Complete the existing English/Russian localization foundation and ensure every application-shell string, including the document title and startup failure message, is supplied through an appropriate locale-aware boundary.

## Last stable commit

`HEAD` after the CI commit (`ci: add Windows and Linux quality gates`).

## Commands to verify

```text
npm ci
npm run format:check
npm run lint
npm run typecheck
npm run test
npm run build
npm audit --audit-level=high
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo check --workspace --all-targets
npm run tauri -- info
npm run tauri -- build --config apps/desktop/src-tauri/tauri.conf.json --debug
```
