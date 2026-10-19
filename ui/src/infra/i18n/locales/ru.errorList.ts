import { registerTranslations } from "../i18n";
import { RU } from "./ru";

/** The Error List in Russian. */
registerTranslations(RU, {
  "Error List": "Список ошибок",
  "Entire Solution": "Все решение",
  "Open Documents": "Открытые документы",
  "Current Document": "Текущий документ",
  "Build + Live Analysis": "Сборка + живой анализ",
  "Build Only": "Только сборка",
  "Live Analysis Only": "Только живой анализ",
  "Search Error List": "Поиск в списке ошибок",
  "{count} Error": { one: "{count} ошибка", few: "{count} ошибки", many: "{count} ошибок", other: "{count} ошибки" },
  "{count} Warning": { one: "{count} предупреждение", few: "{count} предупреждения", many: "{count} предупреждений", other: "{count} предупреждения" },
  "{count} Message": { one: "{count} сообщение", few: "{count} сообщения", many: "{count} сообщений", other: "{count} сообщения" },
  Error: "Ошибка",
  Warning: "Предупреждение",
  Message: "Сообщение",
  Severity: "Серьезность",
  Description: "Описание",
  Project: "Проект",
  File: "Файл",
  Line: "Строка",
  Column: "Столбец",
  Tool: "Средство",
  Scope: "Область",
  "Show items from": "Показать элементы из",
  "Copy Description": "Копировать описание",
  "Go To Location": "Перейти к расположению",
  "Show Error Help": "Показать справку по ошибке",
  "No problems found.": "Проблемы не обнаружены.",
  "No items match the current filters.": "Нет элементов, соответствующих текущим фильтрам.",
});
