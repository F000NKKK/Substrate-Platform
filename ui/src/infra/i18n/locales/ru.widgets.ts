import { registerTranslations } from "../i18n";
import { RU } from "./ru";

/** The property grid and data grid in Russian. */
registerTranslations(RU, {
  // Property values
  True: "Да",
  False: "Нет",
  // Property grid
  "No object selected.": "Объект не выбран.",
  Categorized: "По категориям",
  Alphabetical: "По алфавиту",
  "Edit…": "Изменить…",
  "Search Properties": "Поиск свойств",
  "Show Description": "Показать описание",
  "Hide Description": "Скрыть описание",
  "Not a valid number.": "Недопустимое число.",
  "{object} properties": "Свойства: {object}",
  Misc: "Разное",
  // Data grid
  "Drag a column header here to group by that column": "Перетащите сюда заголовок столбца, чтобы сгруппировать по нему",
  "Stop grouping by {column}": "Отменить группировку по {column}",
  "Select all rows": "Выбрать все строки",
  "Select row {id}": "Выбрать строку {id}",
  "Filter…": "Фильтр…",
  "Filter {column}": "Фильтр: {column}",
  "No rows": "Нет строк",
  "{count} item": { one: "{count} элемент", few: "{count} элемента", many: "{count} элементов", other: "{count} элемента" },
  "Show hidden columns": "Показать скрытые столбцы",
  "No hidden columns": "Нет скрытых столбцов",
  "Clear grouping": "Отменить группировку",
  "Show group panel": "Показать панель группировки",
  "Hide group panel": "Скрыть панель группировки",
  "Hide column": "Скрыть столбец",
  "Group by this column": "Группировать по этому столбцу",
  "Stop grouping": "Отменить группировку",
  "Sort ascending": "Сортировать по возрастанию",
  "Sort descending": "Сортировать по убыванию",
  "Clear sort": "Отменить сортировку",
});
