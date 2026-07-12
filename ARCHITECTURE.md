# Architecture

## Status and scope

This document defines the intended MVP architecture. The current codebase is a compileable desktop foundation with one narrow health IPC contract; it does not yet implement the product layers described below.

## Drivers

- Local-first and offline by default.
- Explicit consent for each data source.
- Redaction before persistence.
- Deny-by-default desktop capabilities and narrow IPC.
- Bounded memory, I/O, concurrency, and retention.
- Windows and Linux platform isolation.
- Small, testable modules without premature distributed-system abstractions.

## Logical layers

```text
React presentation
       |
typed, narrow Tauri IPC
       |
application use cases  -- cancellation/progress/policy
       |
domain model           -- invariants, no framework dependencies
       |
ports/interfaces
       |
SQLite, filesystem, Git, shell-history and platform adapters
```

The `dev-recall-persistence` crate implements the first SQLite boundary with SQLx 0.8.6. ADR-0011 narrows its features to Tokio runtime, SQLite, and migrations and embeds immutable SQL without query macros. The adapter owns a maximum of four connections, bounded SQLite worker buffers and statement cache, acquisition/busy/idle/lifetime timeouts, explicit close, foreign keys, full synchronous mode, and rollback journal mode. It accepts only a canonical application-data directory, constructs the database filename internally, rejects existing non-files/symlinks, and creates a private file on Unix. The desktop composition root does not initialize it yet, so no user data is currently persisted.

The `dev-recall-observability` crate provides bounded local JSON-lines logging without a production dependency. Its event API uses closed enums and numeric values only, excluding arbitrary commands, notes, paths, headers, credentials, and error text by construction. The Tauri composition root owns the logger. Before onboarding consent it creates only an empty application-data-scoped log and performs retention cleanup; no event is emitted. Defaults are a 1 MiB active file, five archives, and seven-day retention, with bounded configurable limits and explicit clearing.

The `dev-recall-application` crate provides the application-owned lifecycle boundary for future long-running work without spawning work itself. At most four operations are admitted by the desktop composition root, operation identifiers are nonzero and unique while active, and each permit exposes a scoped cancellation token. Starting shutdown permanently closes admission, signals every active permit, and waits through a condition variable for at most the caller-supplied duration. The Tauri exit path uses a five-second bound. A timed-out operation remains visible to the owner and may complete cleanup; dropping the manager also signals cancellation. Concrete executors must retain and join their own task handles and must not detach work.

The `dev-recall-domain` crate contains the first framework-independent entity. `Project` uses distinct UUID-compatible project and owner identifiers, bounded display/description text, a closed project-type vocabulary, ordered nonnegative millisecond timestamps limited to SQLite's signed integer range, explicit archive state, and per-source privacy consent that defaults to disabled. It retains root and canonical path placeholders only after non-empty domain validation; platform length, absolute-path, canonicalization, traversal, special-path, and symlink checks belong to the next platform boundary and are not claimed here.

The frontend never receives unrestricted filesystem, database, environment, shell, or process access. Rust validates every request and returns typed, sanitized errors. Domain and application crates remain independent of Tauri.

## Proposed repository structure

```text
apps/desktop/             React/Vite presentation
apps/desktop/src-tauri/   narrow desktop composition root
crates/domain/            entities and invariants
crates/application/       use cases and ports
crates/observability/     bounded privacy-safe local diagnostics
crates/persistence/       SQLite repositories and migrations
crates/privacy/           redaction and retention policy
crates/terminal-history/  bounded import parsers
crates/git-context/       read-only Git adapter
crates/platform/          Windows/Linux/WSL adapters
crates/search/            deterministic local search
docs/                     decisions and operational guidance
migrations/               append-only SQL migrations
tests/                    cross-component tests and safe fixtures
```

Only modules required by the current phase should be created. The first skeleton contains the desktop composition root; domain crates are added one small task at a time.

## Trust boundaries

1. Project paths, repository metadata, terminal history, imports, and database contents are untrusted.
2. React is an unprivileged renderer. XSS must not become privileged IPC access.
3. Tauri commands are the privilege boundary and must be narrow, typed, size-limited, and policy checked.
4. Persistence accepts only validated, redacted domain values through parameterized queries.
5. OS key stores are a separate trust boundary; unavailable secure storage must fail closed for protected collection.

## Domain model

All persistent entities use stable UUID-compatible identifiers and an `owner_id`. The local MVP creates one internal local owner without embedding single-user assumptions in domain rules. Only `Project` is implemented; the remaining entries describe the approved incremental model.

- `Project`: identity, owner, display and canonical paths, type, description, timestamps, archive state, privacy settings.
- `WorkSession`: identity, owner/project, goal, state transitions, timing, summary, next step.
- `CommandRecord`: identity, owner/project/session, shell/platform, working directory, redacted command, fingerprint, timing, exit metadata, source, privacy state. No raw-command field is permitted.
- `TimelineEvent`: identity, relationships, type/time, safe summary, versioned and bounded metadata, source, severity.
- `FavoriteCommand`: identity, relationships, redacted template, description, shell/platform, timestamps.
- `PrivacyRule`: identity, owner, bounded rule type/pattern, enabled state, priority, timestamps.

## Data flow invariant

```text
untrusted bytes -> size/format limits -> early redaction -> validation
-> domain value -> parameterized persistence -> safe text rendering
```

Raw terminal commands must never be logged, persisted to temporary files, included in errors, or stored in the database.

## Background work

Long operations have one owner, bounded queues and concurrency, progress, cancellation, explicit shutdown, and recoverable checkpoints. No detached tasks or high-frequency polling are allowed.

The current lifecycle coordinator covers admission, identity, cancellation, cleanup notification, and bounded shutdown only. Progress, queues, checkpoints, and task executors belong to the individual long-running feature that requires them and are not fabricated in Phase 1.

## Localization

English and Russian resources are compiled into the application; no locale bundle is downloaded at runtime. The presentation layer resolves browser preferences and formats locale-sensitive values with platform `Intl` support. A small Rust boundary embeds localized safe startup errors for failures that occur before the webview exists. Unsupported locales fail predictably to English, and reusable components do not contain user-facing fallback strings.

## Presentation shell

The React shell keeps navigation state in one bounded `NavigationId` union and does not introduce a router dependency before URL/history behavior is required. Native controls, semantic landmarks, a skip link, visible focus, `aria-current`, and live-region/status semantics provide the keyboard and screen-reader foundation. A shared discriminated screen-state renderer prevents individual screens from omitting loading, empty, normal, error, or disabled behavior.

## Initial IPC boundary

`get_application_health` is the only application command currently registered. It accepts one correlation ID limited to 64 ASCII bytes and returns only a fixed readiness status, API version, and the validated ID. It does not inspect the operating system, filesystem, environment, network, database, or user data. Rust rejects unknown fields and unsafe IDs with a fixed serialized error; the TypeScript adapter independently validates requests, responses, and errors before exposing a discriminated result.

## Architecture risks

- History formats omit working directory, exit status, and timestamps; missing metadata must remain explicit.
- Cross-platform path canonicalization and symlink/TOCTOU behavior require dedicated adapters and tests.
- Redaction has unavoidable false positives and negatives and must be treated as a security boundary, not a guarantee.
- SQLite FTS5 availability and million-row performance must be proven on packaged Windows/Linux builds.
- At-rest encryption and OS key-store availability vary by platform; silent plaintext fallback is forbidden.
- Tauri configuration drift could broaden capability or CSP scope; configuration needs regression review.
- Shell profile integration can corrupt user configuration; it is opt-in, previewed, reversible, backed up, and deferred beyond initial import.
- Optional Docker/process context can increase privileges and surveillance risk; it remains disabled and requires a later decision.

## Related decisions

Accepted decisions are in [docs/adr](docs/adr). Threats are analyzed in [THREAT_MODEL.md](THREAT_MODEL.md).
