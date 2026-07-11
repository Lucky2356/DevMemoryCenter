# ADR-0002: Local-first Application Architecture

## Status

Accepted

## Context

Work context and terminal history are highly sensitive; MVP needs no remote collaboration.

## Decision

All MVP processing and persistence occur on the device. No standalone server, cloud sync, registration, remote content, telemetry, automatic crash upload, or application network request is permitted. Collection begins only after source-specific consent.

## Considered alternatives

- Cloud-first service: conflicts with privacy and MVP scope.
- Local HTTP service: adds origin, CSRF, port, lifecycle, and exposure risk without need.
- Hybrid sync-ready runtime: premature complexity; future boundaries remain documentation and ports.

## Security implications

Remote attack surface is minimized, but local files, IPC, dependencies, and same-user attackers remain threats.

## Privacy implications

Data stays local and is user-controlled; the UI must not imply protection against same-user malware.

## Performance implications

No network latency or background sync; local CPU, disk, database, and retention require strict budgets.

## Consequences

Cross-device features are absent. Any future network feature requires a new ADR, threat model, consent design, and owner approval.
