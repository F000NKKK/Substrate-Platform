import { registerTranslations } from "../i18n";
import { messages } from "../../host/host.generated";
import { RU } from "./ru";

const lsp = messages.lsp;

/** Language servers in Russian. */
registerTranslations(RU, {
  [lsp.invalidList]: "Список языков некорректен: {error}",
  [lsp.noServer]: "Для языка {language} нет языкового сервера.",
  [lsp.cantStart]: "Не удалось запустить языковой сервер {server}: {error}",
  [lsp.failed]: "Ошибка языкового сервера {server}: {error}",
  [lsp.stopped]: "Языковой сервер {server} остановлен.",
  [lsp.timedOut]: "{server} не ответил на {method} вовремя.",
  [lsp.error]: "{server} не смог выполнить {method}: {error}",
});
