import { screenStateMessage, type SupportedLocale } from "./i18n";
import type { ScreenStateKind } from "./models/app-shell";
import styles from "./ScreenStatePanel.module.css";

interface ScreenStatePanelProps {
  readonly locale: SupportedLocale;
  readonly state: ScreenStateKind;
}

export function ScreenStatePanel({ locale, state }: ScreenStatePanelProps) {
  const message = screenStateMessage(locale, state);
  const isLoading = state === "loading";
  const isError = state === "error";
  const isDisabled = state === "disabled";

  return (
    <section
      className={styles.panel}
      data-state={state}
      aria-busy={isLoading || undefined}
      aria-disabled={isDisabled || undefined}
      role={isError ? "alert" : undefined}
    >
      <span className={styles.indicator} aria-hidden="true" />
      <div>
        <h2 className={styles.title}>{message.title}</h2>
        <p className={styles.body} role={isLoading ? "status" : undefined}>
          {message.body}
        </p>
      </div>
    </section>
  );
}
