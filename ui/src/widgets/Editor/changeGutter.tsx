import { RangeSet, RangeSetBuilder, StateEffect, StateField, Text, type EditorState, type Extension } from "@codemirror/state";
import { EditorView, GutterMarker, gutter } from "@codemirror/view";
import { Chunk } from "@codemirror/merge";

/**
 * Visual Studio's change margin: a thin bar next to the line numbers
 * marking lines added, modified or deleted relative to a baseline text —
 * typically the file as last committed, which the host supplies with
 * `setChangeBaseline` (this knows nothing about where it comes from). With
 * no baseline, nothing is marked. Recomputed on every edit, so the bars
 * follow the text as it's typed.
 */

type LineChange = "added" | "modified" | "deleted";

class ChangeMarker extends GutterMarker {
  constructor(readonly change: LineChange) {
    super();
  }

  eq(other: ChangeMarker): boolean {
    return other.change === this.change;
  }

  toDOM(): Node {
    const el = document.createElement("div");
    el.className = `sp-change-marker sp-change-${this.change}`;
    return el;
  }
}

const MARKERS: Record<LineChange, ChangeMarker> = {
  added: new ChangeMarker("added"),
  modified: new ChangeMarker("modified"),
  deleted: new ChangeMarker("deleted"),
};

const setBaseline = StateEffect.define<Text | null>();

interface ChangeState {
  baseline: Text | null;
  markers: RangeSet<ChangeMarker>;
}

/** A quick, possibly imprecise diff is better than a stalled keystroke in a huge file. */
const DIFF_CONFIG = { scanLimit: 2000, timeout: 50 };

function computeMarkers(baseline: Text | null, state: EditorState): RangeSet<ChangeMarker> {
  if (!baseline) return RangeSet.empty;
  const doc = state.doc;
  const builder = new RangeSetBuilder<ChangeMarker>();
  for (const chunk of Chunk.build(baseline, doc, DIFF_CONFIG)) {
    if (chunk.fromB === chunk.toB) {
      // Lines only removed — marked on the line the removal sits before.
      const line = doc.lineAt(Math.min(chunk.fromB, doc.length));
      builder.add(line.from, line.from, MARKERS.deleted);
      continue;
    }
    const marker = chunk.fromA === chunk.toA ? MARKERS.added : MARKERS.modified;
    const last = doc.lineAt(Math.min(chunk.endB, doc.length)).number;
    for (let n = doc.lineAt(chunk.fromB).number; n <= last; n++) {
      const line = doc.line(n);
      builder.add(line.from, line.from, marker);
    }
  }
  return builder.finish();
}

const changeField = StateField.define<ChangeState>({
  create: () => ({ baseline: null, markers: RangeSet.empty }),
  update(value, tr) {
    let baseline = value.baseline;
    let changed = false;
    for (const effect of tr.effects) {
      if (effect.is(setBaseline)) {
        baseline = effect.value;
        changed = true;
      }
    }
    if (!changed && !(tr.docChanged && baseline)) return value;
    return { baseline, markers: computeMarkers(baseline, tr.state) };
  },
});

const theme = EditorView.baseTheme({
  ".sp-change-gutter .cm-gutterElement": { padding: "0 1px 0 0" },
  ".sp-change-marker": { width: "3px", height: "100%" },
  ".sp-change-added": { backgroundColor: "hsl(122 39% 49%)" },
  ".sp-change-modified": { backgroundColor: "hsl(207 82% 60%)" },
  // A small wedge at the top edge of the line the deleted lines were above.
  ".sp-change-deleted": {
    height: "0",
    width: "0",
    borderTop: "4px solid transparent",
    borderBottom: "4px solid transparent",
    borderLeft: "5px solid hsl(0 70% 58%)",
    transform: "translateY(-4px)",
  },
});

/** The change margin — part of every `TextEditor`; empty until `setChangeBaseline`. */
export const changeGutter: Extension = [
  changeField,
  gutter({
    class: "sp-change-gutter",
    markers: (view) => view.state.field(changeField).markers,
  }),
  theme,
];

/** Marks `view`'s lines changed relative to `baseline` from now on (`null` clears the margin). */
export function setChangeBaseline(view: EditorView, baseline: string | null): void {
  const current = view.state.field(changeField, false);
  if (current === undefined) return;
  if (baseline === null && current.baseline === null) return;
  if (baseline !== null && current.baseline !== null && current.baseline.toString() === baseline) return;
  view.dispatch({ effects: setBaseline.of(baseline === null ? null : Text.of(baseline.split(/\r\n?|\n/))) });
}
