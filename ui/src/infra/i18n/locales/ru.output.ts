import { registerTranslations } from "../i18n";
import { RU } from "./ru";

/** The Output window and what reports to it, in Russian. */
registerTranslations(RU, {
  // Output
  "Show output from:": "Показать выходные данные из:",
  "Clear {source} Output": "Очистить вывод: {source}",
  "No messages.": "Нет сообщений.",
  "General": "Общие",
  "{what} failed: {error}": "{what}: ошибка — {error}",
  "No output yet.": "Вывода пока нет.",
});
