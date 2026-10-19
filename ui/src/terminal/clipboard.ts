import { terminalApi } from "./terminalApi";

/** Copies a terminal's selection. */
export function copySelection(selection: string | undefined): void {
  if (selection) navigator.clipboard.writeText(selection).catch(() => {});
}

/** Pastes the clipboard into terminal `id` (typed into its shell). */
export function pasteInto(id: string): void {
  navigator.clipboard
    .readText()
    .then((text) => terminalApi.write(id, text))
    .catch(() => {});
}
