import { isolateHistory } from "@codemirror/commands";
import { Annotation, EditorSelection, StateEffect, StateField, type Extension } from "@codemirror/state";
import { Decoration, EditorView, type DecorationSet } from "@codemirror/view";
import { minimalChange } from "./editorCommands";

// Keeping a text view and another view of the same document in step — a
// designer over its markup (Visual Studio's XAML split view): the other side
// writes into the text as undoable edits, follows the cursor, and marks the
// range its selection corresponds to. Composed into a `TextEditor` through
// its `extensions`; the host never needs CodeMirror itself.

/** Who made a transaction through `editorApplyText`/`editorRevealRange`. */
const syncSource = Annotation.define<string>();

export interface EditorSyncUpdate {
  docChanged: boolean;
  selectionChanged: boolean;
  /** The `source` passed to `editorApplyText`/`editorRevealRange`, when the update came from one of them. */
  source: string | null;
  /** The user is typing or deleting — worth waiting for a pause before reacting. Undo, paste, a reload and other edits are `false`. */
  typing: boolean;
  /** The text view has keyboard focus. */
  focused: boolean;
  /** The whole text after the update. */
  text: string;
  /** The main cursor position. */
  cursor: number;
}

/** Calls `onUpdate` after every update that changed the text or the selection. */
export function editorSyncListener(onUpdate: (update: EditorSyncUpdate) => void): Extension {
  return EditorView.updateListener.of((update) => {
    if (!update.docChanged && !update.selectionSet) return;
    const source = update.transactions.map((tr) => tr.annotation(syncSource)).find((s) => s !== undefined) ?? null;
    onUpdate({
      docChanged: update.docChanged,
      selectionChanged: update.selectionSet,
      source,
      typing: update.transactions.some((tr) => tr.isUserEvent("input.type") || tr.isUserEvent("input.complete") || tr.isUserEvent("delete")),
      focused: update.view.hasFocus,
      text: update.docChanged ? update.state.doc.toString() : "",
      cursor: update.state.selection.main.head,
    });
  });
}

/**
 * Makes the document `text` — only the differing middle replaced, so the
 * cursor and scroll stay put — as its own undo step, tagged with `source`
 * (see `EditorSyncUpdate.source`). Fires the editor's `onChange`.
 */
export function editorApplyText(view: EditorView, text: string, source: string): void {
  const current = view.state.doc.toString();
  if (current === text) return;
  view.dispatch({
    changes: minimalChange(current, text),
    annotations: [syncSource.of(source), isolateHistory.of("full")],
    userEvent: "input.sync",
  });
}

/** Moves the cursor to `from` and scrolls `from`–`to` into view if it isn't — without taking focus. */
export function editorRevealRange(view: EditorView, from: number, to: number, source: string): void {
  const length = view.state.doc.length;
  const head = Math.min(Math.max(0, from), length);
  view.dispatch({
    selection: EditorSelection.cursor(head),
    effects: EditorView.scrollIntoView(EditorSelection.range(head, Math.min(Math.max(head, to), length)), { y: "nearest", yMargin: 24 }),
    annotations: syncSource.of(source),
  });
}

const setLinked = StateEffect.define<{ from: number; to: number } | null>();
const linkedMark = Decoration.mark({ class: "sp-editor-linked-range" });

const linkedField = StateField.define<DecorationSet>({
  create: () => Decoration.none,
  update(value, tr) {
    value = value.map(tr.changes);
    for (const effect of tr.effects) {
      if (!effect.is(setLinked)) continue;
      const range = effect.value;
      value = range && range.to > range.from ? Decoration.set([linkedMark.range(range.from, Math.min(range.to, tr.state.doc.length))]) : Decoration.none;
    }
    return value;
  },
  provide: (field) => EditorView.decorations.from(field),
});

/** The range `editorSetLinkedRange` marks — add to the view's extensions. */
export const linkedRangeHighlight: Extension = [
  linkedField,
  EditorView.baseTheme({
    ".sp-editor-linked-range": {
      backgroundColor: "color-mix(in srgb, var(--sp-accent) 18%, transparent)",
      outline: "1px solid color-mix(in srgb, var(--sp-accent) 45%, transparent)",
      borderRadius: "2px",
    },
  }),
];

/** Marks `from`–`to` (the element the other view has selected), or clears the mark. The mark follows edits. */
export function editorSetLinkedRange(view: EditorView, range: { from: number; to: number } | null): void {
  view.dispatch({ effects: setLinked.of(range) });
}
