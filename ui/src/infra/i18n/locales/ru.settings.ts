import { registerTranslations } from "../i18n";
import { RU } from "./ru";

/** Settings in Russian: the window, International, Editor, Appearance, the color picker. */
registerTranslations(RU, {
  // Settings
  Options: "Параметры",
  "Search Settings": "Поиск параметров",
  'No settings match "{query}".': "Нет параметров, соответствующих «{query}».",
  // Settings: International
  Language: "Язык",
  Locale: "Язык и региональные параметры",
  "Same as system ({language})": "Как в системе ({language})",
  "Languages without a translation fall back to English.": "Для языков без перевода используется английский.",
  // Settings: Editor
  "Font Family": "Шрифт",
  "Font Size": "Размер шрифта",
  "Tab Size": "Размер табуляции",
  "Word Wrap": "Перенос по словам",
  'Default monospace — or e.g. "JetBrains Mono", monospace': "Моноширинный по умолчанию — или, например, \"JetBrains Mono\", monospace",
  "Reset to Defaults": "Восстановить значения по умолчанию",
  // Settings: Appearance
  "Accent Color": "Цвет акцента",
  "Current accent color": "Текущий цвет акцента",
  "Choose Color…": "Выбрать цвет…",
  "Editor Color Profile": "Цветовой профиль редактора",
  "Editor Colors": "Цвета редактора",
  "New Profile…": "Создать профиль…",
  "New Color Profile": "Новый цветовой профиль",
  "Profile name": "Имя профиля",
  "Delete Color Profile": "Удаление цветового профиля",
  'Delete profile "{name}"?': "Удалить профиль «{name}»?",
  "Reset to Profile Defaults": "Восстановить цвета профиля",
  "Export…": "Экспорт…",
  "Import…": "Импорт…",
  "That file isn't a valid editor color profile.": "Этот файл не является цветовым профилем редактора.",
  "VS Dark": "Темная VS",
  "Classic (CodeMirror default)": "Классическая (по умолчанию CodeMirror)",
  "{name} color": "Цвет: {name}",
  Background: "Фон",
  Foreground: "Текст",
  Selection: "Выделение",
  Cursor: "Курсор",
  Keyword: "Ключевое слово",
  String: "Строка",
  Comment: "Комментарий",
  Number: "Число",
  Type: "Тип",
  Function: "Функция",
  "Variable/Identifier": "Переменная/идентификатор",
  Operator: "Оператор",
  // Color picker
  "Select Color": "Выбор цвета",
  Lightness: "Яркость",
  Presets: "Предустановки",
  "Reset to default": "Восстановить по умолчанию",
});
