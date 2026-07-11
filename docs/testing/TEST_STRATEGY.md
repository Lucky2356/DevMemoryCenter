# Test Strategy

## Test pyramid

- Rust unit tests: domain invariants, state transitions, limits, redaction, parsers, safe error mapping.
- Property/fuzz tests: redaction and history parsers with bounded generators and malicious fixtures.
- Integration tests: migrations, repositories, transactions, import checkpoint/restart, cancellation, corruption recovery, path/symlink behavior.
- Frontend component tests: loading/empty/normal/error/disabled states, localization, keyboard navigation, focus, XSS regression, large lists.
- Desktop end-to-end tests: onboarding, project lifecycle, import preview/confirm, search, sessions, deletion/export, restart and migration recovery.

## Mandatory security suites

Fixtures use obviously fake data and cover HTML/scripts, shell metacharacters, log controls, bidi text, malformed UTF-8/JSON/history, traversal/device/UNC paths, symlinks, long input, regex attacks, fake JWT/credentials/database URLs, malicious CSV formulas, IPC size/type violations, and least-privilege capabilities.

## Platform matrix

- Windows 10 and 11 packaged/runtime checks.
- Representative supported Linux desktop distribution matrix defined before release.
- Windows PowerShell and Bash source fixtures first; WSL and additional shells follow as dedicated tasks.

## Quality gates

Every behavior change requires relevant tests plus format, lint, typecheck, test, build, dependency/security checks, diff review, and state-document updates. A skipped check records command, reason, and residual risk.

## Long-running and recovery tests

Before release: eight-hour idle/active soak; repeated project open/close; 100 session cycles; large and repeated imports; rapid navigation; shutdown during import; restart after simulated crash; migration from empty, previous, and representative populated databases.
