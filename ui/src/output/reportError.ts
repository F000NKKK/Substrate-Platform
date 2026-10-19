import { emit } from "@tauri-apps/api/event";
import { notify } from "../widgets/Notifications";
import { events } from "../infra/host/host.generated";

/** The Output window's tag for an error line (`defaultLineLevel`). */
const ERROR_TAG = "[error]";

/**
 * Surfaces a failure the user should know about — as an error line in
 * Output (General, unless it belongs to another source's `event`, so it's
 * still there after the toast fades) and as an error notification.
 * `message` is already in the user's language (`errorText`, `t`).
 */
export function reportError(message: string, event: string = events.ideOutput): void {
  emit(event, `${ERROR_TAG} ${message}`).catch(() => {});
  notify({ level: "error", message });
}
