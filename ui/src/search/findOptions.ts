import { t } from "../infra/i18n/i18n";
import type { SearchOptions } from "../widgets/SearchField/SearchField";
import { getEditorView } from "../widgets/Editor/editorRegistry";
import { LOOK_IN, type LookIn } from "./findStore";

/** Where the window's options and the replacement are remembered (global state). */
export const OPTIONS_KEY = "search.findInFiles";
export const REPLACEMENT_KEY = "search.replaceWith";

/** The Find and Replace window's options, remembered across sessions. */
export interface Remembered extends SearchOptions {
  lookIn: LookIn;
  fileMask: string;
  /** Entire project also searches the code the product generates (its search layout says where). */
  includeGenerated?: boolean;
}

export const DEFAULTS: Remembered = { matchCase: false, wholeWord: false, regex: false, lookIn: LOOK_IN.project, fileMask: "" };

export function isRemembered(value: unknown): boolean {
  const v = value as Remembered | null;
  return !!v && typeof v === "object" && typeof v.matchCase === "boolean" && typeof v.fileMask === "string" && Object.values<string>(LOOK_IN).includes(v.lookIn);
}

export function isString(value: unknown): boolean {
  return typeof value === "string";
}

/** The editor's selection (one line of it), else the word at the cursor — what Find what starts with. */
export function seedText(path: string | undefined): string {
  const view = path ? getEditorView(path) : null;
  if (!view) return "";
  const { state } = view;
  const range = state.selection.main;
  if (!range.empty) {
    const text = state.sliceDoc(range.from, range.to);
    return text.includes("\n") ? "" : text;
  }
  const word = state.wordAt(range.head);
  return word ? state.sliceDoc(word.from, word.to) : "";
}

/** A document's current text: its live editor's, else the tab's last known content. */
export function bufferText(tab: { path: string; content: string }): string {
  return getEditorView(tab.path)?.state.doc.toString() ?? tab.content;
}

export function lookInLabel(lookIn: LookIn): string {
  return lookIn === LOOK_IN.current ? t("Current document") : lookIn === LOOK_IN.open ? t("All open documents") : t("Entire project");
}
