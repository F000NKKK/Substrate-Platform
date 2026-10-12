import { toggleComment } from "@codemirror/commands";
import type { ChangeSpec } from "@codemirror/state";
import type { EditorView } from "@codemirror/view";

// Visual Studio's Comment/Uncomment Selection rather than CodeMirror's own
// `toggleComment`: the line-comment token goes in bare (`//`, not `// `) at
// the selection's common indentation, and uncommenting takes exactly that
// token back off — so the two are inverses and never pile up spaces. Doc
// comments (`///`, `//!`) are code here, not commented-out lines: a block
// mixing them with code comments out as a whole and comes back unchanged,
// instead of `///` losing a slash on the way.

/** Whether a line (indentation already trimmed) is a commented-out line, as opposed to code or a doc comment. */
function isCommentedOut(text: string, token: string): boolean {
  if (!text.startsWith(token)) return false;
  if (token === "//") {
    const doc = (text.startsWith("///") && !text.startsWith("////")) || text.startsWith("//!");
    return !doc;
  }
  return true;
}

/**
 * Comments out every line the selection touches, or uncomments them if all
 * (non-blank) already are. Languages with only block comments (XML, ...)
 * fall back to CodeMirror's toggle. Returns `false` if there's nothing to do
 * — usable directly as a keymap `run`.
 */
export function toggleLineComments(view: EditorView): boolean {
  const { state } = view;
  const token = state.languageDataAt<{ line?: string }>("commentTokens", state.selection.main.head)[0]?.line;
  if (!token) return toggleComment(view);

  const lineNumbers = new Set<number>();
  for (const range of state.selection.ranges) {
    const first = state.doc.lineAt(range.from).number;
    let last = state.doc.lineAt(range.to).number;
    // A selection ending at a line's very start doesn't include that line.
    if (last > first && range.to === state.doc.line(last).from) last--;
    for (let n = first; n <= last; n++) lineNumbers.add(n);
  }
  const lines = [...lineNumbers]
    .sort((a, b) => a - b)
    .map((n) => state.doc.line(n))
    .filter((line) => line.text.trim() !== "");
  if (lines.length === 0) return false;

  const indentOf = (text: string) => text.length - text.trimStart().length;
  const uncomment = lines.every((line) => isCommentedOut(line.text.trimStart(), token));
  let changes: ChangeSpec[];
  if (uncomment) {
    changes = lines.map((line) => {
      const from = line.from + indentOf(line.text);
      return { from, to: from + token.length };
    });
  } else {
    const column = Math.min(...lines.map((line) => indentOf(line.text)));
    changes = lines.map((line) => ({ from: line.from + column, insert: token }));
  }
  view.dispatch({ changes, userEvent: uncomment ? "uncomment" : "comment", scrollIntoView: true });
  return true;
}
