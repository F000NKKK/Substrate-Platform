import { registerTranslations } from "../i18n";
import { RU } from "./ru";

/** The shell in Russian: window and panel chrome, keys, the run bar, a failed panel. */
registerTranslations(RU, {
  // Errors in a panel
  "This panel failed: {error}": "Ошибка в панели: {error}",
  "Try Again": "Повторить",
  "Copy Error": "Копировать ошибку",
  // Window and panel chrome
  Minimize: "Свернуть",
  Maximize: "Развернуть",
  Restore: "Восстановить",
  Pin: "Закрепить",
  "Auto Hide": "Автоматически скрывать",
  Float: "Плавающее окно",
  "Show All Tabs": "Показать все вкладки",
  "Multiple Rows of Tabs": "Вкладки в несколько рядов",
  Unpin: "Открепить",
  "Unpin tab": "Открепить вкладку",
  "Close tab": "Закрыть вкладку",
  // Keys (formatShortcut, withShortcut)
  Space: "Пробел",
  Controls: "Управление",
  Enter: "Ввод",
  Backspace: "Backspace",
  Num: "Num",
  LMB: "ЛКМ",
  MMB: "СКМ",
  RMB: "ПКМ",
  Wheel: "Колесо",
  // Run bar
  "Run: {target}": "Запуск: {target}",
  "Select a target first": "Сначала выберите цель",
  "No targets": "Нет целей",
});
