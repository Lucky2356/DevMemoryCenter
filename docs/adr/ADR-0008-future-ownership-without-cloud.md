# ADR-0008: Future Multi-user Compatibility Without Cloud MVP

## Status

Accepted

## Context

MVP is single-user, but schemas that encode paths as identity or assume global ownership would be costly to evolve.

## Decision

Use stable UUID-compatible IDs and explicit internal owner/workspace relationships for persistent domain objects. Keep domain, application, persistence, infrastructure, and desktop concerns separable. Use a local system owner in MVP, without accounts, fake authorization services, remote APIs, synchronization, or public path-based identifiers.

## Considered alternatives

- No ownership fields: simpler now but creates invasive future migrations and ambiguous isolation.
- Implement accounts/RBAC/sync immediately: prohibited scope and premature complexity.
- Global singleton domain state: hard to test and evolve.

## Security implications

Ownership fields are not authorization by themselves; future multi-user work must enforce tenant and object authorization at every boundary.

## Privacy implications

Local-only behavior remains unchanged. Future sync requires new consent and data-flow design.

## Performance implications

Stable identifiers and owner indexes have small storage costs; no network or sync overhead is introduced.

## Consequences

The MVP carries explicit ownership metadata but no user-facing account model.
