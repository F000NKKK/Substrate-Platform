import { redo } from "@codemirror/commands";
import { EditorSelection, Prec, type Extension } from "@codemirror/state";
import { EditorView, keymap, runScopeHandlers, type Command } from "@codemirror/view";
import { eventKey } from "../../infra/keys";
import { goToLineDialog, openQuickFind } from "./findWidget";
import { editorCopy, editorCut, editorPaste } from "./editorCommands";
import { commentLines, toggleLineComments, uncommentLines } from "./lineComments";

// Visual Studio's editing keys on top of CodeMirror's defaults (which
// already cover Ctrl+Z/Y, Ctrl+A, Ctrl+F, Ctrl+D, Alt+Up/Down, ...), and
// every editor binding made independent of the keyboard layout.

/**
 * Re-runs a modified key press the layout typed as something other than
 * its Latin key (Ctrl+Я, Ctrl+.) as the US-layout key in the same place
 * (Ctrl+Z, Ctrl+/), so every binding works without switching to English.
 * CodeMirror's own fallback only covers non-Latin letters, and only where
 * the browser reports a usable `keyCode`.
 */
const layoutIndependentKeys = Prec.highest(
  EditorView.domEventHandlers({
    keydown(e, view) {
      if (!(e.ctrlKey || e.metaKey || e.altKey)) return false;
      const latin = eventKey(e);
      if (latin.length !== 1 || latin === e.key.toLowerCase()) return false;
      const key = e.shiftKey && /[a-z]/.test(latin) ? latin.toUpperCase() : latin;
      const synthetic = new KeyboardEvent("keydown", {
        key,
        code: e.code,
        keyCode: e.keyCode,
        ctrlKey: e.ctrlKey,
        metaKey: e.metaKey,
        altKey: e.altKey,
        shiftKey: e.shiftKey,
      });
      if (!runScopeHandlers(view, synthetic, "editor") && !clipboardKey(view, synthetic)) return false;
      e.preventDefault();
      return true;
    },
  }),
);

/**
 * Ctrl+C / Ctrl+X / Ctrl+V are the browser's own, not bindings, and the
 * webview only recognizes them by the character typed — so under another
 * layout they're done here, through the clipboard API.
 */
function clipboardKey(view: EditorView, e: KeyboardEvent): boolean {
  if (!(e.ctrlKey || e.metaKey) || e.altKey || e.shiftKey) return false;
  const report = (err: unknown) => console.warn("clipboard:", err);
  switch (e.key) {
    case "c":
      editorCopy(view).catch(report);
      return true;
    case "x":
      if (!view.state.readOnly) editorCut(view).catch(report);
      return true;
    case "v":
      if (!view.state.readOnly) editorPaste(view).catch(report);
      return true;
  }
  return false;
}

function lineRanges(view: EditorView) {
  const { state } = view;
  const seen = new Set<number>();
  return state.selection.ranges
    .map((r) => state.doc.lineAt(r.head))
    .filter((line) => !seen.has(line.number) && (seen.add(line.number), true));
}

/** Ctrl+C with nothing selected copies the whole line (with its line break), like VS. */
const copyLine: Command = (view) => {
  if (view.state.selection.ranges.some((r) => !r.empty)) return false;
  const text = lineRanges(view).map((line) => line.text + view.state.lineBreak).join("");
  navigator.clipboard.writeText(text).catch(() => {});
  return true;
};

/** Ctrl+X with nothing selected cuts the whole line, like VS. */
const cutLine: Command = (view) => {
  if (!copyLine(view)) return false;
  const { doc } = view.state;
  const changes = lineRanges(view).map((line) => ({ from: line.from, to: Math.min(line.to + 1, doc.length) }));
  view.dispatch({ changes, userEvent: "delete.cut", scrollIntoView: true });
  return true;
};

/** VS's Ctrl+U / Ctrl+Shift+U: the selection to lower / upper case. */
export function changeCase(upper: boolean): Command {
  return (view) => {
    const { state } = view;
    if (state.selection.ranges.every((r) => r.empty)) return false;
    view.dispatch(
      state.changeByRange((range) => {
        const text = state.sliceDoc(range.from, range.to);
        return {
          changes: { from: range.from, to: range.to, insert: upper ? text.toUpperCase() : text.toLowerCase() },
          range: EditorSelection.range(range.anchor, range.head),
        };
      }),
      { userEvent: "input" },
    );
    return true;
  };
}

/** Ctrl+Shift+Enter: a new line above the current one, cursor on it at the same indentation. */
const insertLineAbove: Command = (view) => {
  const { state } = view;
  view.dispatch(
    state.changeByRange((range) => {
      const line = state.doc.lineAt(range.head);
      const indent = /^\s*/.exec(line.text)![0];
      return {
        changes: { from: line.from, insert: indent + state.lineBreak },
        range: EditorSelection.cursor(line.from + indent.length),
      };
    }),
    { userEvent: "input", scrollIntoView: true },
  );
  return true;
};

const visualStudioKeys = Prec.high(
  keymap.of([
    // Ahead of basicSetup's own Mod-/ (CodeMirror's toggleComment, which adds `// `).
    { key: "Mod-/", run: toggleLineComments },
    { key: "Mod-k Mod-c", run: commentLines, preventDefault: true },
    { key: "Mod-k Mod-u", run: uncommentLines, preventDefault: true },
    { key: "Mod-Shift-z", run: redo, preventDefault: true },
    { key: "Mod-f", run: (view) => openQuickFind(view), preventDefault: true },
    { key: "Mod-h", run: (view) => openQuickFind(view, true), preventDefault: true },
    { key: "Mod-g", run: (view) => (void goToLineDialog(view), true), preventDefault: true },
    { key: "Mod-c", run: copyLine },
    { key: "Mod-x", run: cutLine },
    { key: "Mod-u", run: changeCase(false), preventDefault: true },
    { key: "Mod-Shift-u", run: changeCase(true), preventDefault: true },
    { key: "Mod-Shift-Enter", run: insertLineAbove, preventDefault: true },
  ]),
);

/** Visual Studio's editing keys, working in any keyboard layout. */
export const editorKeymap: Extension = [layoutIndependentKeys, visualStudioKeys];
