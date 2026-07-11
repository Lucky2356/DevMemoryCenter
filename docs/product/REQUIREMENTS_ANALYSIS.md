# Requirements Analysis

## Exact MVP

The MVP is a local, single-user Windows/Linux desktop application that:

1. Provides consent-driven onboarding and English/Russian UI architecture.
2. Manages local project metadata without modifying project files; detects bounded manifest and read-only Git context.
3. Imports one PowerShell and one Bash history source through preview, size limits, early secret redaction, deduplication, progress, cancellation, and explicit consent.
4. Stores only validated/redacted records in migrated SQLite and supports deterministic local search with filters and bounded pagination.
5. Supports explicit work sessions, safe notes/next steps, a bounded timeline, and favorite redacted command templates that can be copied but never executed.
6. Provides Privacy & Data controls for sources, exclusions, redaction, retention, export, scoped deletion, and full reset.

Read-only Docker, port, and process context is Phase 9 and requires a separate owner decision. AI, cloud sync, accounts, remote administration, SSH, command execution, source indexing, shell modification without preview/consent, network scanning, keylogging, screen capture, background clipboard access, telemetry, automatic crash upload, marketplace plugins, and auto-update are excluded.

## Phase 0 deliverables

- Architecture, threat/privacy models, domain prototype, risks, assumptions, and contradictions.
- ADRs for stack, locality, persistence, shell boundaries, redaction, IPC, cancellation, future ownership, and encryption.
- Text wireframes, test strategy, performance budgets, phased TODO, and environment record.
- Only then, a minimal compileable Tauri/React shell with no MVP feature behavior.

## Assumptions

- One internal local owner/workspace is sufficient for MVP while IDs and repository boundaries preserve future ownership.
- CSS Modules is adequate and avoids a heavy UI dependency.
- Tauri local npm CLI is preferable to a machine-wide `cargo-tauri` installation.
- Ordinary history files cannot reliably provide all metadata; absent timestamps, directories, exit codes, or durations remain absent rather than inferred.
- Encryption is a staged capability; until verified, the product must not claim complete database protection.
- macOS is not tested or supported in the first release, though platform interfaces should not preclude it.

## Requirement tensions and resolutions

1. The specification asks to prepare future AI interfaces while prohibiting unused complex abstractions. Resolution: document interface boundaries in `FUTURE_AI.md`; do not create code until a deterministic use case needs a port.
2. The specification names many proposed crates while requiring simple architecture. Resolution: start with the desktop shell and create crates only with the first use case that owns them.
3. The MVP mentions many shell formats, but Phase 3 explicitly limits first implementation to PowerShell and Bash. Resolution: PowerShell and Bash are the exact MVP import milestone; other sources are later tasks within the MVP roadmap after the pipeline is proven.
4. Search targets one million records while performance budgets must be evidence based. Resolution: treat the target as an acceptance benchmark, not an unmeasured implementation claim.
5. Database encryption is required to be designed, but platform key stores can be unavailable. Resolution: fail closed for protected collection and never silently fall back; final UX and library selection are separate tested tasks.

No serious contradiction blocks the initial architecture or skeleton.

## Architectural risks

- Redaction quality and safe handling before persistence are the highest security risk.
- Path and symlink behavior differs materially across Windows, Linux, and WSL.
- Tauri capability mistakes can convert renderer compromise into local privilege abuse.
- History data is incomplete and sensitive; misleading reconstruction would reduce trust.
- Million-row search, migrations, retention, and imports can cause UI stalls or unbounded growth.
- OS key stores, packaging prerequisites, FTS5, and filesystem permissions vary by target.
- A broad dashboard or optional process observation could drift toward surveillance.

## Incremental feasibility

Each roadmap phase has an independently testable result: foundation, projects, bounded history import, search, sessions, timeline, favorites, privacy controls, optional read-only context, then hardening. Later phases depend on explicit completion gates from earlier phases; no phase requires implementing the whole product at once.
