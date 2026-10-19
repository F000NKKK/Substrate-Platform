import { useSyncExternalStore } from "react";

// Requests for Find and Replace — made from keybindings and the product's
// Edit menu, carried out by `FindHost` (which can reach the shell).
// Module-level so the menu needn't reach the host.

/** The Find and Replace window's two tabs. */
export const FIND_MODE = { find: "find", replace: "replace" } as const;
export type FindWindowMode = (typeof FIND_MODE)[keyof typeof FIND_MODE];

/** The Find and Replace tool window's id (the product declares the panel). */
export const FIND_IN_FILES_PANEL = "findInFiles";
/** The Find Results tool window's id (the product declares the panel). */
export const FIND_RESULTS_PANEL = "findResults";

/** The last Find in Files / Replace in Files request; `seq` changes on every one, so a second Ctrl+Shift+F re-seeds the field from the editor. */
export interface FindRequest {
  mode: FindWindowMode;
  seq: number;
}

let request: FindRequest | null = null;
let seq = 0;
const subscribers = new Set<() => void>();

export function useFindRequest(): FindRequest | null {
  return useSyncExternalStore(
    (notify) => {
      subscribers.add(notify);
      return () => subscribers.delete(notify);
    },
    () => request,
  );
}

/** Edit > Find and Replace > Find in Files (Ctrl+Shift+F) / Replace in Files (Ctrl+Shift+H). */
export function openFindInFiles(mode: FindWindowMode): void {
  request = { mode, seq: ++seq };
  subscribers.forEach((notify) => notify());
}

/**
 * What the Find and Replace window was showing — kept across its remounts
 * (a tool window remounts whenever it's moved to another dock) and reopenings.
 */
export const findDrafts = {
  pattern: "",
  mode: FIND_MODE.find as FindWindowMode,
  /** The request `seq` the window last acted on. */
  seq: 0,
};
