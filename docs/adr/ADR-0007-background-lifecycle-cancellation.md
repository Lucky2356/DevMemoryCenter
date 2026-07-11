# ADR-0007: Background Task Lifecycle and Cancellation

## Status

Accepted

## Context

Imports, scans, search maintenance, retention, export, and database maintenance can outlive UI interactions or shutdown.

## Decision

Every long operation has one application-level owner, an operation ID, bounded concurrency/queues, backpressure, progress, cancellation, partial-failure semantics, and an explicit shutdown path. No detached tasks, busy loops, unbounded channels/caches, locks across unrelated I/O, or database transactions across unrelated waits. Shutdown stops admission, signals cancellation, waits up to the documented bound, checkpoints safe state, and releases resources.

## Considered alternatives

- Fire-and-forget tasks: leaks lifecycle and error handling.
- Unbounded producer queues: memory-exhaustion risk.
- Synchronous UI-thread work: blocks accessibility and cancellation.

## Security implications

Limits resist resource exhaustion and duplicate operation abuse; cancellation must not bypass validation or leave partially trusted data committed.

## Privacy implications

Stopping a source or deleting data must also stop related work and release sensitive buffers quickly.

## Performance implications

Bounded pipelines stabilize memory and disk pressure. Async is used only for genuine waiting/concurrency benefit.

## Consequences

Operation state and restart tests are required for every long-running feature.
