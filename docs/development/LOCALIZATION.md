# Localization

Dev Recall currently supports English and Russian without a runtime localization dependency or remote resource loading.

## Frontend resources

User-facing frontend strings live in:

```text
apps/desktop/src/locales/en.ts
apps/desktop/src/locales/ru.ts
```

Both resources must satisfy the shared `Messages` interface in `locales/types.ts`, so TypeScript rejects missing keys. Reusable components receive a resolved locale and request strings through `translate`; they must not embed user-facing fallback text.

At startup, the browser preference list is checked in order. English and Russian BCP 47 and common underscore variants are recognized, and unsupported or empty preferences fall back to English. The selected language tag is applied to the document and the document title is set through the same translation boundary.

Locale-aware number, date/time, plural, and duration formatting uses the built-in `Intl` APIs. Invalid durations are rejected rather than silently normalized.

## Rust startup resources

The Rust composition root can fail before the webview is available. Its safe startup message is embedded from:

```text
apps/desktop/src-tauri/locales/en/startup-error.txt
apps/desktop/src-tauri/locales/ru/startup-error.txt
```

Locale resolution checks `LC_ALL`, `LC_MESSAGES`, `LANGUAGE`, and `LANG` in order and supports colon-separated preferences. Unsupported or unavailable values fall back to English. Only the localized safe message is printed; the internal Tauri error is not exposed.

## Adding or changing a message

1. Add the typed key to `locales/types.ts`.
2. Add English and Russian values in the same change.
3. Render it through `translate` or the appropriate formatter.
4. Add or update tests for locale selection, plural behavior, or invalid input as relevant.
5. Review the text for sensitive-data interpolation before use.

Product and window identifiers such as `Dev Recall` remain the same in both languages. A future user-selectable preference may override system detection, but no preference persistence is implemented yet.
