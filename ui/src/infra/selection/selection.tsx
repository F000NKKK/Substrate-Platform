import { useSyncExternalStore } from "react";

/**
 * The one thing the user last selected anywhere in the shell — a node in a
 * project tree, the active editor tab, a widget on a design surface — the
 * way Visual Studio's Properties window follows "the current selection"
 * regardless of which tool window it came from. The platform only carries
 * it around; what `kind`s exist and what `data` they hold is up to the
 * product (and whatever reads it, e.g. a Properties panel feeding
 * `PropertyGrid`, dispatches on `kind`).
 */
export interface ShellSelection<T = unknown> {
  /** Identifies the selected object (e.g. its path) — stable across re-publishes of the same object. */
  id: string;
  /** What sort of object it is (`"file"`, `"folder"`, ...) — consumers dispatch on this. */
  kind: string;
  data: T;
  /** Which part of the UI published it (`"explorer"`, `"editor"`, ...) — informational. */
  source?: string;
}

// Module-level, like `notify`'s queue: the publishers (a tree, an editor
// tab) and the readers (a Properties panel) are unrelated dock panels with
// no props between them.
let current: ShellSelection | null = null;
const subscribers = new Set<() => void>();

/**
 * Makes `selection` the global selection (`null` clears it). Publishing
 * the same object again still notifies — a reader takes that as "show it
 * again", e.g. re-reading on-disk details that may have changed since.
 */
export function setSelection(selection: ShellSelection | null): void {
  current = selection;
  subscribers.forEach((notify) => notify());
}

/** The current selection, outside React. */
export function getCurrentSelection<T = unknown>(): ShellSelection<T> | null {
  return current as ShellSelection<T> | null;
}

function subscribe(onChange: () => void) {
  subscribers.add(onChange);
  return () => {
    subscribers.delete(onChange);
  };
}

/** The current global selection — re-renders whenever anything publishes one (see `setSelection`). */
export function useSelection<T = unknown>(): ShellSelection<T> | null {
  return useSyncExternalStore(subscribe, () => current) as ShellSelection<T> | null;
}
