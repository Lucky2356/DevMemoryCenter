# Privacy Model

## Principles

- No collection before onboarding consent.
- Each source is independently opt-in, previewable, pausable, and removable.
- All processing is local; the MVP has no telemetry, cloud service, automatic crash upload, or remote content.
- Collect metadata needed to restore work context, not source contents, terminal output, keystrokes, screens, windows, clipboard contents, credentials, or environment dumps.
- Redact before persistence and apply privacy filtering again on export.

## User control

The planned Privacy & Data screen will explain sources, last imports, record counts, database size, exclusions, redaction rules, retention, export, scoped deletion, and full reset. Removing a project record never deletes source project files.

## Retention and deletion

Retention is configurable by data category and performed in bounded, cancellable batches. Deletion distinguishes individual records, project metadata, imported terminal records, sessions, and all application data. Destructive database work should be transactional and clearly confirmed.

Local diagnostic logs default to a 1 MiB active file, five archives, and seven-day retention. Expired, oversized, and excess fixed-name files are pruned at logger startup and rotation. The logging API exposes clearing, but user-facing consent, retention controls, and deletion remain part of the planned Privacy & Data task.

## Export

JSON, CSV, and safe Markdown exports are planned. Exports are re-redacted and CSV fields are protected against formula injection. Diagnostics exclude the full database by default and require an explicit preview.

## Current implementation

The desktop application does not collect, store, export, or transmit user data. It initializes a local logger but emits no event before onboarding consent, so the current runtime creates only an empty application-data-scoped log file and may remove expired, oversized, or excess fixed-name archives. The standalone persistence crate can create the initial non-sensitive owner schema for tests and future composition, but it is not initialized by the desktop application.
