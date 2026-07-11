# Application Shell UI Foundation

The current application shell provides accessible navigation structure only. It does not implement project management, terminal import, sessions, search, timeline storage, favorites, privacy operations, or settings persistence.

## Navigation

The localized primary navigation contains the product sections defined by `SPEC.md`:

- Overview
- Projects
- Timeline
- Terminal Memory
- Sessions
- Favorites
- Privacy & Data
- Settings

Navigation uses native buttons with `aria-current="page"`, a labelled `<nav>`, and a polite live main region. A keyboard-visible skip link moves focus directly to `main-content`. Focus styles are visible, and the layout collapses to a two-column navigation grid at small window widths without horizontal overflow.

## Screen states

`ScreenStatePanel` is the shared, localized renderer for:

- `loading`: `aria-busy` with a polite status message;
- `empty`: an honest no-records explanation;
- `normal`: an available foundation state;
- `error`: an alert that explains no data changed and suggests a recovery action;
- `disabled`: `aria-disabled` with the security/privacy reason.

Current route states describe actual implementation status. Terminal Memory is disabled until privacy and redaction controls exist; sections with no persistence show empty states; Overview, Privacy & Data, and Settings only describe the safe foundation. Loading and error semantics are component-tested but are not fabricated as current runtime failures.

## Security and privacy

All visible strings come from typed English/Russian resources and React renders them as text. The shell has no remote assets, unsafe HTML, IPC calls, data sources, persistence, polling, timers, or event-listener lifecycle. Navigation state is bounded to eight compile-time route identifiers.

## Verification

Static component tests cover semantic navigation, localized labels, every state kind, loading announcements, error alerts, and the disabled Terminal Memory screen. Local browser QA verifies navigation interaction, skip-link focus, the Russian shell, and the 640-pixel responsive breakpoint.
