# ADR-0003: SQLite Persistence Strategy

## Status

Accepted for architecture; driver selection deferred to the persistence task

## Context

The MVP needs transactional local storage, migrations, structured filtering, and potentially FTS5 over at least one million command records.

## Decision

Use SQLite behind repository interfaces with append-only versioned SQL migrations, foreign keys, constraints, parameterized queries, bounded connections, transactions, consistent timestamps, schema compatibility checks, backup/recovery, and tested FTS5. Do not enable WAL until packaged Windows/Linux behavior, growth, and checkpointing are verified.

## Considered alternatives

- Embedded key/value store: weaker relational/query and migration fit.
- Separate database server: operational and attack-surface cost is unjustified.
- Direct frontend database access: violates the privilege boundary.

`sqlx` is the leading driver candidate; the implementation task must compare it with maintained alternatives for features, licenses, transitive graph, migration/test ergonomics, and binary impact.

## Security implications

Parameterized SQL, validation, constraints, file permissions, integrity checks, and sanitized errors are mandatory. Destructive migrations require recovery and approval when data loss is possible.

## Privacy implications

The schema never includes raw command secrets, credentials, full environment dumps, terminal output by default, or source contents.

## Performance implications

Batch writes, bounded pools, indexes justified by measured queries, pagination, and controlled maintenance prevent per-record transactions and unbounded WAL growth.

## Consequences

Migrations are immutable after release and tested from empty, previous, and representative populated databases.
