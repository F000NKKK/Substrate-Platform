import { registerTranslations } from "../i18n";
import { messages } from "../../host/host.generated";
import { RU } from "./ru";

const search = messages.search;

/** Find and Replace in Files in Russian (templates generated from Rust). */
registerTranslations(RU, {
  [search.empty]: "Не задан текст для поиска.",
  [search.badPattern]: "Недопустимое регулярное выражение: {error}",
  [search.badMask]: "Недопустимая маска файлов {mask}: {error}",
  [search.writeFailed]: "{path}: {error}",

  // Find and Replace window, Find Results
  "Find and Replace": "Найти и заменить",
  "Find in Files": "Найти в файлах",
  "Replace in Files": "Заменить в файлах",
  "Find Results": "Результаты поиска",
  "Find what:": "Найти:",
  "Search text": "Искомый текст",
  "Replace with:": "Заменить на:",
  "$1, $2, ${name} insert the captured groups.": "$1, $2, ${name} вставляют захваченные группы.",
  "Look in:": "Где искать:",
  "Current document": "Текущий документ",
  "All open documents": "Все открытые документы",
  "Entire project": "Весь проект",
  "File types:": "Типы файлов:",
  "Includes subfolders; skips .gitignore'd files, target/, node_modules/ and binaries.":
    "Включая вложенные папки; файлы из .gitignore, target/, node_modules/ и двоичные файлы пропускаются.",
  "Find All": "Найти все",
  "Replace All": "Заменить все",
  "Find all": "Найти все",
  "Whole word": "Слово целиком",
  "Regular expressions": "Регулярные выражения",
  'Replace all occurrences of "{find}" with "{replace}" in {where}?': "Заменить все вхождения \"{find}\" на \"{replace}\" ({where})?",
  "Files that aren't open are changed on disk; that can't be undone with Undo.":
    "Неоткрытые файлы изменяются на диске; это нельзя отменить командой \"Отменить\".",
  "Replace in Files couldn't write {error}": "Не удалось записать при замене в файлах: {error}",
  "Replace in Files failed: {error}": "Ошибка замены в файлах: {error}",
  "No occurrences were found.": "Вхождения не найдены.",
  "{count} occurrence replaced": { one: "Заменено {count} вхождение", few: "Заменено {count} вхождения", many: "Заменено {count} вхождений", other: "Заменено {count} вхождения" },
  "{count} file": { one: "{count} файл", few: "{count} файла", many: "{count} файлов", other: "{count} файла" },
  "{count} replaced": "заменено: {count}",
  "Total replaced: {total}  Matching files: {files}": "Всего заменено: {total}  Соответствующих файлов: {files}",
  "Matching lines: {lines}  Matching files: {files}  Total files searched: {searched}":
    "Соответствующих строк: {lines}  Соответствующих файлов: {files}  Всего просмотрено файлов: {searched}",
  "(stopped at the result limit)": "(достигнут предел числа результатов)",
  "(cancelled)": "(отменено)",
  "Searching…": "Поиск…",
  "Stop Search": "Остановить поиск",
  "Clear All": "Очистить все",
  "Use Find in Files ({keys}) to search the project.": "Используйте \"Найти в файлах\" ({keys}) для поиска по проекту.",
  "Include generated files": "Включать созданные файлы",
});
