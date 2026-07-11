# Dependency Policy and Initial Review

## Policy

Prefer the standard library and a small dependency graph. Every production dependency requires review of necessity, maintenance, stable release history, known advisories, license, transitive graph, install scripts, platform support, binary/runtime cost, and permissions. Lockfiles are committed. Git branch dependencies and remote runtime code are prohibited.

## Initial application dependencies

The application uses Tauri `2.11.5`, tauri-build `2.6.3`, repository-local Tauri CLI `2.11.4`, Tauri JavaScript API `2.11.1`, Serde `1.0.228`, React/React DOM `19.2.7`, TypeScript `7.0.2`, Vite `8.1.4`, Vitest `4.1.10`, ESLint `10.7.0`, typescript-eslint `8.63.0`, and Prettier `3.9.5`. Exact transitive versions are recorded by `Cargo.lock` and `package-lock.json`.

`@tauri-apps/api` is required only for the typed frontend invocation boundary. Version `2.11.1` is maintained in the official Tauri repository, has no runtime dependencies or install lifecycle script, is dual MIT/Apache-2.0 licensed, and is approximately 699 kB unpacked according to the npm registry metadata checked on 2026-07-11. Only its core `invoke` entry point is imported; no plugin or additional capability is enabled.

Serde is required for explicit command request, response, and error serialization. The derive feature is the only enabled direct feature. `serde_json 1.0.150` is a test-only dependency used to verify the exact sanitized wire error and unknown-field rejection. Both were already present transitively through Tauri, so making them direct dependencies does not introduce a new package family.

ADR-0010 selects SQLx 0.8 for the planned SQLite persistence adapter after comparing SQLx, rusqlite, and Diesel. SQLx is not yet present in `Cargo.toml` or `Cargo.lock`. The next persistence task may add version `0.8.6` with default features disabled and only `macros`, `migrate`, `runtime-tokio`, and `sqlite`. This keeps other database drivers, `any`, TLS, JSON integration, regular-expression functions, hooks, and extension loading out of the graph. The `sqlite` feature statically bundles SQLite for a deterministic Windows/Linux baseline.

SQLx 0.9.0 is deliberately deferred because its declared Rust requirement is 1.94 while the workspace declares Rust 1.85. SQLx 0.8.6 does not declare an MSRV in crates.io metadata, so the dependency-addition task must verify Rust 1.85 compatibility before commit. It must also inspect the resolved transitive graph and licenses, run available advisory/policy checks, record the bundled SQLite version, measure the release binary delta, and verify the packaged Windows build. Linux packaging remains a CI or representative-host gate. The SQLx CLI is not selected as a project dependency.

The documented review used official SQLx, rusqlite, Diesel, crates.io/docs.rs, and RustSec sources on 2026-07-11. SQLx 0.8.6 is above the fixed boundary for its recorded RUSTSEC-2024-0363 advisory. This does not substitute for auditing the final lockfile.

Redaction, UUID, logging, localization, and other feature dependencies remain deferred until the task that needs them includes a focused comparison.

`npm audit` reported zero known vulnerabilities on 2026-07-11. Rust advisory and license-policy scans remain unverified because `cargo-audit` and `cargo-deny` are not installed; no automatic installation was performed.

## Required checks

- `npm audit` for the npm lockfile.
- `cargo audit` when the tool is available.
- `cargo deny check` when policy and the tool are available.
- License and duplicate-dependency review before release.
