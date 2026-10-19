import { registerTranslations } from "../i18n";
import { messages } from "../../host/host.generated";
import { RU } from "./ru";

/** Remembered state in Russian (templates generated from Rust). */
registerTranslations(RU, {
  [messages.state.failed]: "{path}: {error}",
});
