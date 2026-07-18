import type { ComponentType } from "react";

/**
 * The contract every file editor implements, whatever it renders underneath
 * (CodeMirror text, an image viewer, a grid) — the host (an `Editor` panel
 * managing open-file tabs) only ever needs this shape, never the concrete
 * editor kind.
 */
export interface EditorProps {
  /** Absolute path — an editor that cares what kind of file this is derives it from here. */
  path: string;
  /** The file's content, as last read from (or committed to) disk. */
  content: string;
  /** Called with the new content whenever the user changes it — the host owns deciding when/whether to persist it. */
  onChange: (next: string) => void;
  readOnly?: boolean;
  /** Lines (1-indexed) with a breakpoint set, for editors that render a breakpoint gutter (currently only `TextEditor`-based ones — others ignore this). Omit entirely for a plain editor with no gutter. */
  breakpoints?: ReadonlySet<number>;
  /** Called when the user toggles a breakpoint via the gutter — see `breakpoints`. */
  onToggleBreakpoint?: (line: number) => void;
}

export type EditorComponent = ComponentType<EditorProps>;
