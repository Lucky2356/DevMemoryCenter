import type { Messages } from "./types";

export const ru = {
  productName: "Dev Recall",
  tagline: "Локальная память для разработчиков и системных администраторов",
  foundationTitle: "Безопасная основа готова",
  foundationBody:
    "Это начальный каркас приложения. Он не собирает, не импортирует, не хранит и не передаёт рабочие данные.",
  phaseLabel: "Текущий этап: основа приложения",
  startupFailure: "Не удалось запустить Dev Recall.",
  durationMinutes: {
    one: "минута",
    few: "минуты",
    many: "минут",
    other: "минуты",
  },
} as const satisfies Messages;
