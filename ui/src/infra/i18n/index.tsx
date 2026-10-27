export { t, tn, pt, contextKey, T, useT, useLocale, resolvedLocale, setLocale, localeFollowsSystem, registerTranslations, availableLocales, formatShortcut, withShortcut } from "./i18n";
export type { LocaleInfo, Messages, PluralForms } from "./i18n";
export { errorText, isBackendMessage, messageText } from "./errorText";
export type { BackendMessage } from "./errorText";

export { RU } from "./locales/ru";

// The platform's own translations, by area, registered on import.
import "./locales/ru";
import "./locales/ru.editor";
import "./locales/ru.errorList";
import "./locales/ru.files";
import "./locales/ru.git";
import "./locales/ru.lsp";
import "./locales/ru.output";
import "./locales/ru.project";
import "./locales/ru.search";
import "./locales/ru.settings";
import "./locales/ru.shell";
import "./locales/ru.state";
import "./locales/ru.terminal";
import "./locales/ru.widgets";
