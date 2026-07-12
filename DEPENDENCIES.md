# Dependency Policy and Initial Review

## Policy

Prefer the standard library and a small dependency graph. Every production dependency requires review of necessity, maintenance, stable release history, known advisories, license, transitive graph, install scripts, platform support, binary/runtime cost, and permissions. Lockfiles are committed. Git branch dependencies and remote runtime code are prohibited.

## Initial application dependencies

The application uses Tauri `2.11.5`, tauri-build `2.6.3`, repository-local Tauri CLI `2.11.4`, Tauri JavaScript API `2.11.1`, Serde `1.0.228`, React/React DOM `19.2.7`, TypeScript `7.0.2`, Vite `8.1.4`, Vitest `4.1.10`, ESLint `10.7.0`, typescript-eslint `8.63.0`, and Prettier `3.9.5`. Exact transitive versions are recorded by `Cargo.lock` and `package-lock.json`.

`@tauri-apps/api` is required only for the typed frontend invocation boundary. Version `2.11.1` is maintained in the official Tauri repository, has no runtime dependencies or install lifecycle script, is dual MIT/Apache-2.0 licensed, and is approximately 699 kB unpacked according to the npm registry metadata checked on 2026-07-11. Only its core `invoke` entry point is imported; no plugin or additional capability is enabled.

Serde is required for explicit command request, response, and error serialization. The derive feature is the only enabled direct feature. `serde_json 1.0.150` is a test-only dependency used to verify the exact sanitized wire error and unknown-field rejection. Both were already present transitively through Tauri, so making them direct dependencies does not introduce a new package family.

ADR-0010 selected SQLx 0.8 after comparing SQLx, rusqlite, and Diesel. The persistence crate now uses SQLx `0.8.6` with default features disabled and only `migrate`, `runtime-tokio`, and `sqlite`. ADR-0011 supersedes the earlier `macros` feature choice: a maintained `MigrationSource` implementation embeds the SQL files without retaining SQLx's unrelated database macro packages. The normal dependency graph contains `sqlx-core`, `sqlx-sqlite`, Tokio/futures support, and `libsqlite3-sys`; it contains no MySQL, PostgreSQL, `any`, TLS, JSON integration, regular-expression functions, hooks, or extension loading.

SQLx 0.9.0 remains deferred because its declared Rust requirement is 1.94 while the workspace declares Rust 1.88. ADR-0012 raised the MSRV from 1.85 so Tauri's plist dependency could move to patched parser releases. The complete workspace, including SQLx 0.8.6, must pass `cargo +1.88.0 check --workspace --all-targets`. The SQLx CLI is not a project dependency.

The first automated RustSec scan found RUSTSEC-2026-0194 and RUSTSEC-2026-0195 in `quick-xml 0.38.4` and RUSTSEC-2026-0009 in `time 0.3.45`. The lockfile now uses `plist 1.10.0`, `quick-xml 0.41.0`, and `time 0.3.53`; the High and Medium parser advisories are resolved. `SECURITY_FINDINGS.md` records the remediation and residual transitive advisories.

The resolved `libsqlite3-sys 0.30.1` bundles SQLite 3.46.0. SQLx 0.8.6 is above the fixed boundary for RUSTSEC-2024-0363, and libsqlite3-sys is above the fixed boundary for RUSTSEC-2022-0090. SQLite's official vulnerability list contains later engine fixes; `SECURITY_FINDINGS.md` records the resulting restriction against enabling FTS5 or releasing with this engine. This manual review does not substitute for `cargo audit` over the final lockfile.

Redaction, UUID, and other feature dependencies remain deferred until the task that needs them includes a focused comparison. The logging foundation uses only the Rust standard library: its closed event vocabulary, synchronous bounded writer, rotation, retention, and permissions do not justify `tracing` or a background appender yet. `serde_json` is test-only and was already present in the lockfile; it validates emitted JSON without entering production code. Repository secret scanning uses the Node.js standard library and Git already required for development; it adds no package, action, install script, or network service.

The application lifecycle coordinator also uses only the Rust standard library. Mutex/condition-variable coordination and atomic cancellation flags are sufficient for bounded admission and shutdown; it does not justify an async runtime, cancellation package, channel package, or task-executor dependency. Future asynchronous features must separately review their executor and retain owned task handles.

`cargo-audit 0.22.2` and `cargo-deny 0.20.2` were installed in the user-scoped Cargo toolchain with their published lockfiles. The repository stores audit and deny policy, and CI installs those exact tool versions. `cargo audit` passes with one explicit exception for RUSTSEC-2023-0071: the affected RSA crate exists only in SQLx's disabled optional MySQL backend and is absent from the active all-feature workspace graph. `cargo deny check` validates advisories, approved SPDX licenses, duplicate-version warnings, and crates.io-only sources for Windows and Linux.

The current Tauri Linux backend retains GTK3 lifecycle warnings and RUSTSEC-2024-0429 in a GLib iterator API that Dev Recall does not call. Cargo-deny denies new unsound advisories and contains one reasoned exception for that transitive advisory; cargo-audit keeps the informational warnings visible. These exceptions do not authorize direct MySQL, RSA, GTK3, or GLib use.

`npm audit` reported zero known vulnerabilities on 2026-07-12. Frontend audit already runs in CI. Repository secret scanning remains a separate Phase 1 task.

## Required checks

- `npm audit` for the npm lockfile.
- `cargo audit` with the committed `.cargo/audit.toml` policy.
- `cargo deny check` with the committed `deny.toml` policy.
- License and duplicate-dependency review before release.
