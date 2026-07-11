import { useState } from "react";

import {
  navigationLabel,
  screenDescription,
  translate,
  type SupportedLocale,
} from "./i18n";
import {
  navigationIds,
  type NavigationId,
  type ScreenStateKind,
} from "./models/app-shell";
import { ScreenStatePanel } from "./ScreenStatePanel";
import styles from "./App.module.css";

interface AppProps {
  readonly locale: SupportedLocale;
  readonly initialRoute?: NavigationId;
}

const screenStates: Readonly<Record<NavigationId, ScreenStateKind>> = {
  overview: "normal",
  projects: "empty",
  timeline: "empty",
  terminalMemory: "disabled",
  sessions: "empty",
  favorites: "empty",
  privacy: "normal",
  settings: "normal",
};

export function App({ locale, initialRoute = "overview" }: AppProps) {
  const [activeRoute, setActiveRoute] = useState<NavigationId>(initialRoute);

  return (
    <div className={styles.app}>
      <a className={styles.skipLink} href="#main-content">
        {translate(locale, "skipToContent")}
      </a>

      <aside className={styles.sidebar}>
        <header className={styles.brand}>
          <p className={styles.eyebrow}>{translate(locale, "productName")}</p>
          <p className={styles.tagline}>{translate(locale, "tagline")}</p>
        </header>

        <nav aria-label={translate(locale, "primaryNavigation")}>
          <ul className={styles.navigation}>
            {navigationIds.map((navigationId) => {
              const isActive = navigationId === activeRoute;

              return (
                <li key={navigationId}>
                  <button
                    className={styles.navigationButton}
                    type="button"
                    aria-current={isActive ? "page" : undefined}
                    aria-controls="main-content"
                    onClick={() => setActiveRoute(navigationId)}
                  >
                    {navigationLabel(locale, navigationId)}
                  </button>
                </li>
              );
            })}
          </ul>
        </nav>
      </aside>

      <main
        className={styles.main}
        id="main-content"
        tabIndex={-1}
        aria-live="polite"
      >
        <header className={styles.contentHeader}>
          <p className={styles.phase}>{translate(locale, "phaseLabel")}</p>
          <h1>{navigationLabel(locale, activeRoute)}</h1>
          <p className={styles.description}>
            {screenDescription(locale, activeRoute)}
          </p>
        </header>

        <section aria-label={translate(locale, "applicationStatus")}>
          <ScreenStatePanel locale={locale} state={screenStates[activeRoute]} />
        </section>
      </main>
    </div>
  );
}
