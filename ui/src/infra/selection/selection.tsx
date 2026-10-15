import { useEffect, useSyncExternalStore } from "react";
import type { PropertyDescriptor } from "../../widgets/PropertyGrid";

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

// -- Property contexts -------------------------------------------------------

/**
 * What a window shows in Properties while it's the active one — a designer's
 * selected element, as Visual Studio's designers push theirs: the rows
 * (with their `onCommit`s back into the designer) and the selector line.
 * Published as selection kind `"properties"`, `source` = the owner.
 */
export interface PropertySet {
  /** Bold part of the selector line — the element's name. */
  objectName?: string;
  /** Rest of the selector line — what the element is. */
  objectType?: string;
  /** The description pane's text while no row is selected: what it is, how to use the window. */
  objectDescription?: string;
  /** Identifies the element — an edit in progress is dropped when it changes. */
  objectKey: string;
  properties: PropertyDescriptor[];
}

export const PROPERTIES_KIND = "properties";

const contexts = new Map<string, PropertySet>();

function publishContext(owner: string, set: PropertySet) {
  setSelection({ id: `${owner}#${set.objectKey}`, kind: PROPERTIES_KIND, data: set, source: owner });
}

/**
 * Makes `owner`'s property context the selection, as focusing that window
 * does; `false` when it has none (the caller shows something else — the
 * window's file).
 */
export function activatePropertyContext(owner: string): boolean {
  const set = contexts.get(owner);
  if (!set) return false;
  publishContext(owner, set);
  return true;
}

// Which owners have a context — readers wait for a window's to appear.
let owners = 0;
const ownerSubscribers = new Set<() => void>();
const ownersChanged = () => {
  owners++;
  ownerSubscribers.forEach((notify) => notify());
};

/** Whether `owner` has a property context — re-renders when one comes or goes. */
export function useHasPropertyContext(owner: string | null | undefined): boolean {
  useSyncExternalStore(
    (onChange) => {
      ownerSubscribers.add(onChange);
      return () => {
        ownerSubscribers.delete(onChange);
      };
    },
    () => owners,
  );
  return !!owner && contexts.has(owner);
}

/**
 * A window's property context for its lifetime (`owner`: what names the
 * window — a document's path). It's what Properties shows while the window
 * is active (`activatePropertyContext` when it gets focus); while shown, a
 * new `set` (another element picked, a value changed) shows at once.
 */
export function usePropertyContext(owner: string, set: PropertySet | null): void {
  useEffect(() => {
    const had = contexts.has(owner);
    if (!set) {
      contexts.delete(owner);
      if (had) ownersChanged();
      return;
    }
    contexts.set(owner, set);
    if (!had) ownersChanged();
    if (current?.kind === PROPERTIES_KIND && current.source === owner) publishContext(owner, set);
  });
  useEffect(
    () => () => {
      contexts.delete(owner);
      ownersChanged();
      if (current?.kind === PROPERTIES_KIND && current.source === owner) setSelection(null);
    },
    [owner],
  );
}
