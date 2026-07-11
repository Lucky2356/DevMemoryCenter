# Project State

## Current phase

Phase 1 — Application foundation.

## Last completed task

Added migration compatibility coverage for the empty and representative populated database states.

## Work in progress

None. The migration tests are complete; persistence remains intentionally uninitialized by the desktop application.

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
- Added `dev-recall-persistence` with SQLx 0.8.6, a maximum of four connections, bounded worker buffers and statement cache, acquisition/busy/idle/lifetime timeouts, foreign keys, full synchronous mode, rollback journal mode, and explicit close.
- Added canonical application-data directory handling, an internally fixed database filename, non-file/symlink rejection, a 4,096-unit path limit, and fail-closed Unix permission validation with `0600` creation.
- Added embedded checksum-validated migration `0001_initial.sql` with a strict `owners` table, stable-ID/timestamp constraints, and a single-local-owner index.
- Added ADR-0011 and removed SQLx macros after the resolved dependency graph showed unrelated database packages; the active graph is SQLite-only and has no database TLS or extension loading.
- Installed user-scoped Rust 1.85.0 and verified the full workspace against its declared MSRV.
- Added deterministic migration tests covering an empty database, a representative database with 128 synthetic owner records, repeat migration execution, preserved sentinel records, and rejection of a changed applied-migration checksum.
- Documented that previous-version upgrade coverage becomes mandatory with migration version 2; no previous schema version exists while `0001_initial.sql` is the sole migration.

## Tests passed

- `npm run test`: 30 localization, application-shell, IPC validation, and error-sanitization tests passed.
- `cargo test --workspace --all-features`: 16 Rust unit tests passed; doc tests passed.
- `npm run format:check`, `npm run lint`, `npm run typecheck`, and `npm run build` passed.
- `cargo fmt --all -- --check`, strict workspace Clippy, and `cargo check --workspace --all-targets` passed.
- `npm run tauri -- info` and the Windows Tauri debug build passed.
- `cargo +1.85.0 check --workspace --all-targets` passed with the Rust-aware compatible dependency resolution.

## Checks not run

- `cargo audit`: attempted but unavailable because `cargo-audit` is not installed. Install deliberately, then run `cargo audit`; residual risk is an unverified Rust advisory database scan.
- `cargo deny check`: attempted but unavailable because `cargo-deny` is not installed and policy configuration is a Phase 1 task. Residual risk is unverified Rust license/advisory/duplicate policy.
- Linux build and Tauri prerequisites: not run from the Windows host. Verify in Phase 1 CI or a representative Linux environment.
- The GitHub-hosted workflow itself has not run because no push was authorized; workflow success on both runner images remains externally unverified.

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
- Driver review confirmed SQLx 0.8.6 is above the `>=0.8.1` patched boundary for RUSTSEC-2024-0363, excludes remote database/TLS features, and does not authorize extension loading or raw-handle access.
- Resolved-tree review confirmed only SQLx core/SQLite components, bundled `libsqlite3-sys 0.30.1`, and expected Tokio/futures support; MySQL, PostgreSQL, TLS, query macros, and extension-loading features are absent.
- Database regression tests verify relative/non-file path rejection, explicit pool bounds, foreign keys, WAL disabled, zero virtual tables, migration history, parameterized constraint enforcement, duplicate local-owner rejection, clean reopen, and explicit close.
- Migration compatibility regression tests verify that 128 bounded synthetic records survive a close/reopen migration cycle and that a changed checksum for migration version 1 fails closed with a version-mismatch error.
- Repository scan for common token, private-key, and assigned-secret patterns found no candidate sensitive data; migration fixtures contain only deterministic synthetic identifiers.
- Manual RustSec review confirmed SQLx 0.8.6 and libsqlite3-sys 0.30.1 exceed their recorded patched boundaries. The older bundled SQLite engine risk is recorded separately in `SECURITY_FINDINGS.md`.

## Performance measurements

- Frontend production build completed in approximately 0.2 seconds; output was 199.81 kB JavaScript (63.49 kB gzip) and 4.89 kB CSS (1.57 kB gzip).
- Persistence tests including empty migration, 128-record preservation, checksum rejection, constrained writes, explicit close, and reopen completed in approximately 0.08 seconds on the focused Windows run; this is a regression-test observation, not a production benchmark.
- The pool permits at most four SQLite worker connections, keeps zero minimum idle connections, and bounds command buffers at 32, row buffers at 128, and statement caches at 64 per connection.
- The persistence crate is not linked into the desktop package, so it adds no current desktop runtime threads or binary payload. Re-measure when the composition root begins owning a database.

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
- The persistence crate is not initialized by the desktop application; no user database, backup, recovery, encryption, or product repository behavior exists yet.
- SQLx 0.8.6 bundles SQLite 3.46.0; later upstream security fixes require a driver/native-engine upgrade before FTS5 or release.

## Security findings

One Medium finding is documented in `SECURITY_FINDINGS.md`: bundled SQLite 3.46.0 trails later upstream fixes. Current mitigations remove the known arbitrary-SQL/FTS preconditions; upgrade remains required before FTS5 and release.

## Decisions required

- Repository license selection.
- Clarify whether the GitHub publication restriction permits the `FUTURE_AI.md` product-boundary document required by `SPEC.md`.

Both decisions are documented in `NEEDS_USER_INPUT.md` and do not block local development. No push or publication is authorized.

## Next task

Add privacy-safe structured local logging with rotation and retention.

## Last stable commit

`0a34078` (`feat: add bounded SQLite foundation`) is the stable baseline preceding this work unit.

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
cargo +1.85.0 check --workspace --all-targets
npm run tauri -- info
npm run tauri -- build --config apps/desktop/src-tauri/tauri.conf.json --debug
```
