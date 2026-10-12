import { redo, redoDepth, selectAll, undo, undoDepth } from "@codemirror/commands";
import { EditorSelection } from "@codemirror/state";
import { EditorView } from "@codemirror/view";
import { changeCase } from "./editorKeymap";
import { goToLineDialog, openQuickFind } from "./findWidget";
import { commentLines, toggleLineComments, uncommentLines } from "./lineComments";

// Plain functions over a live `EditorView` (see `useEditorView`) — what a
// host's Edit menu/toolbar binds to. Every action refocuses the editor
// afterward, since clicking a menu or toolbar button took focus away from it.

export function editorCanUndo(view: EditorView): boolean {
  return undoDepth(view.state) > 0;
}

export function editorCanRedo(view: EditorView): boolean {
  return redoDepth(view.state) > 0;
}

export function editorUndo(view: EditorView): void {
  undo(view);
  view.focus();
}

export function editorRedo(view: EditorView): void {
  redo(view);
  view.focus();
}

/** Whether the language at the cursor defines comment syntax at all (plain text doesn't). */
export function editorCanToggleComment(view: EditorView): boolean {
  return view.state.languageDataAt("commentTokens", view.state.selection.main.head).length > 0;
}

export function editorToggleComment(view: EditorView): void {
  toggleLineComments(view);
  view.focus();
}

export function editorCommentSelection(view: EditorView): void {
  commentLines(view);
  view.focus();
}

export function editorUncommentSelection(view: EditorView): void {
  uncommentLines(view);
  view.focus();
}

export function editorSelectAll(view: EditorView): void {
  selectAll(view);
  view.focus();
}

/** Visual Studio's Go To Line dialog (Ctrl+G). */
export function editorGoToLine(view: EditorView): void {
  void goToLineDialog(view);
}

/** The selection to lower (`upper: false`) or upper case. */
export function editorChangeCase(view: EditorView, upper: boolean): void {
  changeCase(upper)(view);
  view.focus();
}

/** Opens Quick Find (Ctrl+F), seeded with the selection. */
export function editorFind(view: EditorView): void {
  openQuickFind(view);
}

/** Opens Quick Replace (Ctrl+H). */
export function editorReplace(view: EditorView): void {
  openQuickFind(view, true);
}

export function editorHasSelection(view: EditorView): boolean {
  return view.state.selection.ranges.some((r) => !r.empty);
}

function selectedText(view: EditorView): string {
  const { state } = view;
  return state.selection.ranges
    .filter((r) => !r.empty)
    .map((r) => state.sliceDoc(r.from, r.to))
    .join(state.lineBreak);
}

/** Copies the selection to the system clipboard. Rejects if the clipboard is unavailable. */
export async function editorCopy(view: EditorView): Promise<void> {
  if (!editorHasSelection(view)) return;
  await navigator.clipboard.writeText(selectedText(view));
  view.focus();
}

/** Copies the selection to the system clipboard, then deletes it — the deletion is skipped if copying failed (rejects instead), so nothing is lost. */
export async function editorCut(view: EditorView): Promise<void> {
  if (!editorHasSelection(view) || view.state.readOnly) return;
  await navigator.clipboard.writeText(selectedText(view));
  view.dispatch(view.state.replaceSelection(""), { userEvent: "delete.cut", scrollIntoView: true });
  view.focus();
}

/** Replaces the selection with the system clipboard's text. Rejects if the clipboard is unavailable. */
export async function editorPaste(view: EditorView): Promise<void> {
  if (view.state.readOnly) return;
  const text = await navigator.clipboard.readText();
  view.dispatch(view.state.replaceSelection(text), { userEvent: "input.paste", scrollIntoView: true });
  view.focus();
}

/**
 * Scrolls `line` (1-indexed) to the middle of the view and selects it — or,
 * with `column` (1-indexed), places the cursor there instead. Out-of-range
 * values are clamped to the document, so a stale location from a debugger
 * or a build log still lands somewhere sensible.
 */
export function revealLine(view: EditorView, line: number, column?: number): void {
  const { doc } = view.state;
  const target = doc.line(Math.min(Math.max(1, Math.floor(line)), doc.lines));
  const selection =
    column === undefined
      ? EditorSelection.range(target.from, target.to)
      : EditorSelection.cursor(Math.min(target.from + Math.max(0, Math.floor(column) - 1), target.to));
  view.dispatch({ selection, effects: EditorView.scrollIntoView(selection.head, { y: "center" }) });
  view.focus();
}

/** The smallest single replacement turning `from` into `to` (common prefix/suffix kept), so a whole-document rewrite doesn't throw the cursor and scroll position to one end. */
export function minimalChange(from: string, to: string): { from: number; to: number; insert: string } {
  let start = 0;
  const max = Math.min(from.length, to.length);
  while (start < max && from.charCodeAt(start) === to.charCodeAt(start)) start++;
  let endFrom = from.length;
  let endTo = to.length;
  while (endFrom > start && endTo > start && from.charCodeAt(endFrom - 1) === to.charCodeAt(endTo - 1)) {
    endFrom--;
    endTo--;
  }
  return { from: start, to: endFrom, insert: to.slice(start, endTo) };
}

/** Replaces the whole document with `text` as one undoable user edit (e.g. a formatter's output) — fires the editor's `onChange` like typing would. */
export function editorSetText(view: EditorView, text: string): void {
  const current = view.state.doc.toString();
  if (current === text) return;
  view.dispatch({ changes: minimalChange(current, text), userEvent: "input.format", scrollIntoView: true });
}
