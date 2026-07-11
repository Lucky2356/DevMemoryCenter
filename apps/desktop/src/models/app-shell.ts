export const navigationIds = [
  "overview",
  "projects",
  "timeline",
  "terminalMemory",
  "sessions",
  "favorites",
  "privacy",
  "settings",
] as const;

export type NavigationId = (typeof navigationIds)[number];

export type ScreenStateKind =
  "loading" | "empty" | "normal" | "error" | "disabled";
