import { registerTranslations } from "../i18n";
import { messages } from "../../host/host.generated";
import { RU } from "./ru";

const project = messages.project;
const workspace = messages.workspace;
const started = messages.process;

/** Projects, solutions, the open folders and what they start, in Russian (templates generated from Rust). */
registerTranslations(RU, {
  [project.noFormat]: "Приложение не знает формата, в котором записан {path}.",
  [project.dependencyCycle]: "{project} зависит от самого себя через свои зависимости сборки.",
  [project.unknownDependency]: "{project} зависит от {dependency}, которого нет в решении.",
  [workspace.nothingOpen]: "Нет открытого проекта.",
  [workspace.notOnDisk]: "{path} не найден на диске.",
  [workspace.outside]: "{path} находится вне открытых проектов.",
  [started.notALink]: "{url} не является веб-ссылкой или адресом электронной почты.",
  [started.cantStart]: "Не удалось запустить {program}: {error}",
});
