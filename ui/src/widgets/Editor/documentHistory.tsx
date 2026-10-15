import { useSyncExternalStore } from "react";

/**
 * Undo for every editor of a document, kept by the platform: each change a
 * designer makes to the document's text is recorded here (`MultiViewEditor`
 * does it for every non-text view), so Undo/Redo work in any designer
 * without it keeping a history of its own. Text editors keep CodeMirror's.
 * A designer with finer steps of its own registers `DocumentCommands`,
 * which win over this.
 *
 * Changes in quick succession (a drag, typing into a field) are one step.
 */
export interface DocumentHistory {
  canUndo: boolean;
  canRedo: boolean;
  undo: () => void;
  redo: () => void;
}

/** Changes closer together than this (ms) merge into one step. */
const MERGE_MS = 400;
const LIMIT = 200;

interface Entry {
  past: string[];
  future: string[];
  /** The text after the last recorded change, and when — to merge a burst. */
  last: string | null;
  at: number;
  /** Writes a text back into the document (the editor's `onChange`). */
  apply: ((text: string) => void) | null;
}

const entries = new Map<string, Entry>();
const subscribers = new Set<() => void>();
let revision = 0;

function changed() {
  revision++;
  subscribers.forEach((notify) => notify());
}

function entryOf(path: string): Entry {
  let e = entries.get(path);
  if (!e) {
    e = { past: [], future: [], last: null, at: 0, apply: null };
    entries.set(path, e);
  }
  return e;
}

/** Records a change of `path` from `before` to `after` (as a designer wrote it). */
export function recordDocumentChange(path: string, before: string, after: string): void {
  if (before === after) return;
  const e = entryOf(path);
  const now = Date.now();
  // Continuing the last change (dragging, typing): one step.
  const merging = e.last === before && now - e.at < MERGE_MS && e.past.length > 0;
  if (!merging) {
    e.past.push(before);
    if (e.past.length > LIMIT) e.past.shift();
  }
  e.future = [];
  e.last = after;
  e.at = now;
  changed();
}

/** How the document is written back on Undo/Redo — the latest editor's `onChange`. */
export function setDocumentWriter(path: string, apply: (text: string) => void): void {
  entryOf(path).apply = apply;
}

function step(path: string, current: string, back: boolean) {
  const e = entries.get(path);
  if (!e?.apply) return;
  const from = back ? e.past : e.future;
  const text = from.pop();
  if (text === undefined) return;
  (back ? e.future : e.past).push(current);
  e.last = null;
  e.apply(text);
  changed();
}

/** Undo/Redo over `path`'s recorded changes; `current` is its text now. */
export function useDocumentHistory(path: string | null | undefined, current: string | undefined): DocumentHistory {
  useSyncExternalStore(
    (notify) => {
      subscribers.add(notify);
      return () => {
        subscribers.delete(notify);
      };
    },
    () => revision
  );
  const e = path ? entries.get(path) : undefined;
  return {
    canUndo: !!e?.past.length && !!e.apply,
    canRedo: !!e?.future.length && !!e.apply,
    undo: () => path && current !== undefined && step(path, current, true),
    redo: () => path && current !== undefined && step(path, current, false),
  };
}

/** Forgets `path`'s history (its tab closed, or reloaded from disk). */
export function forgetDocumentHistory(path: string): void {
  if (entries.delete(path)) changed();
}
