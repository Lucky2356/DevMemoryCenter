# First-run Environment Verification

Verified on 2026-07-11 without installing system-wide dependencies.

| Component | Result |
| --- | --- |
| Operating system | Windows 11 Pro, build 10.0.26200, x64 |
| Rust | `rustc 1.96.0`, stable `x86_64-pc-windows-msvc` |
| Cargo | `cargo 1.96.0` |
| Project MSRV check | User-scoped Rust/Cargo `1.88.0`; full workspace check required by ADR-0012 |
| Installed Rust targets | `x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu` |
| Node.js | `v24.16.0` |
| npm | `11.13.0`; selected by `package-lock.json` |
| Git | `2.53.0.windows.2` |
| Tauri CLI | Repository-local `@tauri-apps/cli 2.11.4`; no global `cargo-tauri` |
| Tauri Rust crates | `tauri 2.11.5`, `tauri-build 2.6.3` |
| MSVC | Visual Studio Build Tools/Community 2026 detected by Tauri; `cl.exe` is not on a normal PowerShell `PATH` |
| WebView2 | Runtime `150.0.4078.48` detected |
| Linux prerequisites | Not verifiable on this Windows host |
| Rust security tools | User-scoped `cargo-audit 0.22.2` and `cargo-deny 0.20.2` |

The repository-local Tauri debug build completed successfully. Rust 1.88.0, cargo-audit, and cargo-deny were installed through the user-scoped Rust toolchain; no system-wide dependency was installed. ADR-0012 records the security-driven MSRV increase. Linux system packages and a Linux build remain CI/target-host checks.
