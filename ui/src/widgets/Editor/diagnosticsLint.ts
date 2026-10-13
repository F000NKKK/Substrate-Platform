import { setDiagnostics, type Diagnostic as LintDiagnostic } from "@codemirror/lint";
import type { EditorState } from "@codemirror/state";
import type { EditorView } from "@codemirror/view";
import type { Diagnostic } from "../../infra/diagnostics";

const LINT_SEVERITY = { error: "error", warning: "warning", message: "info" } as const;

/** Offset of 1-based `line`/`column`, clamped into the document. */
function offset(state: EditorState, line: number, column: number): number {
  const l = state.doc.line(Math.min(Math.max(1, line), state.doc.lines));
  return l.from + Math.min(Math.max(0, column - 1), l.length);
}

/** The squiggle's range: the given span, else the word at the column, else the line's text. */
function rangeOf(state: EditorState, d: Diagnostic): { from: number; to: number } {
  const from = offset(state, d.line!, d.column ?? 1);
  if (d.endLine !== undefined || d.endColumn !== undefined) {
    const to = offset(state, d.endLine ?? d.line!, d.endColumn ?? d.column ?? 1);
    if (to > from) return { from, to };
  }
  const line = state.doc.lineAt(from);
  if (d.column === undefined) {
    const indent = line.text.length - line.text.trimStart().length;
    return { from: line.from + indent, to: Math.max(line.to, line.from + indent) };
  }
  const word = state.wordAt(from);
  return word ? { from: word.from, to: word.to } : { from, to: Math.min(from + 1, line.to) };
}

/** Converts diagnostics with a line into CodeMirror's lint diagnostics, quick fixes as its actions. */
export function toLintDiagnostics(state: EditorState, diagnostics: readonly Diagnostic[]): LintDiagnostic[] {
  return diagnostics
    .filter((d) => d.line !== undefined)
    .map((d) => ({
      ...rangeOf(state, d),
      severity: LINT_SEVERITY[d.severity],
      message: d.message,
      source: d.code,
      actions: d.fixes?.map((fix) => ({ name: fix.label, apply: () => fix.apply() })),
    }));
}

/** Replaces the view's squiggles. */
export function syncDiagnostics(view: EditorView, diagnostics: readonly Diagnostic[]): void {
  view.dispatch(setDiagnostics(view.state, toLintDiagnostics(view.state, diagnostics)));
}
