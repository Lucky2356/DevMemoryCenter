# Security Findings

## Bundled SQLite version trails upstream security fixes

### Severity

Medium

### Affected component

`sqlx-sqlite 0.8.6` / bundled `libsqlite3-sys 0.30.1` / SQLite 3.46.0.

### Description

The selected SQLx 0.8.6 dependency bundles [SQLite 3.46.0](https://sqlite.org/releaselog/3_46_0.html). [SQLite's official vulnerability page](https://sqlite.org/cves.html) records later fixes in SQLite 3.49.1, 3.50.2, 3.50.3, and 3.53.2. Reported preconditions include arbitrary SQL execution, attacker-controlled very large SQL function arguments, or a corrupted FTS5 index.

### Impact

The current Phase 1 adapter does not expose generic SQL, does not accept external database files, creates no virtual/FTS table, and is not initialized by the desktop application. The known preconditions are therefore not reachable through current product behavior. Risk would increase materially before FTS5, database import/recovery, or any less constrained query surface is added.

### Reproduction using safe test data

The resolved bundled header reports SQLite 3.46.0. Compare this version with the fix versions in SQLite's official vulnerability list. No exploit payload is required or included.

### Remediation

Before enabling FTS5 or producing a release, move to a maintained driver/native SQLite combination that bundles a current patched SQLite version, then rerun migration, packaging, search, corruption, and performance tests on Windows and Linux. Do not compensate by enabling arbitrary extension loading or an unverified system SQLite fallback.

### Regression test

`migrates_empty_database_with_foreign_keys_and_initial_schema` asserts that the initial schema contains zero virtual tables and WAL remains disabled. The persistence crate exposes no generic SQL API outside its private test surface.

### Status

Mitigated for the current Phase 1 scope; upgrade remains required before FTS5 and release.

## Vulnerable transitive XML and time parsers

### Severity

High

### Affected component

Former lockfile versions `plist 1.8.0` / `quick-xml 0.38.4` / `time 0.3.45`, reached through Tauri.

### Description

The first automated RustSec scan reported RUSTSEC-2026-0194 and RUSTSEC-2026-0195 for CPU- and memory-exhaustion behavior in `quick-xml`, plus RUSTSEC-2026-0009 for stack exhaustion in `time` when parsing malicious RFC 2822 input.

### Impact

Dev Recall does not currently accept plist/XML or RFC 2822 input from users, which limited reachability in the existing skeleton. The High advisories nevertheless violated the dependency commit gate and blocked feature development.

### Reproduction using safe test data

Run `cargo audit` against commit `d24a68e`. The advisory report identifies the affected locked versions; no malicious document or exhaustion payload is required.

### Remediation

Raised the workspace MSRV to Rust 1.88 and updated to `plist 1.10.0`, `quick-xml 0.41.0`, and `time 0.3.53`. ADR-0012 records the security-driven compatibility decision.

### Regression test

`cargo audit` and `cargo deny check` are repository gates and run in the dependency-security CI job. Rust 1.88.0 checks the complete workspace.

### Status

Resolved locally; CI execution remains unverified until an authorized push.

## Inactive SQLx MySQL RSA advisory

### Severity

Medium

### Affected component

`rsa 0.9.10`, recorded through SQLx's disabled optional MySQL backend.

### Description

RUSTSEC-2023-0071 reports a timing side channel in the `rsa` crate with no fixed release. Cargo records SQLx's optional MySQL package family in `Cargo.lock` even though Dev Recall enables only `migrate`, `runtime-tokio`, and `sqlite`.

### Impact

`cargo tree --workspace --all-features --target all -i rsa` returns no active path. Dev Recall performs no RSA operations and has no MySQL backend, network database support, or TLS feature, so the vulnerable code is not compiled or reachable.

### Reproduction using safe test data

Run `cargo audit` to observe the advisory in the lockfile, then run `cargo tree --workspace --all-features --target all -i rsa` to verify that it is absent from the active graph.

### Remediation

Keep SQLx's MySQL feature disabled. The project audit configuration contains the exact advisory ignore; remove it if SQLx stops recording the inactive backend or a fixed RSA release becomes available. Any future MySQL or RSA feature would require a new review and removal of the exception before acceptance.

### Regression test

`cargo deny check` evaluates the active Windows/Linux graph and the documented `cargo tree` command verifies absence. The exact `cargo audit` exception remains visible in `.cargo/audit.toml` and this finding.

### Status

Mitigated by non-reachability; tracked exception, not accepted for any active dependency path.

## Tauri Linux GTK3 lifecycle and GLib iterator advisories

### Severity

Medium

### Affected component

Tauri's Linux WebKit/GTK3 transitive graph, including `glib 0.18.5` and GTK3 bindings `0.18.x`.

### Description

RustSec marks the GTK3 binding generation as unmaintained and reports RUSTSEC-2024-0429 for unsound `Iterator` implementations on `glib::VariantStrIter`. These packages are required by the current Tauri Linux backend.

### Impact

The application does not call `VariantStrIter` or expose a GLib value iteration API. The unmaintained dependency family still increases future Linux maintenance risk and must be removed when the upstream desktop stack provides a supported migration.

### Reproduction using safe test data

Run `cargo audit` or `cargo deny check` and inspect the Tauri-to-GLib inclusion path. No unsafe input or exploit payload is required.

### Remediation

Track Tauri/Wry's supported Linux backend and migrate when an upstream release removes the affected GTK3/GLib generation. Do not add direct GTK3 or GLib usage. Reassess before the first Linux release.

### Regression test

`cargo deny check` denies all unsound advisories except the single reasoned RUSTSEC-2024-0429 exception and fails on any workspace unmaintained dependency. `cargo audit` keeps informational warnings visible.

### Status

Open and mitigated for the current skeleton; blocks claims of Linux release readiness, but no High/Critical issue remains.
