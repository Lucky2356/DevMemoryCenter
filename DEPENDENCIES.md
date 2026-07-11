# Dependency Policy and Initial Review

## Policy

Prefer the standard library and a small dependency graph. Every production dependency requires review of necessity, maintenance, stable release history, known advisories, license, transitive graph, install scripts, platform support, binary/runtime cost, and permissions. Lockfiles are committed. Git branch dependencies and remote runtime code are prohibited.

## Initial application dependencies

The initial skeleton uses Tauri `2.11.5`, `tauri-build 2.6.3`, repository-local Tauri CLI `2.11.4`, React/React DOM `19.2.7`, TypeScript `7.0.2`, Vite `8.1.4`, Vitest `4.1.10`, ESLint `10.7.0`, typescript-eslint `8.63.0`, and Prettier `3.9.5`. Exact transitive versions are recorded by `Cargo.lock` and `package-lock.json`.

`@tauri-apps/api` is intentionally absent because the skeleton exposes and calls no IPC commands. It should be added only with the first narrow frontend/backend contract.

SQLite, redaction, UUID, logging, localization, and other feature dependencies are deliberately deferred until the task that needs them includes a focused comparison.

`npm audit` reported zero known vulnerabilities on 2026-07-11. Rust advisory and license-policy scans remain unverified because `cargo-audit` and `cargo-deny` are not installed; no automatic installation was performed.

## Required checks

- `npm audit` for the npm lockfile.
- `cargo audit` when the tool is available.
- `cargo deny check` when policy and the tool are available.
- License and duplicate-dependency review before release.
