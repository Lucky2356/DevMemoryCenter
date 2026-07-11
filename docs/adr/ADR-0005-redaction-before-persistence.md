# ADR-0005: Secret Redaction Before Persistence

## Status

Accepted

## Context

Terminal history commonly contains credentials. Persisting first and redacting later creates an unacceptable exposure window.

## Decision

Enforce the pipeline: bounded input -> local redaction -> exclusions -> domain validation/fingerprint -> persistence. Store only `<REDACTED>` representations and never define a raw-command database field. Avoid raw commands in logs, errors, traces, snapshots, crashes, diagnostics, and temporary files.

Use a linear-time/ReDoS-resistant regex engine. Bound rule length/count and command length. Re-run privacy filtering on export.

## Considered alternatives

- Encrypt raw commands: rejected; encryption does not remove exposure or misuse risk.
- Redact after insertion: rejected due to plaintext persistence and crash windows.
- User-only manual exclusions: insufficient and error-prone.

## Security implications

Redaction is a security boundary requiring unit, regression, malformed, property/fuzz, performance, and false-positive/negative review. It cannot guarantee detection of unknown formats.

## Privacy implications

Early minimization reduces retained sensitive data; users retain custom rules, preview, exclusion, and deletion controls.

## Performance implications

Input/rule limits and bounded regex behavior prevent pathological CPU/memory consumption; secret-heavy benchmarks are required.

## Consequences

Some useful command text may be hidden, and unrecognized secrets remain residual risk communicated to users.
