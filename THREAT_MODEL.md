# Threat Model

## Scope

This model covers the local desktop application, React renderer, Tauri IPC boundary, Rust application/domain code, SQLite data, local files and imports, optional shell/Git/platform adapters, packaging, dependencies, and future updater. Remote services are outside the MVP because none are permitted.

## Assets

- Redacted command history and fingerprints.
- Project metadata and canonical paths.
- Notes, sessions, favorites, timeline events, and privacy rules.
- SQLite database, configuration, logs, backups, exports, and diagnostic bundles.
- Encryption keys held by the operating-system secure store when implemented.
- Integrity of project files and shell profiles, which Dev Recall must not damage.

## Actors and assumptions

- A legitimate local user controls consent and data lifecycle.
- A malicious project, repository, history file, import, database row, or local filename supplies hostile data.
- A local attacker may alter application files or user-writable configuration.
- A compromised dependency or update channel may execute trusted code.
- Malware already running as the same OS user is outside the protection boundary and can generally read the same data.

## Trust boundaries and attack surfaces

| Boundary | Representative input | Required controls |
| --- | --- | --- |
| Renderer to IPC | JSON requests, navigation state | narrow commands, runtime validation, size limits, least privilege |
| Filesystem to Rust | paths, names, symlinks, manifests | canonicalization, allowed roots, no-follow where needed, limits, TOCTOU-aware operations |
| History/import to pipeline | bytes, encodings, huge/malformed records | streaming, format checks, limits, early redaction, cancellation |
| Domain to SQLite | structured records and queries | invariants, parameterized SQL, constraints, migrations, transactions |
| SQLite/log/export to renderer/files | stored hostile text | schema validation, safe text rendering, re-redaction, CSV protection |
| Build/update supply chain | packages and artifacts | locked dependencies, audits, minimal CI permissions, signatures before updates |

## Threats and mitigations

### Malicious display data and XSS

Commands, branch names, filenames, notes, and metadata may contain HTML, JavaScript, bidi controls, Unicode spoofing, or log-control characters. React renders text only; unsafe HTML and dynamic code are forbidden. CSP blocks remote content and unneeded sources. IPC remains least-privileged even if the renderer is compromised.

### Command and shell injection

The MVP never executes remembered commands and exposes no generic shell endpoint. Read-only Git/platform subprocesses, if later needed, use fixed executables and argument arrays without shell interpretation; all options and paths are validated.

### Path traversal, symlinks, special paths, and TOCTOU

Reject oversized, device, disallowed UNC, traversal, and out-of-scope paths. Canonicalize within an approved root, explicitly inspect symlinks, avoid following links for dangerous operations, and handle files changing between validation and use. Scans are depth, entry, metadata, size, duration, and cancellation limited and exclude heavy directories.

### SQL injection and corrupted databases

Only parameterized queries are allowed. Schema constraints, foreign keys, bounded transactions, migration compatibility tests, integrity checks, backup, and forward recovery handle corruption. Raw internal database errors never reach the UI.

### Secrets and sensitive terminal data

Size validation precedes a ReDoS-resistant redaction pipeline. The raw command is held only as briefly as processing needs, never logged or persisted, and replaced with `<REDACTED>`. User rules are count/length bounded and validated. Regression, malformed, property, and performance tests address false positives/negatives without claiming perfect detection.

### Oversized and malicious imports

Imports are allow-listed, signature/format checked, streaming, bounded, cancellable, and transactional/checkpointed. Archive expansion is not required for initial history import; if introduced, entry count, ratio, total size, nesting, link, and traversal controls prevent zip bombs.

### ReDoS and resource exhaustion

Use Rust's bounded regex engine, cap input/rule sizes and counts, stream large files, batch database writes, use bounded queues/backpressure, paginate results, cap caches/logs, and cancel owned work at shutdown.

### Log, diagnostic, crash, and export leakage

Structured logs accept safe identifiers and counts, normalize control characters, rotate, expire, and omit raw commands and secrets. Panic/crash reports are local and sanitized. Diagnostic bundles are previewed and exclude the database by default. Export re-runs redaction and neutralizes CSV cells beginning with `=`, `+`, `-`, or `@`.

### Weak file permissions and temporary files

Application data, backups, and logs use user-only permissions where supported. Secrets are never written to temporary files. Configuration writes are atomic; temporary paths are private and safely created.

### Capability and IPC abuse

Tauri capabilities are deny-by-default and separated by purpose. Filesystem, shell, process, environment, and database access are not exposed to the frontend. CSP, no remote content, release devtools policy, runtime input validation, and IPC regression tests reduce XSS-to-IPC escalation.

### Supply-chain, DLL loading, and updates

Review and lock dependencies, prohibit branch dependencies and downloaded runtime code, audit licenses/vulnerabilities, and use least-privilege pinned CI actions. Windows library loading must avoid user-controlled search locations. Auto-update is off until signatures, integrity, HTTPS, version checks, key rotation, downgrade/rollback protection, and revocation are established.

### Local configuration tampering

Validate and version configuration; reject insecure or malformed values. Never silently disable redaction, expand allowed roots, or fall back from protected to plaintext storage.

### Local HTTP and debug endpoints

No standalone local server or debug endpoint is permitted. If one is ever approved, its CSRF, origin, authentication, binding, and shutdown model requires a new ADR and threat-model revision.

## Security invariants

1. Dev Recall does not execute stored commands.
2. Unredacted terminal commands never cross the persistence or logging boundary.
3. Project removal never deletes project files.
4. Collection is off until explicit consent and is independently controllable.
5. No user data leaves the device in the MVP.
6. Failure of secure key storage never causes silent plaintext fallback.

## Verification plan

Security regression suites will cover XSS strings, traversal/device/UNC paths, symlinks and races, SQL injection, malformed encodings and formats, oversized records, regex attacks, fake credentials, log controls, bidi text, CSV formulas, cancellation, corruption, and least-privilege Tauri configuration. Dependency audits run after dependency changes and before releases.

## Residual risk

Redaction cannot recognize all secret formats, filesystem races cannot be eliminated on every platform API, OS-specific metadata may require unavailable permissions, and same-user malware can access local data. The UI and documentation must state these limits honestly.
