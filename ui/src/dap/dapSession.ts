import type { UnlistenFn } from "@tauri-apps/api/event";
import { errorText } from "../infra/i18n";
import { reportError } from "../output/reportError";
import { firstThread, toFrames, toScopes, toVariables } from "./convert";
import type { DapEvent, DapEventMessage, DapHost } from "./dapHost";
import { initialState, NO_LINES, OUTPUT_LIMIT, pausedLine, running, type DebugState } from "./debugState";

/** How many frames a pause asks for. */
const STACK_DEPTH = 50;

/** A promise settled from outside. */
function deferred(): { promise: Promise<void>; resolve: () => void } {
  let resolve!: () => void;
  const promise = new Promise<void>((r) => (resolve = r));
  return { promise, resolve };
}

/**
 * One debug session for one workspace root and adapter, as the editors, the
 * toolbar and the panels share it: breakpoints per file (kept across runs
 * and sent to the adapter while one runs), launch, run control, and what the
 * paused program shows — call stack, scopes, variables. A view reads
 * {@link snapshot} and re-reads on {@link subscribe}.
 */
export class DapSession {
  private state: DebugState = initialState();
  private readonly listeners = new Set<() => void>();
  private unlisten?: Promise<UnlistenFn>;
  private initialized = deferred();
  private connected = false;
  /** Bumped when the program runs on or the session ends: answers about an older pause are dropped. */
  private generation = 0;

  constructor(
    readonly host: DapHost,
    readonly root: string,
    readonly adapter: string
  ) {}

  /** The current state — the same object until something changes. */
  readonly snapshot = (): DebugState => this.state;

  /** Calls `listener` after each change; returns how to stop. */
  readonly subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  };

  private set(next: DebugState): void {
    this.state = next;
    this.listeners.forEach((l) => l());
  }

  private fail(err: unknown): void {
    reportError(errorText(err));
  }

  private request<T>(command: string, args: unknown): Promise<T> {
    return this.host.request<T>(this.root, this.adapter, command, args);
  }

  // -- breakpoints --------------------------------------------------------------

  /** The lines (1-based) with a breakpoint in `path`; the same set until it changes. */
  breakpointsFor(path: string): ReadonlySet<number> {
    return this.state.breakpoints.get(path) ?? NO_LINES;
  }

  /** The line the paused program is at in `path`, if it is. */
  pausedLineIn(path: string): number | null {
    return pausedLine(this.state, path);
  }

  /** Sets or clears the breakpoint on `line` of `path`; while the adapter runs, it hears the file's new set. */
  async toggleBreakpoint(path: string, line: number): Promise<void> {
    const lines = new Set(this.breakpointsFor(path));
    if (!lines.delete(line)) lines.add(line);
    const breakpoints = new Map(this.state.breakpoints);
    if (lines.size) breakpoints.set(path, lines);
    else breakpoints.delete(path);
    this.set({ ...this.state, breakpoints });
    if (this.connected) await this.sendBreakpoints(path).catch((err) => this.fail(err));
  }

  private sendBreakpoints(path: string): Promise<unknown> {
    const lines = [...this.breakpointsFor(path)].sort((a, b) => a - b);
    return this.request("setBreakpoints", { source: { path }, breakpoints: lines.map((line) => ({ line })) });
  }

  // -- the session --------------------------------------------------------------

  /**
   * Starts the adapter and launches the program with `launch` (the adapter's
   * own arguments, on top of its `launch_defaults`): breakpoints are sent when
   * the adapter is ready, then the program runs. A failure is reported and
   * ends the session.
   */
  async start(launch: Record<string, unknown>): Promise<void> {
    if (this.state.status !== "idle" && this.state.status !== "ended") return;
    this.set({ ...initialState(this.state.breakpoints), status: "starting" });
    this.initialized = deferred();
    try {
      await this.listen();
      const capabilities = (await this.host.start(this.root, this.adapter)) as { supportsConfigurationDoneRequest?: boolean } | null;
      // Many adapters answer `launch` only after `configurationDone`: not awaited yet.
      const launched = this.request("launch", launch);
      const launchFailed = new Promise<never>((_, reject) => void launched.catch(reject));
      await Promise.race([this.initialized.promise, launchFailed]);
      this.connected = true;
      await Promise.all([...this.state.breakpoints.keys()].map((path) => this.sendBreakpoints(path)));
      if (capabilities?.supportsConfigurationDoneRequest !== false) await this.request("configurationDone", {});
      await launched;
      if (this.snapshot().status === "starting") this.set({ ...this.state, status: "running" });
    } catch (err) {
      this.fail(err);
      await this.stop();
    }
  }

  /** Ends the session: the adapter is stopped (and the program with it). Breakpoints stay. */
  async stop(): Promise<void> {
    this.generation++;
    this.connected = false;
    this.set(initialState(this.state.breakpoints));
    await this.unlisten?.then((off) => off()).catch(() => {});
    this.unlisten = undefined;
    await this.host.stop(this.root, this.adapter).catch(() => {});
  }

  // -- run control --------------------------------------------------------------

  /** Lets the program run on. */
  resume(): Promise<void> {
    return this.control("continue");
  }

  /** Runs to the next line of this function. */
  stepOver(): Promise<void> {
    return this.control("next");
  }

  /** Runs to the next line, entering calls. */
  stepIn(): Promise<void> {
    return this.control("stepIn");
  }

  /** Runs until this function returns. */
  stepOut(): Promise<void> {
    return this.control("stepOut");
  }

  /** Stops the running program where it is. */
  async pauseExecution(): Promise<void> {
    try {
      const threadId = firstThread(await this.request("threads", {}));
      if (threadId !== undefined) await this.request("pause", { threadId });
    } catch (err) {
      this.fail(err);
    }
  }

  private async control(command: string): Promise<void> {
    const threadId = this.state.stop?.threadId;
    if (this.state.status !== "stopped" || threadId === undefined) return;
    this.generation++;
    this.set(running(this.state));
    try {
      await this.request(command, { threadId });
    } catch (err) {
      this.fail(err);
    }
  }

  // -- what the paused program shows --------------------------------------------

  /** Shows `frameId`'s place and scopes. */
  async selectFrame(frameId: number): Promise<void> {
    const generation = this.generation;
    this.set({ ...this.state, selectedFrameId: frameId, scopes: [], variables: new Map() });
    try {
      const scopes = toScopes(await this.request("scopes", { frameId }));
      if (generation !== this.generation) return;
      this.set({ ...this.state, scopes });
      const first = scopes.find((s) => !s.expensive);
      if (first) await this.loadVariables(first.variablesReference);
    } catch (err) {
      this.fail(err);
    }
  }

  /** Loads the children under `reference` (once). */
  async loadVariables(reference: number): Promise<void> {
    if (this.state.variables.has(reference)) return;
    const generation = this.generation;
    try {
      const variables = toVariables(await this.request("variables", { variablesReference: reference }));
      if (generation !== this.generation) return;
      this.set({ ...this.state, variables: new Map(this.state.variables).set(reference, variables) });
    } catch (err) {
      this.fail(err);
    }
  }

  // -- the adapter's events -----------------------------------------------------

  private async listen(): Promise<void> {
    this.unlisten ??= this.host.onEvent((message) => this.hear(message));
    await this.unlisten;
  }

  private hear({ root, adapter, event }: DapEventMessage): void {
    if (root !== this.root || adapter !== this.adapter) return;
    void this.handle(event);
  }

  private async handle(event: DapEvent): Promise<void> {
    switch (event.event) {
      case "initialized":
        this.initialized.resolve();
        break;
      case "stopped":
        await this.stopped(event.body);
        break;
      case "continued":
        this.generation++;
        this.set(running(this.state));
        break;
      case "output":
        this.set({ ...this.state, output: [...this.state.output, { category: event.body.category, text: event.body.output }].slice(-OUTPUT_LIMIT) });
        break;
      case "exited":
        this.set({ ...this.state, exitCode: event.body.exitCode });
        break;
      case "terminated":
        await this.ended();
        break;
      default:
        break;
    }
  }

  private async stopped(body: Extract<DapEvent, { event: "stopped" }>["body"]): Promise<void> {
    const generation = ++this.generation;
    try {
      const threadId = body.threadId ?? firstThread(await this.request("threads", {}));
      if (threadId === undefined || generation !== this.generation) return;
      this.set({ ...this.state, status: "stopped", stop: { reason: body.reason, description: body.description, threadId } });
      const frames = toFrames(await this.request("stackTrace", { threadId, startFrame: 0, levels: STACK_DEPTH }));
      if (generation !== this.generation) return;
      this.set({ ...this.state, frames, selectedFrameId: frames[0]?.id });
      if (frames[0]) await this.selectFrame(frames[0].id);
    } catch (err) {
      this.fail(err);
    }
  }

  /** The program ended: the adapter is let go, the output stays. */
  private async ended(): Promise<void> {
    const output = this.state.output;
    const exitCode = this.state.exitCode;
    await this.stop();
    this.set({ ...this.state, status: "ended", output, exitCode });
  }
}

const hostIds = new WeakMap<DapHost, number>();
let nextHostId = 1;
const sessions = new Map<string, DapSession>();

function sessionKey(host: DapHost, root: string, adapter: string): string {
  if (!hostIds.has(host)) hostIds.set(host, nextHostId++);
  return `${hostIds.get(host)}\u0000${root}\u0000${adapter}`;
}

/** The session of `adapter` for `root` — the same one for every editor and panel asking. */
export function dapSession(host: DapHost, root: string, adapter: string): DapSession {
  const key = sessionKey(host, root, adapter);
  let session = sessions.get(key);
  if (!session) {
    session = new DapSession(host, root, adapter);
    sessions.set(key, session);
  }
  return session;
}

/** Ends every session and forgets them (and their breakpoints) — the workspace closed. */
export async function stopDapSessions(): Promise<void> {
  const all = [...sessions.values()];
  sessions.clear();
  await Promise.all(all.map((s) => s.stop().catch(() => {})));
}
