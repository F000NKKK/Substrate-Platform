import { Label } from "../../widgets/Label";
import { availableLocales, localeFollowsSystem, resolvedLocale, setLocale, useT } from "../../infra/i18n";
import "./InternationalSettings.css";

/** Every setting label on this page — pass as the section's `keywords` so the Settings window's search finds the page by any of them. */
export const internationalSettingsKeywords: readonly string[] = ["Language", "Locale"];

const SYSTEM = "";

/**
 * The Settings window's "International Settings" section (Visual Studio's
 * Environment > International Settings) — the UI language, out of every
 * locale with registered translations, or the system's. Switching applies
 * live, in every window.
 */
export function InternationalSettings() {
  const t = useT();
  const locales = availableLocales();
  const shown = resolvedLocale();
  const shownName = locales.find((l) => l.id === shown)?.name ?? shown;

  return (
    <div className="sp-international-settings">
      <Label>{t("Language")}</Label>
      <select
        className="sp-international-settings-select"
        value={localeFollowsSystem() ? SYSTEM : shown}
        onChange={(e) => setLocale(e.target.value === SYSTEM ? null : e.target.value)}
      >
        <option value={SYSTEM}>{t("Same as system ({language})", { language: shownName })}</option>
        {locales.map((l) => (
          <option key={l.id} value={l.id}>
            {l.name}
          </option>
        ))}
      </select>
      <span className="sp-international-settings-hint">{t("Languages without a translation fall back to English.")}</span>
    </div>
  );
}
