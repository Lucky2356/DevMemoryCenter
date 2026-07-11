# Changelog

All notable changes will be documented here. The project has no release yet.

## Unreleased

- Added Phase 0 planning, security, privacy, architecture, and decision documentation.
- Added a minimal compileable application skeleton.
- Added read-only, SHA-pinned Windows and Linux continuous integration checks.
- Added typed English/Russian resources and locale-aware shell formatting.
- Added accessible localized application navigation and shared screen states.
- Added localized system, light, and dark theme controls with reduced-motion behavior.
- Added the first bounded health IPC contract with sanitized typed errors.
- Selected a minimal SQLite-only SQLx 0.8 dependency profile after a documented driver, MSRV, security, and packaging comparison.
- Added a bounded SQLite persistence crate, embedded immutable initial migration, private application-database path handling, and explicit shutdown.
- Added migration compatibility tests for empty and representative populated databases plus fail-closed applied-checksum validation.
- Added reproducible Rust dependency audit/deny policy and CI gates, remediated High XML parser advisories, and raised the security baseline to Rust 1.88.
- Added privacy-safe structured local logging with closed event fields, bounded rotation, retention, private Unix permissions, and explicit clearing.
- Added bounded fail-closed repository/history secret scanning, a full-history CI gate, and GitHub contribution/security metadata.
