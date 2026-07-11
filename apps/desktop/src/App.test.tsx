import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";

import { App } from "./App";
import { ScreenStatePanel } from "./ScreenStatePanel";
import type { ScreenStateKind } from "./models/app-shell";

describe("application shell", () => {
  it("renders localized semantic navigation", () => {
    const markup = renderToStaticMarkup(
      <App locale="ru" initialRoute="overview" />,
    );

    expect(markup).toContain('aria-label="Основная навигация"');
    expect(markup).toContain('aria-current="page"');
    expect(markup).toContain("Приватность и данные");
    expect(markup).toContain('href="#main-content"');
    expect(markup).toContain('aria-live="polite"');
  });

  it("renders an honest disabled section", () => {
    const markup = renderToStaticMarkup(
      <App locale="en" initialRoute="terminalMemory" />,
    );

    expect(markup).toContain("Terminal history collection is unavailable");
    expect(markup).toContain('aria-disabled="true"');
    expect(markup).toContain("Not available yet");
  });

  it.each<ScreenStateKind>(["loading", "empty", "normal", "error", "disabled"])(
    "renders the %s state",
    (state) => {
      const markup = renderToStaticMarkup(
        <ScreenStatePanel locale="en" state={state} />,
      );

      expect(markup).toContain("<section");
    },
  );

  it("announces loading and error states", () => {
    const loading = renderToStaticMarkup(
      <ScreenStatePanel locale="en" state="loading" />,
    );
    const error = renderToStaticMarkup(
      <ScreenStatePanel locale="en" state="error" />,
    );

    expect(loading).toContain('aria-busy="true"');
    expect(loading).toContain('role="status"');
    expect(error).toContain('role="alert"');
    expect(error).toContain("No data was changed");
  });
});
