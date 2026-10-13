export { t, tn, pt, contextKey, T, useT, useLocale, resolvedLocale, setLocale, localeFollowsSystem, registerTranslations, availableLocales, formatShortcut } from "./i18n";
export type { LocaleInfo, Messages, PluralForms } from "./i18n";

// The platform's own translations, registered on import.
import "./locales/ru";
