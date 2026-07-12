# Project State

## Current phase

Phase 2 — Local projects. Phase 1 application foundation is complete.

## Last completed task

Added the framework-independent `Project` domain entity and validation tests.

## Work in progress

None. The next Phase 2 task is platform path validation and canonicalization.

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
- Initially preserved the Rust 1.85 MSRV while deferring SQLx 0.9.0, which declares Rust 1.94, and recorded rusqlite as the fallback if implementation gates fail.
- Added `dev-recall-persistence` with SQLx 0.8.6, a maximum of four connections, bounded worker buffers and statement cache, acquisition/busy/idle/lifetime timeouts, foreign keys, full synchronous mode, rollback journal mode, and explicit close.
- Added canonical application-data directory handling, an internally fixed database filename, non-file/symlink rejection, a 4,096-unit path limit, and fail-closed Unix permission validation with `0600` creation.
- Added embedded checksum-validated migration `0001_initial.sql` with a strict `owners` table, stable-ID/timestamp constraints, and a single-local-owner index.
- Added ADR-0011 and removed SQLx macros after the resolved dependency graph showed unrelated database packages; the active graph is SQLite-only and has no database TLS or extension loading.
- Installed user-scoped Rust 1.85.0 and verified the full workspace against its declared MSRV.
- Added deterministic migration tests covering an empty database, a representative database with 128 synthetic owner records, repeat migration execution, preserved sentinel records, and rejection of a changed applied-migration checksum.
- Documented that previous-version upgrade coverage becomes mandatory with migration version 2; no previous schema version exists while `0001_initial.sql` is the sole migration.
- Installed user-scoped `cargo-audit 0.22.2`, `cargo-deny 0.20.2`, and Rust 1.88.0 without system-wide changes.
- Added `.cargo/audit.toml` and `deny.toml` with Windows/Linux target policy, approved licenses, crates.io-only sources, duplicate warnings, and narrow documented advisory exceptions.
- Updated `plist 1.8.0` to `1.10.0`, `quick-xml 0.38.4` to `0.41.0`, and `time 0.3.45` to `0.3.53`, resolving two High and one Medium RustSec advisories.
- Added ADR-0012 for the security-driven Rust 1.88 MSRV and a dedicated CI job that installs locked security-tool versions and runs both Rust dependency gates.
- Added `dev-recall-observability` with a closed JSON-lines schema: fixed levels, components, event codes, numeric operation IDs, and at most eight numeric context metrics.
- Added synchronous mutex-serialized writes with no background task, a 1 MiB active file, five archives, seven-day retention, bounded custom limits, oversized/excess archive cleanup, and explicit clearing.
- Added fixed application-data log paths, non-file/symlink rejection, `0700`/`0600` Unix permissions, sanitized typed errors, and tests for concurrent writers, rotation, retention, bounds, clearing, and unsafe targets.
- Added ADR-0013 and connected logger ownership to the Tauri composition root without emitting pre-consent events.
- Added a dependency-free Node.js repository scanner for private keys, common platform tokens, JWT-like values, Basic Auth URLs, and likely assigned secrets.
- Added fail-closed limits for file/blob size, file/blob count, total bytes, and Git enumeration output; working-tree symlinks are scanned as link text without following them.
- Added regression tests for synthetic credentials, safe placeholders, deleted-but-reachable history, oversized files, and Windows path normalization.
- Added a minimal-permission full-history CI job plus CODEOWNERS, structured non-sensitive issue forms, a security-focused pull-request checklist, and controlled weekly Dependabot configuration.
- Corrected expired/oversized/excess archive test fixtures to create Unix files with `0600`, matching the production logger's fail-closed permission policy without weakening it.
- Published `main` to the owner repository with a production-oriented description, scoped topics, issue/project support, squash-only merges, branch deletion after merge, security/dependency alerts, CODEOWNERS, labels, issue forms, and pull-request guidance.
- Added `dev-recall-application`, a standard-library-only lifecycle coordinator with a default four-operation limit, nonzero unique operation IDs, scoped atomic cancellation, condition-variable cleanup notification, and permanent admission closure during shutdown.
- Connected the lifecycle manager to Tauri application state and invoked cancellation plus a bounded five-second wait on `ExitRequested`; no production background operation is started yet.
- Added `dev-recall-domain` with distinct validated UUID-compatible project/owner IDs, bounded and normalized display/description text, a closed project-type enum, ordered timestamps, explicit archive state, and source consent disabled by default.
- Added non-empty root/canonical path placeholders without claiming platform path safety; platform length, traversal, special-path, canonicalization, and symlink enforcement remain the next isolated task.

## Tests passed

- `npm run test`: 4 repository-security tests and 30 localization, application-shell, IPC validation, and error-sanitization tests passed.
- `cargo test --workspace --all-features`: 41 Rust unit tests passed; doc tests passed.
- Focused application lifecycle tests: 5 tests passed for invalid limits/IDs, duplicate and capacity rejection, scoped cancellation, cooperative shutdown, bounded timeout/retry, and manager-drop cancellation.
- Focused `Project` domain tests: 11 tests passed for valid construction, UUID shape, UTF-8 byte limits, control/bidi rejection, safe plain-text preservation, description normalization, path placeholders, timestamp/archive ordering/range, and privacy defaults.
- GitHub Actions run `29167499295` passed the full-history repository-security job, dependency audits, Ubuntu 24.04 quality/tests/Tauri build, and Windows 2025 quality/tests/Tauri build.
- `npm run format:check`, `npm run lint`, `npm run typecheck`, and `npm run build` passed.
- `cargo fmt --all -- --check`, strict workspace Clippy, and `cargo check --workspace --all-targets` passed.
- `npm run tauri -- info` and the Windows Tauri debug build passed.
- `cargo +1.88.0 check --workspace --all-targets` passed against the new declared MSRV.
- `cargo audit` passed with one committed exception for an inactive SQLx MySQL/RSA lockfile package.
- `cargo deny check` passed advisory, license, ban, and source policy for the Windows/Linux all-feature graph; duplicate versions remain warnings.

## Checks not run

- Representative packaged permission behavior still requires installer-level Windows/Linux release testing; current hosted debug builds are not installer verification.
- Hosted Windows/Linux CI was not run for this local commit because the current autonomous-run instruction forbids push; local Windows checks and debug build passed.

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
- Automated RustSec scanning found and the lockfile update removed RUSTSEC-2026-0194, RUSTSEC-2026-0195, and RUSTSEC-2026-0009.
- `cargo tree --workspace --all-features --target all -i rsa` confirmed that the RSA advisory recorded by SQLx's disabled optional MySQL backend has no active dependency path.
- Cargo-deny verified only crates.io sources, the reviewed license set, no active vulnerability advisory, and no new unsound advisory beyond the documented Tauri Linux GLib exception.
- Observability regression tests verify fixed-schema valid JSON, no arbitrary string context surface, bounded metrics/files/archives, serialized concurrent writes, startup retention, oversized/excess cleanup, explicit clearing, and unsafe path rejection.
- The desktop composition root emits no log event before onboarding consent; current startup creates only an empty local log file and performs bounded retention maintenance.
- The observability crate adds no production dependency, network access, shell/process capability, background thread, unbounded queue, or raw error logging.
- Lifecycle regression tests verify bounded admission, permanent shutdown admission closure, cancellation of every active permit, condition-variable cleanup waiting, timeout reporting, and drop-time cancellation without production task spawning.
- Project validation rejects nil/non-RFC/versionless IDs, oversized text, control/bidi characters, invalid timestamp/archive ordering, and empty path placeholders; privacy consent is disabled by default.
- `npm run security:secrets` passed over the current repository and every blob reachable from local refs without printing candidate values or paths.
- Secret-scanner regression tests verify historical detection after working-tree deletion and fail-closed handling when configured limits are exceeded.
- CI uses a full-history checkout only for the isolated repository-security job, keeps `contents: read`, and persists no checkout credentials.

## Performance measurements

- Frontend production build completed in approximately 0.2 seconds; output was 199.81 kB JavaScript (63.49 kB gzip) and 4.89 kB CSS (1.57 kB gzip).
- Persistence tests including empty migration, 128-record preservation, checksum rejection, constrained writes, explicit close, and reopen completed in approximately 0.08 seconds on the focused Windows run; this is a regression-test observation, not a production benchmark.
- The pool permits at most four SQLite worker connections, keeps zero minimum idle connections, and bounds command buffers at 32, row buffers at 128, and statement caches at 64 per connection.
- The persistence crate is not linked into the desktop package, so it adds no current desktop runtime threads or binary payload. Re-measure when the composition root begins owning a database.
- Focused observability tests completed in approximately 0.04 seconds on Windows. The default log budget is approximately 6 MiB across one active file and five archives, with no idle polling or background worker.
- The bounded repository and reachable-history scan completed locally in approximately 0.8 seconds for the current repository after switching historical reads to one validated Git batch operation.
- Focused lifecycle tests completed in approximately 0.03 seconds on Windows. The coordinator has no worker thread, queue, polling loop, network access, disk I/O, or retained payload collection.
- Focused domain tests completed in approximately 0.00 seconds on Windows. The crate adds no dependency, I/O, background work, network access, global state, or unbounded collection.

## Known issues

- Global `cargo-tauri` is not installed; the verified repository-local npm CLI is used.
- GitHub branch protection and required-check enforcement are unavailable for this private personal repository without GitHub Pro or changing visibility to public; neither paid service nor visibility change was authorized.
- The application currently has only the foundation status screen; all MVP features remain planned.
- The initial hosted Linux run exposed two synthetic archive fixtures inheriting `0644`; the production permission rejection worked as designed, and the corrected follow-up run is green.
- Locale selection currently follows system/browser preferences; a user-selected persisted override is not implemented yet.
- Navigation is in-memory only and intentionally does not preserve routes across restart or expose unfinished feature actions.
- Theme selection is in-memory only and intentionally resets to the system preference until an approved local settings store exists.
- The health adapter is not yet called by a screen; it establishes and tests the first contract without fabricating runtime health UI behavior.
- The persistence crate is not initialized by the desktop application; no user database, backup, recovery, encryption, or product repository behavior exists yet.
- SQLx 0.8.6 bundles SQLite 3.46.0; later upstream security fixes require a driver/native-engine upgrade before FTS5 or release.
- Tauri's Linux backend retains unmaintained GTK3 bindings and a documented GLib iterator unsoundness exception; Dev Recall does not call the affected API, but Linux release readiness remains blocked on review/upstream migration.
- SQLx's disabled optional MySQL backend records an RSA advisory in `Cargo.lock`; the backend is absent from the active graph and the exact audit exception must not be broadened.
- Logger consent, configurable retention, and clearing are not yet exposed in Privacy & Data UI; until consent state exists, the runtime intentionally emits no events.
- Windows log permissions inherit the application-data ACL and still require representative packaged permission verification; Unix creation and fail-closed permission checks are covered by code/tests.
- Repository secret detection is heuristic and cannot prove absence of unknown, encoded, fragmented, or unrecognized credential formats; GitHub push protection is configured separately when supported.
- Scanner limits intentionally fail closed as history grows and must be reviewed rather than bypassed if the repository approaches them.
- The lifecycle coordinator intentionally does not own a concrete async executor yet; every future long-running feature must retain and join its task handles, add progress/checkpoint semantics where required, and test forced shutdown at that feature boundary.
- `ProjectPaths` currently enforces only non-empty placeholders; no filesystem access may use them until the next platform-validation task adds absolute-path, length, canonicalization, traversal, special-path, and symlink enforcement.

## Security findings

`SECURITY_FINDINGS.md` records the resolved High XML/time parser finding and three residual Medium findings: bundled SQLite age, inactive SQLx-MySQL/RSA lockfile metadata, and the Tauri Linux GTK3/GLib lifecycle. No open Critical or High finding remains.

## Decisions required

- Repository license selection.

The remaining decision is documented in `NEEDS_USER_INPUT.md`. Owner-authorized repository publication may proceed under the temporary all-rights-reserved notice, but external contributions and releases remain blocked.

## Next task

Implement platform path validation, canonicalization, special-path, traversal, and symlink tests.

## Last stable commit

`b3df1e6` (`feat: add bounded background operation lifecycle`) is the stable baseline preceding this task.

## Commands to verify

```text
npm ci
npm run format:check
npm run lint
npm run typecheck
npm run test
npm run build
npm audit --audit-level=high
npm run security:secrets
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo check --workspace --all-targets
cargo +1.88.0 check --workspace --all-targets
cargo audit
cargo deny check
npm run tauri -- info
npm run tauri -- build --config apps/desktop/src-tauri/tauri.conf.json --debug
```
