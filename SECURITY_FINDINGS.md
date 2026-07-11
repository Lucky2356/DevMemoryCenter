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

`opens_database_with_foreign_keys_and_initial_migration` asserts that the initial schema contains zero virtual tables and WAL remains disabled. The persistence crate exposes no generic SQL API outside its private test surface.

### Status

Mitigated for the current Phase 1 scope; upgrade remains required before FTS5 and release.
