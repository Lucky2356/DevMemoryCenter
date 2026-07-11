# ADR-0011: Embedded SQLx Migrations Without Macros

## Status

Accepted. This decision supersedes only the `macros` feature selection in ADR-0010; SQLx 0.8 and its other constraints remain accepted.

## Context

The first resolved SQLx 0.8.6 lockfile showed that enabling `macros` records SQLx's MySQL and PostgreSQL support crates and their supporting packages even though the normal runtime dependency tree contains only SQLite. Dev Recall needs embedded, checksum-validated migrations, but it does not need compile-time query macros in Phase 1.

## Decision

Use SQLx with only `migrate`, `runtime-tokio`, and `sqlite`, with default features disabled. Implement SQLx's maintained `MigrationSource` interface over SQL text embedded with Rust `include_str!`. Construct ordinary SQLx `Migration` values and let `Migrator` calculate and validate migration checksums.

Keep each migration as an append-only numbered SQL file in the repository. The embedded source must list every migration explicitly and in ascending order. Tests must verify the applied count and schema. Do not add `sqlx-cli` or query macros until a concrete task demonstrates that their value exceeds their dependency cost.

## Considered alternatives

- Keep `macros` for `migrate!`: convenient, but retains unrelated database packages in the lockfile.
- Load migrations from disk at runtime: smaller compile graph, but packaged applications could start without required schema files or accept modified files beside the executable.
- Construct `Migrator` through its hidden public fields: avoids a small source adapter but depends on explicitly semver-exempt implementation details.
- Implement migrations without SQLx: duplicates checksum, transaction, ordering, and compatibility behavior already supplied by the selected driver.

## Security implications

Removing `macros` reduces the recorded supply-chain surface. Embedded SQL cannot be replaced beside the packaged executable, and SQLx checks applied migration hashes. No migration accepts user input or enables extension loading.

The explicit migration list creates a review obligation: a new SQL file that is not added to the source will not run. Migration tests and review of the source list are mandatory.

## Privacy implications

This decision changes no collected or persisted data. The initial schema contains no command, credential, note, environment, source-content, or terminal-output field.

## Performance implications

Removing proc-macro and unrelated database packages reduces compile and audit work. Constructing the small migration list and its checksums once per database open is bounded and negligible; database migrations themselves remain transactional.

## Consequences

- The accepted direct SQLx features are now `migrate`, `runtime-tokio`, and `sqlite`.
- Compile-time checked query macros are unavailable until deliberately reconsidered.
- Every migration addition must update the embedded source and tests in the same commit.
