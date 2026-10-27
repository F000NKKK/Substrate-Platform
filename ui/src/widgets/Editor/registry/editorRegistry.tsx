import { useSyncExternalStore } from "react";
import type { EditorView } from "@codemirror/view";

// Every mounted `TextEditor`'s live view, keyed by its `path` — the handle a
// host's toolbar/menu needs to act on "the active editor" (Undo, Toggle
// Comment, Find, Go to Line, ...) without each editor component exposing
// it through its own props. Module-level rather than a context, the same
// way `outputBuffer` is: the editor panels and the menu/toolbar acting on
// them live in unrelated parts of the dock tree.
const views = new Map<string, EditorView>();
const subscribers = new Set<() => void>();
/** Bumped on every registration change and every relevant view update, so `useEditorView` re-renders its caller even when the `EditorView` identity itself didn't change. */
let revision = 0;

function changed() {
  revision++;
  subscribers.forEach((notify) => notify());
}

function subscribe(notify: () => void) {
  subscribers.add(notify);
  return () => {
    subscribers.delete(notify);
  };
}

/** Called by `TextEditor` — returns the matching unregister. The last editor mounted for a path wins; unregistering only removes the entry if it still belongs to `view`. */
export function registerEditorView(path: string, view: EditorView): () => void {
  views.set(path, view);
  changed();
  return () => {
    if (views.get(path) !== view) return;
    views.delete(path);
    changed();
  };
}

/** Called by `TextEditor` after a view update that can change what an action on it would do (document, selection, configuration) — re-renders every `useEditorView` consumer. */
export function notifyEditorUpdate(): void {
  changed();
}

/** The live view editing `path`, if a text editor for it is currently mounted. */
export function getEditorView(path: string): EditorView | null {
  return views.get(path) ?? null;
}

/**
 * The live view editing `path` (`null` when none is mounted, or `path` is
 * `null`), re-rendering the caller whenever that view's document, selection
 * or configuration changes — so derived state like "can undo" stays current.
 */
export function useEditorView(path: string | null | undefined): EditorView | null {
  useSyncExternalStore(subscribe, () => revision);
  return path ? (views.get(path) ?? null) : null;
}
