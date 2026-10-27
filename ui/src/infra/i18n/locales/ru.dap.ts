import { registerTranslations } from "../i18n";
import { messages } from "../../host/host.generated";
import { RU } from "./ru";

const dap = messages.dap;

/** Debugging in Russian. */
registerTranslations(RU, {
  [dap.invalidList]: "Список адаптеров отладки некорректен: {error}",
  [dap.noAdapter]: "Нет адаптера отладки {adapter}.",
  [dap.cantStart]: "Не удалось запустить адаптер отладки {adapter}: {error}",
  [dap.failed]: "Ошибка адаптера отладки {adapter}: {error}",
  [dap.stopped]: "Адаптер отладки {adapter} остановлен.",
  [dap.timedOut]: "{adapter} не ответил на {command} вовремя.",
  [dap.error]: "{adapter} не смог выполнить {command}: {error}",
  "Start Debugging": "Начать отладку",
  "Stop Debugging": "Остановить отладку",
  Continue: "Продолжить",
  Pause: "Пауза",
  "Step Over": "Шаг с обходом",
  "Step Into": "Шаг с заходом",
  "Step Out": "Шаг с выходом",
  "Call Stack": "Стек вызовов",
  Variables: "Переменные",
  "The program is running.": "Программа выполняется.",
  "Not paused.": "Выполнение не приостановлено.",
});
