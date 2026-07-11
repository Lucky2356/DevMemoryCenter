export const supportedLocales = ["en", "ru"] as const;

export type SupportedLocale = (typeof supportedLocales)[number];

const messages = {
  en: {
    productName: "Dev Recall",
    tagline: "Local memory for developers and system administrators",
    foundationTitle: "The secure foundation is ready",
    foundationBody:
      "This is the initial application shell. It does not collect, import, store, or transmit work data.",
    phaseLabel: "Current phase: research and architecture",
  },
  ru: {
    productName: "Dev Recall",
    tagline: "Локальная память для разработчиков и системных администраторов",
    foundationTitle: "Безопасная основа готова",
    foundationBody:
      "Это начальный каркас приложения. Он не собирает, не импортирует, не хранит и не передаёт рабочие данные.",
    phaseLabel: "Текущий этап: исследование и проектирование",
  },
} as const;

export type MessageKey = keyof (typeof messages)["en"];

export function resolveLocale(language: string): SupportedLocale {
  return language.toLowerCase().startsWith("ru") ? "ru" : "en";
}

export function translate(locale: SupportedLocale, key: MessageKey): string {
  return messages[locale][key];
}
