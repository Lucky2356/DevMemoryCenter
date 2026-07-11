import { en } from "./locales/en";
import { ru } from "./locales/ru";
import type { Messages, PluralCategory } from "./locales/types";
import type { NavigationId, ScreenStateKind } from "./models/app-shell";

export const supportedLocales = ["en", "ru"] as const;

export type SupportedLocale = (typeof supportedLocales)[number];

const resources: Readonly<Record<SupportedLocale, Messages>> = { en, ru };

const languageTags: Readonly<Record<SupportedLocale, string>> = {
  en: "en-US",
  ru: "ru-RU",
};

const pluralCategories: readonly PluralCategory[] = [
  "one",
  "few",
  "many",
  "other",
];

export type MessageKey = Exclude<
  keyof Messages,
  "durationMinutes" | "navigation" | "screenDescriptions" | "screenStates"
>;

function tryResolveLocale(language: string): SupportedLocale | undefined {
  const normalized = language.trim().toLowerCase();

  if (/^ru(?:[-_.@]|$)/u.test(normalized)) {
    return "ru";
  }

  if (/^en(?:[-_.@]|$)/u.test(normalized)) {
    return "en";
  }

  return undefined;
}

export function resolveLocale(language: string): SupportedLocale {
  return tryResolveLocale(language) ?? "en";
}

export function resolvePreferredLocale(
  languages: readonly string[],
): SupportedLocale {
  for (const language of languages) {
    const locale = tryResolveLocale(language);
    if (locale !== undefined) {
      return locale;
    }
  }

  return "en";
}

export function languageTag(locale: SupportedLocale): string {
  return languageTags[locale];
}

export function translate(locale: SupportedLocale, key: MessageKey): string {
  return resources[locale][key];
}

export function navigationLabel(
  locale: SupportedLocale,
  navigationId: NavigationId,
): string {
  return resources[locale].navigation[navigationId];
}

export function screenDescription(
  locale: SupportedLocale,
  navigationId: NavigationId,
): string {
  return resources[locale].screenDescriptions[navigationId];
}

export function screenStateMessage(
  locale: SupportedLocale,
  state: ScreenStateKind,
): Readonly<{ title: string; body: string }> {
  return resources[locale].screenStates[state];
}

export function formatNumber(locale: SupportedLocale, value: number): string {
  return new Intl.NumberFormat(languageTags[locale]).format(value);
}

export function formatDateTime(
  locale: SupportedLocale,
  value: Date | number,
  timeZone?: string,
): string {
  return new Intl.DateTimeFormat(languageTags[locale], {
    dateStyle: "medium",
    timeStyle: "short",
    timeZone,
  }).format(value);
}

export function formatDurationMinutes(
  locale: SupportedLocale,
  minutes: number,
): string {
  if (!Number.isFinite(minutes) || minutes < 0) {
    throw new RangeError(
      "Duration minutes must be a finite non-negative number.",
    );
  }

  const category = new Intl.PluralRules(languageTags[locale]).select(minutes);
  const supportedCategory: PluralCategory = pluralCategories.includes(
    category as PluralCategory,
  )
    ? (category as PluralCategory)
    : "other";
  const unit = resources[locale].durationMinutes[supportedCategory];

  return `${formatNumber(locale, minutes)} ${unit}`;
}
