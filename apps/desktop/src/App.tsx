import { useState } from "react";

import {
  navigationLabel,
  screenDescription,
  themeLabel,
  translate,
  type SupportedLocale,
} from "./i18n";
import {
  navigationIds,
  themePreferences,
  type NavigationId,
  type ScreenStateKind,
  type ThemePreference,
} from "./models/app-shell";
import { ScreenStatePanel } from "./ScreenStatePanel";
import styles from "./App.module.css";

interface AppProps {
  readonly locale: SupportedLocale;
  readonly initialRoute?: NavigationId;
  readonly initialTheme?: ThemePreference;
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

export function App({
  locale,
  initialRoute = "overview",
  initialTheme = "system",
}: AppProps) {
  const [activeRoute, setActiveRoute] = useState<NavigationId>(initialRoute);
  const [themePreference, setThemePreference] =
    useState<ThemePreference>(initialTheme);

  return (
    <div className={styles.app} data-theme={themePreference}>
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

        <fieldset className={styles.themePicker}>
          <legend className={styles.themeLegend}>
            {translate(locale, "themeLabel")}
          </legend>
          <div className={styles.themeOptions}>
            {themePreferences.map((preference) => (
              <label className={styles.themeOption} key={preference}>
                <input
                  type="radio"
                  name="theme"
                  value={preference}
                  checked={themePreference === preference}
                  onChange={() => setThemePreference(preference)}
                />
                <span>{themeLabel(locale, preference)}</span>
              </label>
            ))}
          </div>
        </fieldset>
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
