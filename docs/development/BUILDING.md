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
