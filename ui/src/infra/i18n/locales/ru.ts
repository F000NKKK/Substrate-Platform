import { registerTranslations, type LocaleInfo } from "../i18n";

/** Russian — the one definition of the locale every Russian catalog registers into. */
export const RU: LocaleInfo = { id: "ru", name: "Русский" };

/**
 * The platform's strings in Russian, by area: the words every area shares
 * here, each area's own in `ru.<area>.ts`. Visual Studio's Russian
 * terminology where there is one. Products register their own strings
 * separately; a product entry for the same English text wins.
 */
registerTranslations(RU, {
  OK: "ОК",
  Cancel: "Отмена",
  Close: "Закрыть",
  Delete: "Удалить",
  Create: "Создать",
  Cut: "Вырезать",
  Copy: "Копировать",
  Paste: "Вставить",
  Dismiss: "Скрыть",
  Add: "Добавить",
  Remove: "Удалить",
});
