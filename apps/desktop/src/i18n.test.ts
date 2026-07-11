import { describe, expect, it } from "vitest";

import { resolveLocale, translate } from "./i18n";

describe("localization foundation", () => {
  it("resolves Russian language variants", () => {
    expect(resolveLocale("ru-RU")).toBe("ru");
  });

  it("falls back to English for unsupported languages", () => {
    expect(resolveLocale("de-DE")).toBe("en");
  });

  it("provides a localized foundation status", () => {
    expect(translate("en", "foundationTitle")).toBe(
      "The secure foundation is ready",
    );
    expect(translate("ru", "foundationTitle")).toBe("Безопасная основа готова");
  });
});
