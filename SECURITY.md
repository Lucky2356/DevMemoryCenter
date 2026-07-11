# Security

## Security posture

Dev Recall is local-first, deny-by-default, and non-executing. The MVP must not provide generic shell execution, arbitrary file access, remote content, cloud sync, telemetry, or elevated administration.

The initial skeleton collects and persists no user data and exposes no application IPC commands.

## Core controls

- Treat paths, history, Git metadata, imports, database rows, and UI text as untrusted.
- Validate type, length, format, scope, canonical path, and symlink behavior before access.
- Redact terminal input before domain processing or persistence; never create raw-command storage.
- Render untrusted values as text; forbid `dangerouslySetInnerHTML`, `eval`, remote scripts, and unsafe URL schemes.
- Use narrow typed IPC commands with sanitized error codes and explicit request limits.
- Use parameterized SQL, migrations, foreign keys, constraints, and bounded connections.
- Use structured local logs with rotation/retention; exclude commands, secrets, notes, source, headers, cookies, full environment dumps, and unnecessary full paths.
- Own and cancel every background task; bound queues and caches.

## Vulnerability reporting

Do not put secrets or personal command history in reports. Record repository findings in `SECURITY_FINDINGS.md` only when a concrete issue exists. Critical and High findings block feature work.

## Dependency and release security

Dependencies require necessity, maintenance, license, vulnerability, platform, permission, and transitive-impact review. Git branch dependencies, remote runtime code, and unnecessary install scripts are prohibited. Auto-update remains disabled until signed release infrastructure and downgrade protection are designed and tested.

## Protection limits

Dev Recall cannot protect data from malware or another process already running with the same user's permissions. Redaction reduces exposure but cannot guarantee detection of every secret. At-rest protection must not be claimed until implemented and verified.
