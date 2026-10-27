import type { Extension } from "@codemirror/state";
import { ViewPlugin } from "@codemirror/view";
import { breakpointGutter, syncBreakpoints, syncExecutionLine } from "../widgets/Editor/gutters/breakpointGutter";
import type { DapSession } from "./dapSession";

/**
 * Debugging in the editor of `path`: a gutter to set and clear breakpoints
 * by clicking, the session's breakpoints of this file drawn in it, and the
 * line a paused program is at marked (and scrolled into view).
 */
export function dapExtension(session: DapSession, path: string): Extension {
  const sync = ViewPlugin.define((view) => {
    let breakpoints: ReadonlySet<number> | undefined;
    let paused: number | null = null;
    const apply = () => {
      const lines = session.breakpointsFor(path);
      if (lines !== breakpoints) {
        breakpoints = lines;
        syncBreakpoints(view, lines);
      }
      const line = session.pausedLineIn(path);
      if (line !== paused) {
        paused = line;
        syncExecutionLine(view, line);
      }
    };
    // A view can't be dispatched to while it is being made.
    queueMicrotask(apply);
    const off = session.subscribe(apply);
    return { destroy: off };
  });
  return [breakpointGutter((line) => void session.toggleBreakpoint(path, line)), sync];
}
