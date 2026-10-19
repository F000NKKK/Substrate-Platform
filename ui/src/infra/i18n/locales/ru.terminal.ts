import { registerTranslations } from "../i18n";
import { messages } from "../../host/host.generated";
import { RU } from "./ru";

const terminal = messages.terminal;

/** The Terminal window in Russian. */
registerTranslations(RU, {
  Terminal: "Терминал",
  "New Terminal": "Новый терминал",
  "Split Terminal": "Разделить терминал",
  "Kill Terminal": "Завершить терминал",
  "Close Terminal": "Закрыть терминал",
  "More Actions": "Другие действия",
  "Clear Terminal": "Очистить терминал",
  "Command Prompt": "Командная строка",
  [terminal.cantStart]: "Не удалось запустить оболочку: {error}",
  [terminal.failed]: "Ошибка терминала: {error}",
});
