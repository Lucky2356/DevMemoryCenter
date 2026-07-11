# Project State

## Current phase

Phase 1 — Application foundation.

## Last completed task

Compared maintained Rust SQLite drivers and selected a minimal SQLx 0.8 dependency profile in ADR-0010.

## Work in progress

None. The driver selection is documented; SQLx and persistence code have not been added.

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
- Moved frontend shell strings into typed English/Russian resource modules.
- Added ordered locale resolution, document language/title application, and `Intl` number/date/duration formatting with plural rules and invalid-duration rejection.
- Added embedded English/Russian Rust resources for safe startup failures without exposing internal Tauri errors.
- Added the eight `SPEC.md` navigation sections using bounded route identifiers and native keyboard controls.
- Added a skip link, visible focus, semantic landmarks, current-page state, polite content announcements, and responsive small-window layout.
- Added localized screen descriptions and honest shared loading, empty, normal, error, and disabled state semantics.
- Added a typed three-value theme preference with system as the default and explicit light/dark overrides.
- Added localized native radio controls, semantic palette tokens, native-control color schemes, and reduced-motion transition suppression.
- Added `get_application_health` with a 1–64 byte ASCII correlation ID, unknown-field rejection, fixed protocol response, and no host or user-data inspection.
- Added a TypeScript IPC adapter that reconstructs bounded payloads, validates exact responses, sanitizes arbitrary rejections, and returns discriminated results.
- Added localized English/Russian messages for the fixed `invalid_request`, `invalid_response`, and `operation_failed` error keys.
- Compared SQLx 0.8.6, rusqlite 0.40.1, and Diesel 2.3.11 against migration, concurrency, scope, native packaging, license, security-history, and project-fit criteria.
- Added ADR-0010 selecting SQLx 0.8.6 with defaults disabled and only `macros`, `migrate`, `runtime-tokio`, and bundled `sqlite` features.
- Preserved the Rust 1.85 MSRV by deferring SQLx 0.9.0, which declares Rust 1.94, and recorded rusqlite as the fallback if implementation gates fail.

## Tests passed

- `npm run test`: 30 localization, application-shell, IPC validation, and error-sanitization tests passed.
- `cargo test --workspace --all-features`: 10 Rust unit tests passed; doc tests passed.
- `npm run format:check`, `npm run lint`, `npm run typecheck`, and `npm run build` passed.
- `cargo fmt --all -- --check`, strict workspace Clippy, and `cargo check --workspace --all-targets` passed.
- `npm run tauri -- info` and the Windows Tauri debug build passed.

## Checks not run

- `cargo audit`: not run because `cargo-audit` is not installed. Install deliberately, then run `cargo audit`; residual risk is an unverified Rust advisory database scan.
- `cargo deny check`: not run because `cargo-deny` is not installed and policy configuration is a Phase 1 task. Residual risk is unverified Rust license/advisory/duplicate policy.
- Linux build and Tauri prerequisites: not run from the Windows host. Verify in Phase 1 CI or a representative Linux environment.
- The GitHub-hosted workflow itself has not run because no push was authorized; workflow success on both runner images remains externally unverified.
- Rust 1.85 compatibility of SQLx 0.8.6 was not run because only the current stable 1.96 toolchain is installed and the dependency is not yet part of the workspace. The next task must verify the resolved dependency before commit; the residual risk is that the selected 0.8 line or a transitive dependency may require a higher compiler.

## Security checks passed

- Manual threat/privacy review completed and security invariants documented.
- `npm audit --audit-level=high`: zero known vulnerabilities.
- Source review found no `unsafe`, shell plugin, generic IPC command, remote content, user-data persistence, or unsafe React HTML in the skeleton.
- CI workflow review confirmed top-level `contents: read`, non-persisted checkout credentials, full 40-character action SHAs, no cache, no secrets, no artifact upload, and no publish/release step.
- Localization review confirmed no remote locale loading, no persistence or transmission of language preferences, no unsafe HTML fallback, and no internal Tauri error exposure.
- UI review confirmed bounded navigation state, text-only rendering, no IPC/network/storage behavior, and accessible loading/error/disabled semantics.
- Theme review confirmed bounded state, no storage/network/IPC behavior, native keyboard controls, and reduced-motion handling for presentation transitions.
- IPC regression tests confirmed oversized/unsafe IDs and unknown fields are rejected, malformed responses fail closed, arbitrary error text is discarded, and rejected input is not serialized.
- Dependency review confirmed the official Tauri API package has no runtime dependencies or install script; `npm audit --audit-level=high` found zero known vulnerabilities.
- Driver review confirmed SQLx 0.8.6 is above the `>=0.8.1` patched boundary for RUSTSEC-2024-0363, excludes remote database/TLS features, and does not authorize extension loading or raw-handle access. Final transitive results remain pending until the dependency enters `Cargo.lock`.

## Performance measurements

- Frontend production build completed in approximately 0.2 seconds; output was 199.81 kB JavaScript (63.49 kB gzip) and 4.89 kB CSS (1.57 kB gzip).
- This documentation-only task added no runtime code or dependency, so no new runtime measurement was applicable.
- SQLx creates one SQLite worker thread per connection; ADR-0010 therefore requires a deliberately small pool, bounded worker buffers, and binary/idle measurements when the dependency is added.

## Known issues

- Global `cargo-tauri` is not installed; the verified repository-local npm CLI is used.
- Linux prerequisites and builds remain unverified from this Windows host.
- `cargo-audit` and `cargo-deny` are unavailable and were not installed automatically.
- The application currently has only the foundation status screen; all MVP features remain planned.
- CI remains unverified on GitHub until an explicitly authorized push triggers it.
- Locale selection currently follows system/browser preferences; a user-selected persisted override is not implemented yet.
- Navigation is in-memory only and intentionally does not preserve routes across restart or expose unfinished feature actions.
- Theme selection is in-memory only and intentionally resets to the system preference until an approved local settings store exists.
- The health adapter is not yet called by a screen; it establishes and tests the first contract without fabricating runtime health UI behavior.
- SQLx is selected but not installed; Rust 1.85 compatibility, resolved transitive licenses/advisories, bundled SQLite version, binary delta, and Windows/Linux packaging remain implementation gates.

## Security findings

No concrete vulnerability is known in the current repository.

## Decisions required

- Repository license selection.
- Clarify whether the GitHub publication restriction permits the `FUTURE_AI.md` product-boundary document required by `SPEC.md`.

Both decisions are documented in `NEEDS_USER_INPUT.md` and do not block local development. No push or publication is authorized.

## Next task

Add the bounded SQLite connection setup and immutable initial migration.

## Last stable commit

`HEAD` after the SQLite driver selection commit (`docs: select SQLx SQLite driver`).

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
