# ADR-0004: Terminal History Import and Shell Integration Boundaries

## Status

Accepted

## Context

History restores useful context but is incomplete, sensitive, shell-specific, and dangerous to execute or intercept.

## Decision

First implement explicit, previewed, read-only file import for one PowerShell and one Bash source. Stream records through size checks, redaction, exclusions, validation, fingerprinting, and persistence. Missing metadata remains missing. Never execute imported commands or capture stdout/stderr, keystrokes, interactive input, or clipboard data.

Shell-profile integration is deferred. If approved, it must be separately installed on explicit request, preview exact edits, back up the profile, use an idempotent marked block, be fully removable, and avoid secrets/output.

## Considered alternatives

- Hidden shell hooks or keylogging: prohibited and surveillance-like.
- Import every shell immediately: expands parser and platform risk before the pipeline is proven.
- Generic shell execution: prohibited.

## Security implications

Files, encodings, and commands are malicious input; imports require limits, format detection, no symlink following, cancellation, adversarial tests, and no raw-data logs.

## Privacy implications

Sources are separately opt-in with redacted preview, exclusions, retention, and deletion.

## Performance implications

Streaming, bounded buffers, batches, backpressure, progress, and checkpoints support large histories without UI blocking.

## Consequences

Initial metadata fidelity is intentionally limited. Each additional shell is a focused task with fixtures and platform tests.
