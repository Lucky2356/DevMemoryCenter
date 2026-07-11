# ADR-0010: SQLx SQLite Driver Selection

## Status

Accepted for the Phase 1 persistence implementation. This decision refines ADR-0003 without changing its SQLite architecture constraints.

## Context

Dev Recall needs a maintained SQLite driver that supports parameterized queries, transactions, immutable SQL migrations, bounded connections, safe shutdown, and Windows/Linux packaging. The persistence adapter will run behind repository interfaces and must not expose the database to the renderer.

The comparison was performed on 2026-07-11 against the current project MSRV of Rust 1.85. The evaluated maintained candidates were SQLx, rusqlite, and Diesel. Primary evidence came from the projects' official repositories, crates.io metadata, docs.rs documentation, and the RustSec advisory database:

- [SQLx repository and feature documentation](https://github.com/launchbadge/sqlx)
- [SQLx 0.8.6 migration API](https://docs.rs/sqlx/0.8.6/sqlx/migrate/index.html)
- [SQLx SQLite connection model](https://docs.rs/sqlx/0.8.6/sqlx/sqlite/struct.SqliteConnection.html)
- [rusqlite repository and build options](https://github.com/rusqlite/rusqlite)
- [Diesel SQLite documentation](https://docs.rs/diesel/latest/diesel/sqlite/index.html)
- [RustSec SQLx advisory history](https://rustsec.org/packages/sqlx.html)
- [RustSec rusqlite advisory history](https://rustsec.org/packages/rusqlite.html)
- [RustSec Diesel advisory history](https://rustsec.org/packages/diesel.html)

## Decision

Use SQLx 0.8 for the first persistence adapter, starting with 0.8.6, and retain the Cargo lockfile. Configure the dependency with default features disabled and only these direct features:

```toml
sqlx = { version = "0.8.6", default-features = false, features = [
  "macros",
  "migrate",
  "runtime-tokio",
  "sqlite",
] }
```

The implementation task must confirm the resolved lockfile before accepting the dependency. The `sqlite` feature uses the bundled, statically linked SQLite build. Do not enable `sqlite-unbundled`, other database drivers, `any`, a TLS backend, JSON integration, regular-expression functions, pre-update hooks, or extension loading unless a later requirement and review justify them.

Use SQLx's normal parameter binding APIs rather than constructing SQL from user input. Embed append-only migrations with `migrate!`, validate previously applied migration checksums, and keep generated query metadata in version control if compile-time query macros require it. The SQLx CLI is not a runtime dependency and is not selected by this decision.

The persistence implementation must set explicit limits rather than accepting pool and channel defaults. It must use a small bounded connection pool, bounded worker command and row buffers, acquisition timeouts, and explicit close/shutdown handling. Exact values belong to the connection implementation task and must be covered by tests.

SQLx 0.9.0 is not selected because its declared Rust requirement is 1.94, above the workspace MSRV of 1.85. Moving to SQLx 0.9 requires a separate, deliberate MSRV decision and Windows/Linux validation. SQLx 0.8.6 does not publish a `rust-version` value in crates.io metadata, so compatibility with Rust 1.85 remains a commit-gate check when the dependency is actually added.

## Comparison

| Criterion | SQLx 0.8.6 | rusqlite 0.40.1 | Diesel 2.3.11 |
| --- | --- | --- | --- |
| Query model | Direct SQL with parameter binding and optional compile-time checking | Direct synchronous SQLite API with parameter binding | Typed ORM/query-builder plus raw SQL escape hatch |
| Migrations | Built-in ordered migration support and embedded `migrate!` macro | Requires a separate migration mechanism or project code | Separate maintained `diesel_migrations` crate with embedded migrations |
| Concurrency fit | Async API; each SQLite connection owns a background worker with bounded-buffer controls | Synchronous; application must own blocking isolation, pooling, and shutdown | Synchronous SQLite connection; optional r2d2 pool adds another layer |
| Scope control | Default features can be disabled; SQLite-only build avoids network database and TLS code | Smallest direct SQLite-specific API of the three | ORM/schema/code-generation surface is broader than the MVP needs |
| Transitive shape under evaluated features | SQLx facade, core, SQLite driver, macros, Tokio/futures support, and `libsqlite3-sys`; proc-macro tooling is compile-time | rusqlite, `libsqlite3-sys`, and default statement-cache support; migration, pool, and worker lifecycle need project or additional crates | Diesel, Diesel derives, `libsqlite3-sys`, and SQLite support; migrations and r2d2 pooling add separate crates/features |
| Native SQLite | `sqlite` bundles and statically links SQLite; `sqlite-unbundled` is optional | `bundled` is optional and recommended upstream for controlled desktop apps | SQLite support uses `libsqlite3-sys`; bundling requires explicit native-library configuration |
| Expected cost before measurement | More compile-time and worker/pool code than rusqlite, partly shared with the existing Tokio stack | Smallest driver surface, offset by application-owned lifecycle and migration code | Largest conceptual/code-generation surface for this use case |
| License | MIT OR Apache-2.0 | MIT | MIT OR Apache-2.0 |
| Project fit | Best match for Tauri's Tokio runtime, bounded background work, embedded migrations, and direct SQL | Strong fallback if measured SQLx cost is unacceptable | Capable but adds ORM and synchronous-pool complexity without an MVP requirement |

SQLx is selected because it combines embedded migrations, direct parameterized SQL, a bounded pool, backpressure controls, and an async boundary that does not block Tauri's executor. rusqlite remains the preferred fallback if cross-platform packaging, MSRV, or measured binary/resource cost makes SQLx unsuitable. Diesel is not selected because the project does not need an ORM or schema DSL and would still need deliberate blocking isolation.

## Considered alternatives

- **rusqlite with bundled SQLite:** mature, focused, and lighter conceptually. Rejected for the initial adapter because Dev Recall would need to build and own the blocking worker lifecycle, bounded job queue, pooling policy, and migration integration that SQLx already supplies.
- **Diesel with SQLite:** mature and strongly typed. Rejected because its ORM/query DSL, derive/code-generation surface, separate migration crate, and synchronous connection model add complexity that is not required by the specification.
- **SQLx 0.9:** current major line with an explicit Rust 1.94 requirement. Deferred because silently raising the project's Rust 1.85 MSRV is outside this task.
- **System SQLite through `sqlite-unbundled`:** can reduce duplicated native code and allows OS-managed updates. Deferred because Windows availability and Linux distribution versions are not deterministic enough for the initial packaged desktop baseline.
- **Tauri SQL plugin or frontend database access:** rejected because persistence belongs behind narrow Rust repository interfaces and must never broaden renderer capabilities.

## Security implications

- SQLx 0.8.6 is newer than the patched boundary for RUSTSEC-2024-0363 (`>=0.8.1`). A lockfile audit is still mandatory after the dependency is resolved because this documentation task does not create a dependency graph.
- Bundling SQLite gives the application a deterministic native engine across Windows and Linux, but security updates become the project's responsibility. Dependency update checks and packaged version verification are required.
- No extension loading, arbitrary SQL IPC, direct database handle, or frontend database capability is permitted.
- All application queries remain parameterized, inputs remain bounded and validated before persistence, and database errors are mapped to sanitized typed errors.
- SQLx internally uses native SQLite bindings and worker threads; application production code remains Safe Rust and must not access raw SQLite handles.

## Privacy implications

The selected feature set adds no remote database driver or TLS stack and performs no network access. Driver selection does not weaken the rule that raw commands, credentials, full environment dumps, source contents, and terminal output by default must never enter the schema.

## Performance implications

SQLx creates a worker thread for each SQLite connection because SQLite exposes a blocking API. The pool must therefore stay deliberately small, and command/row buffers must be bounded. The implementation task must measure the release binary delta and packaged idle resources; FTS5 throughput and WAL behavior remain separate platform validation tasks.

Bundled SQLite increases compile time and binary size but avoids runtime dependency on a missing or incompatible system SQLite. This trade-off must be re-evaluated with measured Windows and Linux packages before release.

## Consequences

- The next task may add only SQLx 0.8 with the reviewed feature set, a bounded connection setup, and the immutable initial migration.
- Adding SQLx is conditional on a clean resolved dependency audit, Rust 1.85 compatibility verification, and successful Windows build. Linux remains verified by CI or a representative Linux host.
- SQLx 0.9, system SQLite, encryption, FTS5, WAL, backup, and recovery are not approved by this decision; each remains governed by its dedicated task and existing ADRs.
- If the dependency fails the MSRV, audit, packaging, or measured-resource gates, supersede this ADR and use rusqlite rather than weakening the gates.
