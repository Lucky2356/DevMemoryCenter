# ADR-0009: At-rest Encryption and Key Storage

## Status

Accepted as security policy; implementation mechanism deferred pending prototype

## Context

The local database may contain sensitive work context. Cross-platform database encryption and secure key availability have packaging, licensing, recovery, and usability tradeoffs.

## Decision

Do not invent cryptography and do not claim full at-rest protection before implementation and verification. Evaluate a maintained SQLite-compatible encryption mechanism in a dedicated task. Generate keys with an approved cryptographic API and store them separately in Windows DPAPI/Credential Manager or Linux Secret Service-compatible storage. If secure storage is unavailable, fail closed for protected collection, explain the condition, and allow collection to remain disabled; never silently fall back to plaintext.

Backups, rotation, recovery, deletion, migration, and corruption behavior are part of the design before enabling encryption.

## Considered alternatives

- Plain SQLite indefinitely: insufficient for the target sensitivity.
- Key beside database or hardcoded key: provides no meaningful separation.
- Custom field encryption: high cryptographic and query/migration risk.
- Silent plaintext fallback: violates user expectations and policy.

## Security implications

The chosen library, key lifecycle, memory exposure, permissions, backup, rollback, and recovery require threat-model and dependency review. Same-user malware remains outside the guarantee.

## Privacy implications

Encryption reduces offline disclosure but does not replace redaction, minimization, retention, consent, or deletion.

## Performance implications

Startup, migration, import, search, backup, and database-size costs must be benchmarked on Windows and Linux before acceptance.

## Consequences

Persistence may initially remain disabled or explicitly marked unprotected until the secure implementation is complete. The final library choice requires evidence, not an automatic owner decision.
