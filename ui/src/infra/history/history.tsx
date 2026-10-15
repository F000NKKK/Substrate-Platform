import { useSyncExternalStore } from "react";

/**
 * Undo for every editor of a document, kept by the platform — the one way
 * Undo/Redo work in a designer (Ctrl+Z, Ctrl+Y / Ctrl+Shift+Z, the Edit
 * menu, the toolbar):
 *
 * - each change a designer makes to the document's text is a step,
 *   recorded for it (`MultiViewEditor` wraps every non-text view's
 *   `onChange`); text views keep CodeMirror's history;
 * - what isn't text (a texture's pixels) a designer records as a step of
 *   its own (`pushDocumentStep`);
 * - steps made by one action (`documentAction`) are one step, and changes
 *   in quick succession (a drag, typing into a field) merge.
 */
export interface DocumentHistory {
  canUndo: boolean;
  canRedo: boolean;
  undo: () => void;
  redo: () => void;
}

/** One undoable step: how to take it back and do it again. */
export interface DocumentStep {
  undo: () => void;
  redo: () => void;
  /**
   * Steps with the same key in quick succession are one (typing into an
   * option, clicking a number's arrows): the first's undo, the last's redo.
   */
  merge?: string;
}

/** The Options' history: the platform's settings record their changes here (`undoable`). */
export const SETTINGS_HISTORY = "settings";

/** Changes closer together than this (ms) merge into one step. */
const MERGE_MS = 400;
const LIMIT = 200;

/** A text change: from `before` to `after`, written back through the document's writer. */
interface TextStep extends DocumentStep {
  text: { before: string; after: string };
}

interface Entry {
  past: DocumentStep[];
  future: DocumentStep[];
  /** When the last step was recorded — a burst merges. */
  at: number;
  /** Writes a text back into the document (the editor's `onChange`). */
  write: ((text: string) => void) | null;
  /** Steps of the action under way (`documentAction`), made one when it ends. */
  group: DocumentStep[] | null;
  /** Undoing or redoing: what that writes isn't a new step. */
  replaying: boolean;
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
    e = { past: [], future: [], at: 0, write: null, group: null, replaying: false };
    entries.set(path, e);
  }
  return e;
}

function push(e: Entry, step: DocumentStep) {
  if (e.group) {
    e.group.push(step);
    return;
  }
  const last = e.past[e.past.length - 1];
  if (step.merge && last?.merge === step.merge && Date.now() - e.at < MERGE_MS) {
    last.redo = step.redo;
    e.at = Date.now();
    e.future = [];
    changed();
    return;
  }
  e.past.push(step);
  if (e.past.length > LIMIT) e.past.shift();
  e.future = [];
  e.at = Date.now();
  changed();
}

/** Several steps as one, undone last first. */
function composite(steps: DocumentStep[]): DocumentStep {
  return {
    undo: () => [...steps].reverse().forEach((s) => s.undo()),
    redo: () => steps.forEach((s) => s.redo()),
  };
}

/** Records a change of `path`'s text from `before` to `after` (as a designer wrote it). */
export function recordDocumentChange(path: string, before: string, after: string): void {
  if (before === after) return;
  const e = entryOf(path);
  if (e.replaying) return;
  // Continuing the last change (dragging, typing): one step.
  const last = e.group ? e.group[e.group.length - 1] : e.past[e.past.length - 1];
  const lastText = last && "text" in last ? (last as TextStep).text : null;
  if (lastText && lastText.after === before && (e.group || Date.now() - e.at < MERGE_MS)) {
    lastText.after = after;
    e.at = Date.now();
    if (!e.group) e.future = [];
    return;
  }
  const text = { before, after };
  const step: TextStep = {
    text,
    undo: () => e.write?.(text.before),
    redo: () => e.write?.(text.after),
  };
  push(e, step);
}

/** Records a step that isn't the text (a texture's pixels) for `path`. */
export function pushDocumentStep(path: string, step: DocumentStep): void {
  const e = entryOf(path);
  if (!e.replaying) push(e, step);
}

/** Runs `action` on `path` as one undoable step, whatever it changes. */
export function documentAction<T>(path: string, action: () => T): T {
  const e = entryOf(path);
  if (e.group) return action();
  e.group = [];
  try {
    return action();
  } finally {
    const steps = e.group;
    e.group = null;
    if (steps.length) push(e, steps.length === 1 ? steps[0] : composite(steps));
  }
}

/** How the document's text is written back on Undo/Redo — the latest editor's `onChange`. */
export function setDocumentWriter(path: string, write: (text: string) => void): void {
  entryOf(path).write = write;
}

function step(path: string, back: boolean) {
  const e = entries.get(path);
  if (!e) return;
  const s = (back ? e.past : e.future).pop();
  if (!s) return;
  (back ? e.future : e.past).push(s);
  e.replaying = true;
  try {
    if (back) s.undo();
    else s.redo();
  } finally {
    e.replaying = false;
  }
  e.at = 0;
  changed();
}

/** Undo/Redo over `path`'s steps. */
export function documentHistory(path: string | null | undefined): DocumentHistory {
  const e = path ? entries.get(path) : undefined;
  return {
    canUndo: !!e?.past.length,
    canRedo: !!e?.future.length,
    undo: () => path && step(path, true),
    redo: () => path && step(path, false),
  };
}

function subscribe(notify: () => void) {
  subscribers.add(notify);
  return () => {
    subscribers.delete(notify);
  };
}

/** `documentHistory(path)`, re-rendering the caller as steps come and go. */
export function useDocumentHistory(path: string | null | undefined): DocumentHistory {
  useSyncExternalStore(subscribe, () => revision);
  return documentHistory(path);
}

// -- Scopes ------------------------------------------------------------------
//
// A history isn't only a document's: a window that changes things outside
// any document (Options, Project Properties) has one of its own, named by
// the scope it marks (`UndoScope`). Ctrl+Z goes to the history of the scope
// the focus is in, else to the active document's.

/**
 * Where Ctrl+Z / Ctrl+Y go from the focused element `el`: `"own"` — it
 * keeps its own undo (a field with text not committed yet, marked
 * `data-own-undo`, or a plain text field outside any scope); a scope's
 * history (`UndoScope`, even from its fields: they commit as they change);
 * or `null` — the active document.
 */
export function undoTarget(el: Element | null): "own" | { scope: string } | null {
  if (el?.closest("[data-own-undo]")) return "own";
  const scope = undoScopeOf(el);
  if (scope) return { scope };
  const typing = el instanceof HTMLTextAreaElement || (el instanceof HTMLInputElement && !["checkbox", "radio", "button", "range", "color"].includes(el.type));
  return typing ? "own" : null;
}

/** The undo scope `el` is in (`data-undo-scope` of its nearest marked ancestor), if any. */
export function undoScopeOf(el: Element | null): string | null {
  const marked = el?.closest("[data-undo-scope]");
  return marked?.getAttribute("data-undo-scope") || null;
}

/**
 * Sets something undoably in history `scope`: `apply(next)` now, and a step
 * that applies `before` again (and `next` on Redo) — for a setter outside
 * any document (an option, a project's manifest field). Nothing when equal.
 */
export function undoable<T>(scope: string, before: T, next: T, apply: (value: T) => void, merge?: string): void {
  apply(next);
  if (Object.is(before, next) || JSON.stringify(before) === JSON.stringify(next)) return;
  pushDocumentStep(scope, { undo: () => apply(before), redo: () => apply(next), merge });
}

/** Forgets `path`'s history (its tab closed, or reloaded from disk). */
export function forgetDocumentHistory(path: string): void {
  if (entries.delete(path)) changed();
}
