import type { Messages } from "./types";

export const en = {
  productName: "Dev Recall",
  tagline: "Local memory for developers and system administrators",
  foundationTitle: "The secure foundation is ready",
  foundationBody:
    "This is the initial application shell. It does not collect, import, store, or transmit work data.",
  phaseLabel: "Current phase: application foundation",
  startupFailure: "Dev Recall could not start.",
  durationMinutes: {
    one: "minute",
    few: "minutes",
    many: "minutes",
    other: "minutes",
  },
} as const satisfies Messages;
