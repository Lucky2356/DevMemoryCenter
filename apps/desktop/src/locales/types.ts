export type PluralCategory = "one" | "few" | "many" | "other";

export interface Messages {
  readonly productName: string;
  readonly tagline: string;
  readonly foundationTitle: string;
  readonly foundationBody: string;
  readonly phaseLabel: string;
  readonly startupFailure: string;
  readonly durationMinutes: Readonly<Record<PluralCategory, string>>;
}
