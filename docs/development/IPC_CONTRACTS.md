# IPC Contracts

## Boundary rules

Every frontend/backend contract is purpose-specific, typed on both sides, validated at runtime, size-limited, and denied unless explicitly registered. The frontend must treat successful responses and rejected values as untrusted. Rust errors contain only stable codes, localization keys, and a retryability flag; internal errors and rejected input are never returned.

The frontend converts IPC outcomes into a discriminated `IpcResult<T>`. Unknown rejection values become `operation_failed`; malformed successful values become `invalid_response`. Components localize the fixed `messageKey` instead of rendering backend text.

## `get_application_health`

Purpose: confirm that the local Tauri command boundary is responsive and speaks API version 1.

Request:

```text
{
  correlationId: string
}
```

The correlation ID must contain 1–64 ASCII bytes from `A-Z`, `a-z`, `0-9`, `.`, `_`, or `-`. The frontend reconstructs the request from this single field. Rust uses `deny_unknown_fields` and repeats the same validation.

Success response:

```text
{
  status: "ready",
  apiVersion: 1,
  correlationId: string
}
```

The response is accepted only when the status and version are fixed and the correlation ID matches the request.

Rejected request:

```text
{
  code: "invalid_request",
  messageKey: "invalid_request",
  retryable: false
}
```

This command performs no filesystem, environment, process, shell, database, network, or user-data access. It is not a diagnostic or host-health endpoint and does not reveal application paths, dependency versions, operating-system details, timestamps, or internal errors. It is synchronous, performs bounded constant-time validation, spawns no task, and requires no Tauri capability or plugin.
