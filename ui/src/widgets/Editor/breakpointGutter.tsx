import { RangeSet, StateEffect, StateField } from "@codemirror/state";
import { EditorView, gutter, GutterMarker } from "@codemirror/view";

/** Dispatched to replace the live breakpoint set without touching the document/undo history — see `TextEditor`'s second effect, which reacts to the `breakpoints` prop by dispatching this into the already-mounted view instead of recreating it. */
export const setBreakpoints = StateEffect.define<ReadonlySet<number>>();

const breakpointState = StateField.define<ReadonlySet<number>>({
  create: () => new Set(),
  update(value, tr) {
    for (const effect of tr.effects) {
      if (effect.is(setBreakpoints)) return effect.value;
    }
    return value;
  },
});

class BreakpointMarker extends GutterMarker {
  eq() {
    return true;
  }
  toDOM() {
    const dot = document.createElement("div");
    dot.className = "sp-breakpoint-dot";
    return dot;
  }
}
const marker = new BreakpointMarker();

/**
 * A clickable breakpoint gutter: one dot per line currently in the live
 * `breakpointState`, toggled via `onToggle(line)` (1-indexed, matching
 * CodeMirror's own `line.number`). Purely a rendering/interaction
 * mechanism — it doesn't know what a "breakpoint" means to the host
 * product (persistence, calling a debugger backend, ...), that's
 * `onToggle`'s job entirely.
 */
export function breakpointGutter(onToggle: (line: number) => void) {
  return [
    breakpointState,
    gutter({
      class: "sp-breakpoint-gutter",
      markers(view) {
        const set = view.state.field(breakpointState);
        const builder: { from: number; to: number; value: GutterMarker }[] = [];
        for (const lineNumber of set) {
          if (lineNumber < 1 || lineNumber > view.state.doc.lines) continue;
          const line = view.state.doc.line(lineNumber);
          builder.push({ from: line.from, to: line.from, value: marker });
        }
        builder.sort((a, b) => a.from - b.from);
        return RangeSet.of(
          builder.map((b) => marker.range(b.from)),
          true
        );
      },
      initialSpacer: () => marker,
      domEventHandlers: {
        mousedown(view, line, event) {
          // Without this, the browser's/CodeMirror's default mousedown
          // behavior (placing the selection at the nearest content
          // position and scrolling it into view) still runs alongside
          // ours, which is what was jumping the view back to the top —
          // returning `true` alone only stops *our* handler's own event
          // from bubbling, it doesn't cancel the default action.
          event.preventDefault();
          const lineNumber = view.state.doc.lineAt(line.from).number;
          onToggle(lineNumber);
          return true;
        },
      },
    }),
  ];
}

/** Imperatively syncs `view`'s live breakpoint set — call from a `useEffect` watching the host's breakpoint set, so it updates without recreating the `EditorView` (which would drop undo history/cursor position). */
export function syncBreakpoints(view: EditorView, lines: ReadonlySet<number>) {
  view.dispatch({ effects: setBreakpoints.of(lines) });
}
