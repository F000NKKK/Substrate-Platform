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
});
