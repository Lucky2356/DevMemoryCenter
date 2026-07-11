# ADR-0013: Bounded structured local logging

## Status

Accepted for the Phase 1 observability foundation.

## Context

Dev Recall needs useful local diagnostics without turning logs into another store for commands, notes, credentials, paths, environment data, or attacker-controlled text. Logs must be structured, size- and time-bounded, private, locally removable, and inactive as a data source until consent exists.

## Decision

Implement a small Safe Rust observability crate using the standard library. Store one JSON object per line with a timestamp, level, component, numeric operation ID, fixed event code, and at most eight numeric metrics. Levels, components, event codes, and metric names are closed enums; the API accepts no arbitrary string value.

Use a fixed `logs/dev-recall.log` path below the platform application-data directory. Default to a 1 MiB active file, five 1 MiB archives, and seven-day retention. Validate custom limits between 4 KiB and 16 MiB, one and ten archives, and one and 365 days. Prune expired, oversized, and excess fixed-name files at startup and rotation, rotate synchronously before a write would exceed the cap, and expose explicit clearing for the future Privacy & Data flow.

Create the directory and files with `0700`/`0600` permissions on Unix and reject broad existing Unix permissions, symlinks, and non-file targets. Windows inherits the user application-data ACL and requires a release-target permission test. The Tauri composition root owns the logger, but emits no event before onboarding consent.

## Considered alternatives

- `tracing`, `tracing-subscriber`, and `tracing-appender`: capable and familiar, but add a broad formatting/filtering/background-worker surface before the project needs spans or asynchronous logging.
- Arbitrary key/value strings with runtime redaction: rejected because redaction is not yet implemented and mistakes would create a secret-persistence boundary.
- Console-only logging: rejected because it does not provide bounded local diagnostics, retention, or privacy deletion.
- Unbounded single log file: rejected because it violates disk and retention requirements.

## Security implications

The closed schema prevents log injection and raw sensitive string values by construction. Fixed paths, symlink/non-file checks, bounded files, bounded context, and fail-closed errors reduce local tampering and resource-exhaustion risk. Same-user TOCTOU and Windows ACL inheritance remain platform limits and are not claimed as complete protection.

## Privacy implications

No event is emitted before onboarding consent. The current managed logger creates only an empty local file and performs retention cleanup. Logs never leave the device, and `clear` removes archives and recreates an empty active file. A later Privacy & Data task must connect consent, retention settings, and the clear action to UI policy.

## Performance implications

There are no background threads, polling loops, channels, or caches. Each event allocates one bounded line and performs one mutex-protected append. Rotation performs bounded work over at most ten fixed archive paths and syncs only when closing a file for rotation or clearing.

## Consequences

- New event/component/metric vocabulary requires a reviewed enum change.
- Arbitrary diagnostics and raw error strings cannot be logged.
- The default total on-disk budget is at most approximately 6 MiB before filesystem overhead.
- Async logging can be reconsidered only after measured contention or latency demonstrates a need and an owned shutdown design exists.
