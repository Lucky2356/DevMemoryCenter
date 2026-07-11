import type {
  NavigationId,
  ScreenStateKind,
  ThemePreference,
} from "../models/app-shell";
import type { ApplicationErrorMessageKey } from "../ipc/application-error";

export type PluralCategory = "one" | "few" | "many" | "other";

export interface StateMessage {
  readonly title: string;
  readonly body: string;
}

export interface Messages {
  readonly productName: string;
  readonly tagline: string;
  readonly foundationTitle: string;
  readonly foundationBody: string;
  readonly phaseLabel: string;
  readonly startupFailure: string;
  readonly skipToContent: string;
  readonly primaryNavigation: string;
  readonly applicationStatus: string;
  readonly themeLabel: string;
  readonly themes: Readonly<Record<ThemePreference, string>>;
  readonly errors: Readonly<Record<ApplicationErrorMessageKey, string>>;
  readonly navigation: Readonly<Record<NavigationId, string>>;
  readonly screenDescriptions: Readonly<Record<NavigationId, string>>;
  readonly screenStates: Readonly<Record<ScreenStateKind, StateMessage>>;
  readonly durationMinutes: Readonly<Record<PluralCategory, string>>;
}
