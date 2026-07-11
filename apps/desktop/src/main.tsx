import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { App } from "./App";
import { languageTag, resolvePreferredLocale, translate } from "./i18n";

const locale = resolvePreferredLocale(navigator.languages);
const rootElement = document.getElementById("root");

document.documentElement.lang = languageTag(locale);
document.title = translate(locale, "productName");

if (rootElement === null) {
  document.body.textContent = translate(locale, "startupFailure");
} else {
  createRoot(rootElement).render(
    <StrictMode>
      <App locale={locale} />
    </StrictMode>,
  );
}
