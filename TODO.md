# TODO

## Current phase

Phase 1 — Application foundation

### In progress

None. The Windows/Linux CI task is complete.

### Ready

#### Phase 1 — Application foundation

- [ ] Add English/Russian localization infrastructure and move all shell strings into locale resources.
- [ ] Add accessible application navigation with loading, empty, error, normal, and disabled states.
- [ ] Add light, dark, and system theme support with reduced-motion behavior.
- [ ] Define typed sanitized application errors and the first narrow IPC health command with runtime limits.
- [ ] Compare maintained SQLite Rust drivers and document the selected dependency.
- [ ] Add the bounded SQLite connection setup and immutable initial migration.
- [ ] Test migrations from empty and representative databases.
- [ ] Add privacy-safe structured local logging with rotation and retention.
- [ ] Configure `cargo-deny`, dependency audits, frontend audits, and secret scanning.
- [ ] Add application-owned background-operation lifecycle and shutdown tests.

#### Phase 2 — Local projects

- [ ] Add the `Project` domain entity and validation tests.
- [ ] Implement platform path validation, canonicalization, special-path, traversal, and symlink tests.
- [ ] Add a narrow, consent-aware project record command and repository.
- [ ] Add bounded manifest detection without recursive source scanning.
- [ ] Add read-only Git repository, branch, commit, and dirty-state inspection.
- [ ] Add paginated project list and accessible empty/error/loading states.
- [ ] Add project detail, archive, and metadata-only removal confirmation.

#### Phase 3 — Terminal Memory import

- [ ] Define bounded import/redaction domain types with no raw persistence field.
- [ ] Implement and adversarially test the built-in secret redaction pipeline.
- [ ] Add bounded user redaction/exclusion rules and ReDoS tests.
- [ ] Implement a streaming PowerShell history parser with malformed/oversized fixtures.
- [ ] Implement a streaming Bash history parser with malformed/oversized fixtures.
- [ ] Add redacted import preview with explicit source consent.
- [ ] Add batched transactional persistence, deduplication, progress, cancellation, and restart recovery.
- [ ] Add source pause, deletion, retention, and privacy explanations.

#### Phase 4 — Search

- [ ] Verify packaged SQLite FTS5 support on Windows and Linux.
- [ ] Add deterministic local indexed search with parameterized queries and bounded filters.
- [ ] Add cursor pagination, stale-request cancellation, and keyboard navigation.
- [ ] Benchmark first-page search on 1,000,000 synthetic redacted records.

#### Phase 5 — Work sessions

- [ ] Add `WorkSession` state transitions and invariant tests.
- [ ] Add start, pause, resume, and finish use cases with recovery.
- [ ] Add localized goal, safe note, summary, and next-step UI states.
- [ ] Link sessions to imported events without duplicating large datasets.

#### Phase 6 — Timeline

- [ ] Add versioned, size-limited `TimelineEvent` metadata.
- [ ] Add paginated event queries and safe grouping rules.
- [ ] Add accessible virtualized day/project timeline views and filters.

#### Phase 7 — Favorites

- [ ] Add redacted favorite command templates and bounded parameter validation.
- [ ] Add tags, project/platform/shell metadata, and safe text rendering.
- [ ] Add explicit clipboard copy without command execution or background reads.

#### Phase 8 — Privacy Center

- [ ] Add source, storage, exclusion, redaction, and last-import status views.
- [ ] Add bounded cancellable retention cleanup per data category.
- [ ] Add re-redacted JSON and Markdown export.
- [ ] Add CSV export formula-injection protection and tests.
- [ ] Add scoped transactional deletion and full reset without touching project files.
- [ ] Add previewed, sanitized diagnostic bundle export.

#### Phase 9 — Optional read-only local context

- [ ] Obtain owner approval for optional Docker, local port, and process context scope.
- [ ] If approved, threat-model minimal read-only adapters with no remote Docker, scanning, process control, or elevation.

#### Phase 10 — Security, performance, and release hardening

- [ ] Complete security and privacy audit and resolve blocking findings.
- [ ] Run parser/redaction fuzzing and long-duration resource tests.
- [ ] Profile idle, import, search, shutdown, and frontend memory behavior against budgets.
- [ ] Complete WCAG 2.2 AA-oriented accessibility audit.
- [ ] Test backup, corruption, migration, interruption, and full-reset recovery.
- [ ] Review dependency licenses, SBOM, installers, capabilities, CSP, and release devtools.
- [ ] Design and verify signed releases and updater before enabling updates.

### Blocked

- [ ] Select the repository license — owner decision required; see `NEEDS_USER_INPUT.md`.

### Completed

- [x] Read and analyze `AGENTS.md` and `SPEC.md` completely.
- [x] Inspect the initial repository and local toolchain without system-wide installation.
- [x] Prepare requirements analysis, exact MVP, assumptions, risks, architecture, privacy model, threat model, wireframes, test strategy, and performance budgets.
- [x] Record nine initial ADRs.
- [x] Create the minimal Tauri 2, React, strict TypeScript, and Vite shell with no collection, persistence, IPC commands, or MVP feature behavior.
- [x] Verify formatting, linting, type checks, tests, frontend build, Rust check/clippy, npm audit, Tauri environment, and Windows Tauri debug build.
- [x] Add read-only Windows/Linux CI with fixed runner images, full-SHA-pinned actions, locked dependency installation, frontend/Rust checks, npm audit, and Tauri debug builds.
