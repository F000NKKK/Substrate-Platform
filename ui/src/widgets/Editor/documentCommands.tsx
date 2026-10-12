import { useSyncExternalStore } from "react";

/**
 * What a non-text editor (a designer) can do through the host's Edit menu
 * and toolbar — the counterpart of `registerEditorView` for editors that
 * aren't a CodeMirror view. Each `can*` is read when the menu renders; call
 * `notifyDocumentCommands` when one of them may have changed.
 */
export interface DocumentCommands {
  canUndo: () => boolean;
  undo: () => void;
  canRedo: () => boolean;
  redo: () => void;
  /** Something is selected that Cut/Copy/Delete would act on. */
  hasSelection: () => boolean;
  cut: () => void;
  copy: () => void;
  canPaste: () => boolean;
  paste: () => void;
  selectAll: () => void;
  delete?: () => void;
}

const commands = new Map<string, DocumentCommands>();
const subscribers = new Set<() => void>();
let revision = 0;

/** Re-renders every `useDocumentCommands` consumer — after an edit or a selection change. */
export function notifyDocumentCommands(): void {
  revision++;
  subscribers.forEach((notify) => notify());
}

/** Called by a designer for the file it edits — returns the matching unregister. The last one registered for a path wins. */
export function registerDocumentCommands(path: string, handlers: DocumentCommands): () => void {
  commands.set(path, handlers);
  notifyDocumentCommands();
  return () => {
    if (commands.get(path) !== handlers) return;
    commands.delete(path);
    notifyDocumentCommands();
  };
}

function subscribe(notify: () => void) {
  subscribers.add(notify);
  return () => {
    subscribers.delete(notify);
  };
}

/** The commands of the designer editing `path`, if one is mounted — re-rendering the caller on `notifyDocumentCommands`. */
export function useDocumentCommands(path: string | null | undefined): DocumentCommands | null {
  useSyncExternalStore(subscribe, () => revision);
  return path ? (commands.get(path) ?? null) : null;
}
