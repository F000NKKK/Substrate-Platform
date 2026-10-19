export { t, tn, pt, contextKey, T, useT, useLocale, resolvedLocale, setLocale, localeFollowsSystem, registerTranslations, availableLocales, formatShortcut, withShortcut } from "./i18n";
export type { LocaleInfo, Messages, PluralForms } from "./i18n";
export { errorText, isBackendMessage, messageText } from "./errorText";
export type { BackendMessage } from "./errorText";

// The platform's own translations, registered on import.
import "./locales/ru";
import "./locales/ru.git";
