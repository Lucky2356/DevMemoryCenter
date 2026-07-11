import type { Messages } from "./types";

export const en = {
  productName: "Dev Recall",
  tagline: "Local memory for developers and system administrators",
  foundationTitle: "The secure foundation is ready",
  foundationBody:
    "This is the initial application shell. It does not collect, import, store, or transmit work data.",
  phaseLabel: "Current phase: application foundation",
  startupFailure: "Dev Recall could not start.",
  skipToContent: "Skip to main content",
  primaryNavigation: "Primary navigation",
  applicationStatus: "Application status",
  themeLabel: "Theme",
  themes: {
    system: "System",
    light: "Light",
    dark: "Dark",
  },
  navigation: {
    overview: "Overview",
    projects: "Projects",
    timeline: "Timeline",
    terminalMemory: "Terminal Memory",
    sessions: "Sessions",
    favorites: "Favorites",
    privacy: "Privacy & Data",
    settings: "Settings",
  },
  screenDescriptions: {
    overview: "A safe summary of the current Dev Recall foundation.",
    projects: "Local projects will appear here after explicit setup.",
    timeline: "Work events will appear here after a data source is enabled.",
    terminalMemory:
      "Terminal history collection is unavailable until privacy controls and redaction are implemented.",
    sessions: "Explicitly started work sessions will appear here.",
    favorites: "Saved redacted command templates will appear here.",
    privacy:
      "Collection is off. This foundation does not store or transmit work data.",
    settings: "Only implemented and safe preferences will appear here.",
  },
  screenStates: {
    loading: {
      title: "Loading",
      body: "The local view is being prepared. No data is sent over the network.",
    },
    empty: {
      title: "Nothing here yet",
      body: "No records are available for this section.",
    },
    normal: {
      title: "Foundation ready",
      body: "This section is available without collecting work data.",
    },
    error: {
      title: "This view could not be opened",
      body: "The local view may be temporarily unavailable. No data was changed. Close and reopen the section to try again.",
    },
    disabled: {
      title: "Not available yet",
      body: "This section stays off until its privacy and security controls are complete.",
    },
  },
  durationMinutes: {
    one: "minute",
    few: "minutes",
    many: "minutes",
    other: "minutes",
  },
} as const satisfies Messages;
