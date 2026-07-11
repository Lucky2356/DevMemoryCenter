# Building

## Prerequisites

- Rust stable with Cargo and the platform target.
- Node.js and npm as selected by `package-lock.json`.
- Git.
- Windows: MSVC C++ build tools and Microsoft Edge WebView2 Runtime.
- Linux: the Tauri 2 system packages for the selected distribution, including WebKitGTK and related development libraries; exact commands must be verified on the target distribution rather than installed automatically.

`cargo-tauri` does not need a global installation; use the repository-local npm CLI.

## Commands

```text
npm ci
npm run format:check
npm run lint
npm run typecheck
npm run test
npm run build
npm run tauri -- build --config apps/desktop/src-tauri/tauri.conf.json --debug
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo check --workspace --all-targets
```

Security tools are separate prerequisites: `cargo audit` and `cargo deny check`.

## Continuous integration

`.github/workflows/ci.yml` runs the locked frontend and Rust checks plus a Tauri debug build on fixed `ubuntu-24.04` and `windows-2025` [GitHub-hosted runner images](https://docs.github.com/en/actions/how-tos/write-workflows/choose-where-workflows-run/choose-the-runner-for-a-job). The workflow has read-only repository permissions, persists no checkout credentials, uploads no artifacts, publishes nothing, and pins external actions to full commit SHAs.

The Linux job installs only the desktop development packages required by the [official Tauri prerequisites](https://v2.tauri.app/start/prerequisites/). Dependency caching is disabled to reduce shared supply-chain state. Action SHA updates require verification against the signed upstream releases for [checkout](https://github.com/actions/checkout/releases) and [setup-node](https://github.com/actions/setup-node/releases) before editing the workflow.
