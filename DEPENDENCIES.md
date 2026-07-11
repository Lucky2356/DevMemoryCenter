# Dependency Policy and Initial Review

## Policy

Prefer the standard library and a small dependency graph. Every production dependency requires review of necessity, maintenance, stable release history, known advisories, license, transitive graph, install scripts, platform support, binary/runtime cost, and permissions. Lockfiles are committed. Git branch dependencies and remote runtime code are prohibited.

## Initial application dependencies

The application uses Tauri `2.11.5`, tauri-build `2.6.3`, repository-local Tauri CLI `2.11.4`, Tauri JavaScript API `2.11.1`, Serde `1.0.228`, React/React DOM `19.2.7`, TypeScript `7.0.2`, Vite `8.1.4`, Vitest `4.1.10`, ESLint `10.7.0`, typescript-eslint `8.63.0`, and Prettier `3.9.5`. Exact transitive versions are recorded by `Cargo.lock` and `package-lock.json`.

`@tauri-apps/api` is required only for the typed frontend invocation boundary. Version `2.11.1` is maintained in the official Tauri repository, has no runtime dependencies or install lifecycle script, is dual MIT/Apache-2.0 licensed, and is approximately 699 kB unpacked according to the npm registry metadata checked on 2026-07-11. Only its core `invoke` entry point is imported; no plugin or additional capability is enabled.

Serde is required for explicit command request, response, and error serialization. The derive feature is the only enabled direct feature. `serde_json 1.0.150` is a test-only dependency used to verify the exact sanitized wire error and unknown-field rejection. Both were already present transitively through Tauri, so making them direct dependencies does not introduce a new package family.

SQLite, redaction, UUID, logging, localization, and other feature dependencies are deliberately deferred until the task that needs them includes a focused comparison.

`npm audit` reported zero known vulnerabilities on 2026-07-11. Rust advisory and license-policy scans remain unverified because `cargo-audit` and `cargo-deny` are not installed; no automatic installation was performed.

## Required checks

- `npm audit` for the npm lockfile.
- `cargo audit` when the tool is available.
- `cargo deny check` when policy and the tool are available.
- License and duplicate-dependency review before release.
