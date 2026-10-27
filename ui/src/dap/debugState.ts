/** Where a debug session is. */
export type DebugStatus =
  /** Not started, or stopped by the user. */
  | "idle"
  /** The adapter is starting and being given the breakpoints. */
  | "starting"
  /** The program runs. */
  | "running"
  /** The program is paused (a breakpoint, a step, an exception). */
  | "stopped"
  /** The program or the adapter ended; the output stays to read. */
  | "ended";

/** Why and where the program paused. */
export interface DebugStop {
  /** The adapter's reason: `breakpoint`, `step`, `exception`, `pause`… */
  reason: string;
  description?: string;
  threadId: number;
}

/** One frame of the call stack. */
export interface DebugFrame {
  id: number;
  name: string;
  /** The source file, if the frame has one. */
  path?: string;
  /** 1-based. */
  line: number;
  column: number;
}

/** A frame's scope (locals, arguments, registers…) — its variables load on request. */
export interface DebugScope {
  name: string;
  variablesReference: number;
  /** The adapter says reading it is slow: load it only when asked. */
  expensive: boolean;
}

/** A variable; `variablesReference` above 0 means it has children. */
export interface DebugVariable {
  name: string;
  value: string;
  type?: string;
  variablesReference: number;
}

/** One line the program or the adapter wrote. */
export interface DebugOutputLine {
  /** `stdout`, `stderr`, `console`… */
  category: string;
  text: string;
}

/** Everything a debug session shows; replaced, never changed, so a view can compare snapshots. */
export interface DebugState {
  status: DebugStatus;
  /** Breakpoint lines (1-based) by file; kept across runs. */
  breakpoints: ReadonlyMap<string, ReadonlySet<number>>;
  stop?: DebugStop;
  frames: readonly DebugFrame[];
  selectedFrameId?: number;
  scopes: readonly DebugScope[];
  /** Loaded children, by `variablesReference`. */
  variables: ReadonlyMap<number, readonly DebugVariable[]>;
  output: readonly DebugOutputLine[];
  exitCode?: number;
}

export const NO_LINES: ReadonlySet<number> = new Set();

/** How many output lines a session keeps. */
export const OUTPUT_LIMIT = 2000;

export function initialState(breakpoints: DebugState["breakpoints"] = new Map()): DebugState {
  return { status: "idle", breakpoints, frames: [], scopes: [], variables: new Map(), output: [] };
}

/** What a paused program is not showing any more: the stack, scopes and variables. */
export function running(state: DebugState): DebugState {
  return { ...state, status: "running", stop: undefined, frames: [], selectedFrameId: undefined, scopes: [], variables: new Map() };
}

/** The frame the user looks at — the selected one, else the top. */
export function selectedFrame(state: DebugState): DebugFrame | undefined {
  return state.frames.find((f) => f.id === state.selectedFrameId) ?? state.frames[0];
}

/** The line (1-based) the program is paused at in `path`, if it is. */
export function pausedLine(state: DebugState, path: string): number | null {
  if (state.status !== "stopped") return null;
  const frame = selectedFrame(state);
  return frame?.path === path ? frame.line : null;
}
