import { EditorState, type Extension } from "@codemirror/state";
import { t } from "../../../infra/i18n";

/**
 * CodeMirror's own UI text (the find/replace panel, Go to line, fold
 * markers, screen-reader announcements), keyed by its English phrase — the
 * same text the catalogs key it by. `$` is CodeMirror's placeholder.
 */
const PHRASES = [
  "Find",
  "Replace",
  "next",
  "previous",
  "all",
  "match case",
  "regexp",
  "by word",
  "replace",
  "replace all",
  "close",
  "current match",
  "on line",
  "replaced match on line $",
  "replaced $ matches",
  "Go to line",
  "go",
  "Folded lines",
  "Unfolded lines",
  "to",
  "folded code",
  "unfold",
  "Fold line",
  "Unfold line",
  "Control character",
  "Selection deleted",
  "Completions",
  "Diagnostics",
  "No diagnostics",
  // The diff view (@codemirror/merge).
  "$ unchanged lines",
];

/** CodeMirror's phrases in the active locale — rebuild (via a compartment) when the locale changes. */
export function editorPhrases(): Extension {
  return EditorState.phrases.of(Object.fromEntries(PHRASES.map((phrase) => [phrase, t(phrase)])));
}
