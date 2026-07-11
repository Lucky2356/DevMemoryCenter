# Project State

## Current phase

Phase 1 — Application foundation.

## Last completed task

Added responsive, localized, keyboard-accessible application navigation and a shared renderer for loading, empty, normal, error, and disabled screen states.

## Work in progress

None. The navigation task is complete; the next Phase 1 task has not started.

## Completed

- Fully read `AGENTS.md` and `SPEC.md`.
- Inspected the initial two-file directory and confirmed it was not a Git repository.
- Verified local Windows toolchain prerequisites.
- Defined exact MVP boundaries and incremental phases.
- Added nine ADRs, including encryption/key storage.
- Added a localized React/Vite shell with strict TypeScript and CSS Modules.
- Added a Safe Rust Tauri composition root with no application IPC commands, no plugins, empty capabilities, strict CSP, no collection, and no persistence.
- Generated local placeholder application icons required by the Windows resource build.
- Added `.github/workflows/ci.yml` for fixed Ubuntu 24.04 and Windows 2025 runners with no publishing or artifact upload.
- Added a visible CI status badge and documented CI security/update policy.
- Moved frontend shell strings into typed English/Russian resource modules.
- Added ordered locale resolution, document language/title application, and `Intl` number/date/duration formatting with plural rules and invalid-duration rejection.
- Added embedded English/Russian Rust resources for safe startup failures without exposing internal Tauri errors.
- Added the eight `SPEC.md` navigation sections using bounded route identifiers and native keyboard controls.
- Added a skip link, visible focus, semantic landmarks, current-page state, polite content announcements, and responsive small-window layout.
- Added localized screen descriptions and honest shared loading, empty, normal, error, and disabled state semantics.

## Tests passed

- `npm run test`: 16 localization and application-shell component tests passed.
- `cargo test --workspace --all-features`: 5 Rust unit tests passed; doc tests passed.
- `npm ci --ignore-scripts`: locked dependency installation completed without lifecycle scripts.
- Local browser QA passed for Russian rendering, navigation changes, skip-link focus, disabled-state messaging, and the 640-pixel responsive layout.

## Checks not run

- `cargo audit`: not run because `cargo-audit` is not installed. Install deliberately, then run `cargo audit`; residual risk is an unverified Rust advisory database scan.
- `cargo deny check`: not run because `cargo-deny` is not installed and policy configuration is a Phase 1 task. Residual risk is unverified Rust license/advisory/duplicate policy.
- Linux build and Tauri prerequisites: not run from the Windows host. Verify in Phase 1 CI or a representative Linux environment.
- The GitHub-hosted workflow itself has not run because no push was authorized; workflow success on both runner images remains externally unverified.

## Security checks passed

- Manual threat/privacy review completed and security invariants documented.
- `npm audit --audit-level=high`: zero known vulnerabilities.
- Source review found no `unsafe`, shell plugin, generic IPC command, remote content, user-data persistence, or unsafe React HTML in the skeleton.
- CI workflow review confirmed top-level `contents: read`, non-persisted checkout credentials, full 40-character action SHAs, no cache, no secrets, no artifact upload, and no publish/release step.
- Localization review confirmed no remote locale loading, no persistence or transmission of language preferences, no unsafe HTML fallback, and no internal Tauri error exposure.
- UI review confirmed bounded navigation state, text-only rendering, no IPC/network/storage behavior, and accessible loading/error/disabled semantics.

## Performance measurements

- Frontend production build completed in approximately 0.2 seconds; output was 198.48 kB JavaScript (63.05 kB gzip) and 3.59 kB CSS (1.34 kB gzip) after the navigation shell was added.
- No runtime/idle measurement was taken; the skeleton is not an MVP and initial acceptance budgets remain in `docs/performance/PERFORMANCE_BUDGETS.md`.

## Known issues

- Global `cargo-tauri` is not installed; the verified repository-local npm CLI is used.
- Linux prerequisites and builds remain unverified from this Windows host.
- `cargo-audit` and `cargo-deny` are unavailable and were not installed automatically.
- The application currently has only the foundation status screen; all MVP features remain planned.
- CI remains unverified on GitHub until an explicitly authorized push triggers it.
- Locale selection currently follows system/browser preferences; a user-selected persisted override is not implemented yet.
- Navigation is in-memory only and intentionally does not preserve routes across restart or expose unfinished feature actions.

## Security findings

No concrete vulnerability found in the initial documentation-only repository.

## Decisions required

- Repository license selection.
- Clarify whether the GitHub publication restriction permits the `FUTURE_AI.md` product-boundary document required by `SPEC.md`.

Both decisions are documented in `NEEDS_USER_INPUT.md` and do not block local development. No push or publication is authorized.

## Next task

Add explicit light, dark, and system theme support with a user-selectable, reduced-motion-aware presentation boundary.

## Last stable commit

`HEAD` after the navigation commit (`feat: add accessible application shell navigation`).

## Commands to verify

```text
npm ci
npm run format:check
npm run lint
npm run typecheck
npm run test
npm run build
npm audit --audit-level=high
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo check --workspace --all-targets
npm run tauri -- info
npm run tauri -- build --config apps/desktop/src-tauri/tauri.conf.json --debug
```
