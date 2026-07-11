import { resolveLocale, translate } from "./i18n";
import styles from "./App.module.css";

export function App() {
  const locale = resolveLocale(navigator.language);

  return (
    <main className={styles.shell}>
      <section className={styles.panel} aria-labelledby="foundation-title">
        <p className={styles.eyebrow}>{translate(locale, "productName")}</p>
        <h1 id="foundation-title">{translate(locale, "foundationTitle")}</h1>
        <p className={styles.tagline}>{translate(locale, "tagline")}</p>
        <p>{translate(locale, "foundationBody")}</p>
        <p className={styles.phase}>{translate(locale, "phaseLabel")}</p>
      </section>
    </main>
  );
}
