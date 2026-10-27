import { registerTranslations } from "../i18n";
import { messages } from "../../host/host.generated";
import { RU } from "./ru";

const fs = messages.fs;

/** File operations in Russian (templates generated from Rust). */
registerTranslations(RU, {
  [fs.notAFile]: "{path} не является файлом.",
  [fs.alreadyExists]: "{path} уже существует.",
  [fs.intoItself]: "Папку нельзя скопировать в саму себя.",
  [fs.failed]: "{path}: {error}",
});
