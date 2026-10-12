import { RangeSet, StateEffect, StateField } from "@codemirror/state";
import { Decoration, EditorView, gutter, GutterMarker } from "@codemirror/view";

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

/** Dispatched to move (or, with `null`, clear) the debugger's current-execution line — see `syncExecutionLine`. */
export const setExecutionLine = StateEffect.define<number | null>();

const executionLineState = StateField.define<number | null>({
  create: () => null,
  update(value, tr) {
    for (const effect of tr.effects) {
      if (effect.is(setExecutionLine)) return effect.value;
    }
    return value;
  },
  provide: (field) =>
    EditorView.decorations.from(field, (line) => (view) => {
      if (line === null || line < 1 || line > view.state.doc.lines) return Decoration.none;
      return Decoration.set([executionLineDecoration.range(view.state.doc.line(line).from)]);
    }),
});

const executionLineDecoration = Decoration.line({ class: "sp-execution-line" });

class BreakpointMarker extends GutterMarker {
  constructor(
    readonly breakpoint: boolean,
    readonly executing: boolean
  ) {
    super();
  }
  eq(other: BreakpointMarker) {
    return other.breakpoint === this.breakpoint && other.executing === this.executing;
  }
  toDOM() {
    const cell = document.createElement("div");
    cell.className = "sp-breakpoint-cell";
    if (this.breakpoint) {
      const dot = document.createElement("div");
      dot.className = "sp-breakpoint-dot";
      cell.appendChild(dot);
    }
    if (this.executing) {
      const arrow = document.createElement("div");
      arrow.className = "sp-execution-arrow";
      cell.appendChild(arrow);
    }
    return cell;
  }
}
const marker = new BreakpointMarker(true, false);
const arrowMarker = new BreakpointMarker(false, true);
const bothMarker = new BreakpointMarker(true, true);

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
    executionLineState,
    gutter({
      class: "sp-breakpoint-gutter",
      markers(view) {
        const set = view.state.field(breakpointState);
        const executing = view.state.field(executionLineState);
        const lines = new Set(set);
        if (executing !== null) lines.add(executing);
        const builder: { from: number; value: GutterMarker }[] = [];
        for (const lineNumber of lines) {
          if (lineNumber < 1 || lineNumber > view.state.doc.lines) continue;
          const line = view.state.doc.line(lineNumber);
          const isBreakpoint = set.has(lineNumber);
          const isExecuting = lineNumber === executing;
          const value = isBreakpoint && isExecuting ? bothMarker : isExecuting ? arrowMarker : marker;
          builder.push({ from: line.from, value });
        }
        builder.sort((a, b) => a.from - b.from);
        return RangeSet.of(
          builder.map((b) => b.value.range(b.from)),
          true
        );
      },
      initialSpacer: () => marker,
      lineMarkerChange: (update) =>
        update.transactions.some((tr) => tr.effects.some((e) => e.is(setBreakpoints) || e.is(setExecutionLine))),
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

/** Imperatively moves the debugger's current-execution marker (arrow + highlighted line) in `view`, or clears it with `null`; scrolls it into view when set. */
export function syncExecutionLine(view: EditorView, line: number | null) {
  const inRange = line !== null && line >= 1 && line <= view.state.doc.lines;
  view.dispatch({
    effects: [
      setExecutionLine.of(inRange ? line : null),
      ...(inRange ? [EditorView.scrollIntoView(view.state.doc.line(line).from, { y: "center" })] : []),
    ],
  });
}
