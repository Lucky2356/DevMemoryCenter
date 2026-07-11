# ADR-0012: Security-driven Rust 1.88 MSRV

## Status

Accepted. This supersedes the Rust 1.85 workspace baseline in ADR-0010 without changing its SQLx 0.8 selection.

## Context

The first automated RustSec scan found two High denial-of-service advisories in `quick-xml 0.38.4` and one Medium stack-exhaustion advisory in `time 0.3.45`. Both packages entered through Tauri's `plist 1.8.0` dependency. The maintained `plist 1.10.0` release upgrades to patched `quick-xml 0.41.0` and `time 0.3.53`, but declares Rust 1.88 as its minimum supported version.

## Decision

Raise the workspace MSRV from Rust 1.85 to Rust 1.88 and lock `plist 1.10.0`, `quick-xml 0.41.0`, and `time 0.3.53`. Verify the complete workspace with the user-scoped Rust 1.88.0 toolchain and keep stable Rust as the normal CI toolchain.

This decision does not approve SQLx 0.9 or any other dependency expansion. Future MSRV increases remain deliberate reviewed changes.

## Considered alternatives

- Keep Rust 1.85 and accept the High advisories: rejected because known High vulnerabilities block feature work.
- Vendor or patch `plist` locally: rejected because it would create a private third-party maintenance fork and additional supply-chain responsibility.
- Add `quick-xml 0.41` directly while retaining `plist 1.8`: rejected because the incompatible semver range would retain vulnerable `quick-xml 0.38` in the active graph.
- Wait for another upstream release: rejected because a maintained patched release already exists and the current development environment supports the required stable compiler.

## Security implications

The update removes RUSTSEC-2026-0194, RUSTSEC-2026-0195, and RUSTSEC-2026-0009 from the resolved application graph. Automated `cargo audit` and `cargo deny check` gates prevent their silent reintroduction.

## Privacy implications

The compiler and transitive parser updates do not add collection, persistence, network behavior, or access to user data.

## Performance implications

The patched XML parser replaces pathological duplicate-attribute behavior with bounded handling. Normal application behavior is unchanged; compile time may change slightly because of the dependency update.

## Consequences

- Contributors need Rust 1.88 or newer.
- The repository retains an explicit Rust 1.88 verification command.
- Linux and Windows builds must remain green after the dependency update.
- The inactive SQLx MySQL dependency recorded in `Cargo.lock` and Linux GTK lifecycle advisories remain separately documented and reviewed.
