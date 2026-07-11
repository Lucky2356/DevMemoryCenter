import { describe, expect, it } from "vitest";

import {
  formatDateTime,
  formatDurationMinutes,
  formatNumber,
  languageTag,
  resolveLocale,
  resolvePreferredLocale,
  translate,
} from "./i18n";

describe("localization foundation", () => {
  it("resolves Russian language variants", () => {
    expect(resolveLocale("ru-RU")).toBe("ru");
    expect(resolveLocale("RU_ru.UTF-8")).toBe("ru");
  });

  it("falls back to English for unsupported languages", () => {
    expect(resolveLocale("de-DE")).toBe("en");
    expect(resolveLocale("")).toBe("en");
  });

  it("uses the first supported preferred language", () => {
    expect(resolvePreferredLocale(["de-DE", "ru-RU", "en-US"])).toBe("ru");
    expect(resolvePreferredLocale([])).toBe("en");
  });

  it("provides a localized foundation status", () => {
    expect(translate("en", "foundationTitle")).toBe(
      "The secure foundation is ready",
    );
    expect(translate("ru", "foundationTitle")).toBe("Безопасная основа готова");
  });

  it("provides document language tags", () => {
    expect(languageTag("en")).toBe("en-US");
    expect(languageTag("ru")).toBe("ru-RU");
  });

  it("formats numbers and dates for the selected locale", () => {
    expect(formatNumber("en", 1234.5)).toContain("1,234.5");
    expect(formatNumber("ru", 1234.5)).toContain("1 234,5");

    const instant = Date.UTC(2026, 6, 11, 9, 30);
    expect(formatDateTime("en", instant, "UTC")).not.toBe(
      formatDateTime("ru", instant, "UTC"),
    );
  });

  it("formats plural duration units", () => {
    expect(formatDurationMinutes("en", 2)).toBe("2 minutes");
    expect(formatDurationMinutes("ru", 1)).toBe("1 минута");
    expect(formatDurationMinutes("ru", 2)).toBe("2 минуты");
    expect(formatDurationMinutes("ru", 5)).toBe("5 минут");
  });

  it("rejects invalid duration values", () => {
    expect(() => formatDurationMinutes("en", -1)).toThrow(RangeError);
    expect(() => formatDurationMinutes("en", Number.NaN)).toThrow(RangeError);
  });
});
