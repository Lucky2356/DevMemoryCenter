# ADR-0006: Tauri Capability and IPC Security Model

## Status

Accepted

## Context

The renderer handles hostile display data and must not become a general bridge to local privileges.

## Decision

Use deny-by-default, purpose-specific Tauri capabilities and a strict local-only CSP. Expose only narrow domain commands with typed requests/responses, runtime field and size validation, path policy checks, and sanitized error codes. The frontend receives no unrestricted filesystem, shell, process, environment, or database access. The shell plugin, remote content, release devtools, generic file/SQL/command endpoints, `eval`, and unsafe HTML are prohibited.

## Considered alternatives

- Broad convenience capabilities: unacceptable blast radius.
- A generic service endpoint with frontend policy: renderer compromise would bypass trust.
- Local HTTP API: unnecessary CSRF/origin/lifecycle surface.

## Security implications

XSS must remain contained by safe rendering, CSP, runtime validation, and least-privilege commands. Capability configuration needs regression review.

## Privacy implications

The renderer cannot silently enumerate local data sources; Rust enforces consent and scope.

## Performance implications

Command-specific pagination and payload limits avoid large IPC copies and renderer retention.

## Consequences

More explicit command types are required, but auditing and testing remain tractable.
