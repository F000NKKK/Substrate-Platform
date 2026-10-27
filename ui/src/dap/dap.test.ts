import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { act, createElement } from "react";
import { createRoot } from "react-dom/client";
import { EditorState } from "@codemirror/state";
import { EditorView } from "@codemirror/view";

vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn(), emit: vi.fn(() => Promise.resolve()) }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn(async () => undefined) }));
vi.mock("../output/reportError", () => ({ reportError: vi.fn() }));

import { invoke } from "@tauri-apps/api/core";
import { reportError } from "../output/reportError";
import { firstThread, toFrames, toScopes, toVariables } from "./convert";
import { adapterFor, tauriDapHost, type DapEvent, type DapEventMessage, type DapHost } from "./dapHost";
import { dapExtension } from "./dapExtension";
import { dapSession, stopDapSessions } from "./dapSession";
import { DebugToolbar } from "./DebugToolbar";

const ROOT = "/w";
const MAIN = "/w/src/main.rs";

/** A host that records every call and answers requests from `answers` (a function answers per call). */
function fakeHost(answers: Record<string, unknown | ((args: unknown) => unknown)> = {}) {
  const calls: string[] = [];
  const requests: { command: string; args: unknown }[] = [];
  let listener: ((m: DapEventMessage) => void) | undefined;
  const host: DapHost = {
    adapters: async () => [{ id: "lldb", languages: ["rust"] }],
    start: vi.fn(async () => {
      calls.push("start");
      return { supportsConfigurationDoneRequest: true };
    }),
    request: async <T,>(_root: string, _adapter: string, command: string, args: unknown) => {
      calls.push(command);
      requests.push({ command, args });
      const answer = answers[command];
      return (typeof answer === "function" ? answer(args) : answer ?? {}) as T;
    },
    stop: vi.fn(async () => {
      calls.push("stop");
    }),
    onEvent: async (handler) => {
      listener = handler;
      return () => {
        listener = undefined;
      };
    },
    onOutput: async () => () => {},
  };
  const say = (event: DapEvent, from: Partial<DapEventMessage> = {}) => listener?.({ root: ROOT, adapter: "lldb", event, ...from });
  return { host, calls, requests, say };
}

const settle = () => new Promise((resolve) => setTimeout(resolve, 0));

const STOP_ANSWERS = {
  stackTrace: { stackFrames: [{ id: 10, name: "main", source: { path: MAIN }, line: 3, column: 5 }, { id: 11, name: "start", line: 0, column: 0 }] },
  scopes: { scopes: [{ name: "Locals", variablesReference: 100 }, { name: "Registers", variablesReference: 200, expensive: true }] },
  variables: (args: unknown) =>
    (args as { variablesReference: number }).variablesReference === 100
      ? { variables: [{ name: "answer", value: "42", type: "int", variablesReference: 0 }, { name: "point", value: "{...}", variablesReference: 101 }] }
      : { variables: [{ name: "x", value: "1", variablesReference: 0 }] },
};

beforeEach(() => vi.mocked(reportError).mockClear());
afterEach(async () => {
  await stopDapSessions();
});

describe("convert", () => {
  it("reads frames, scopes and variables, defaulting what is left out", () => {
    expect(toFrames({ stackFrames: [{ id: 1, name: "f", source: { path: "/a.rs" }, line: 2, column: 3 }, { id: 2 }, "junk"] })).toEqual([
      { id: 1, name: "f", path: "/a.rs", line: 2, column: 3 },
      { id: 2, name: "", path: undefined, line: 0, column: 0 },
    ]);
    expect(toScopes({ scopes: [{ name: "Locals", variablesReference: 5, expensive: true }, { name: "Args" }] })).toEqual([
      { name: "Locals", variablesReference: 5, expensive: true },
      { name: "Args", variablesReference: 0, expensive: false },
    ]);
    expect(toVariables({ variables: [{ name: "a", value: "1", type: "int", variablesReference: 3 }] })).toEqual([{ name: "a", value: "1", type: "int", variablesReference: 3 }]);
    expect(toFrames(null)).toEqual([]);
    expect(toVariables({ variables: "no" })).toEqual([]);
    expect(firstThread({ threads: [{ id: 7, name: "main" }] })).toBe(7);
    expect(firstThread({})).toBeUndefined();
  });
});

describe("tauriDapHost", () => {
  it("calls the host's commands with the session's names", async () => {
    const host = tauriDapHost();
    await host.start(ROOT, "lldb");
    await host.request(ROOT, "lldb", "stackTrace", { threadId: 1 });
    await host.stop(ROOT, "lldb");
    expect(vi.mocked(invoke).mock.calls.slice(-3)).toEqual([
      ["plugin:substrate|dap_start", { root: ROOT, adapter: "lldb" }],
      ["plugin:substrate|dap_request", { root: ROOT, adapter: "lldb", command: "stackTrace", arguments: { threadId: 1 } }],
      ["plugin:substrate|dap_stop", { root: ROOT, adapter: "lldb" }],
    ]);
  });

  it("finds the adapter that debugs a language", () => {
    const adapters = [{ id: "lldb", languages: ["rust", "c"] }, { id: "python", languages: ["python"] }];
    expect(adapterFor(adapters, "c")?.id).toBe("lldb");
    expect(adapterFor(adapters, "go")).toBeUndefined();
  });
});

describe("breakpoints", () => {
  it("are set and cleared per file, kept as a new set each time", async () => {
    const { host, calls } = fakeHost();
    const session = dapSession(host, ROOT, "lldb");
    const none = session.breakpointsFor(MAIN);
    await session.toggleBreakpoint(MAIN, 3);
    await session.toggleBreakpoint(MAIN, 9);
    const two = session.breakpointsFor(MAIN);
    expect([...two]).toEqual([3, 9]);
    expect(two).not.toBe(none);
    await session.toggleBreakpoint(MAIN, 3);
    expect([...session.breakpointsFor(MAIN)]).toEqual([9]);
    await session.toggleBreakpoint(MAIN, 9);
    expect(session.breakpointsFor(MAIN).size).toBe(0);
    expect(session.snapshot().breakpoints.has(MAIN)).toBe(false);
    // No adapter running: nothing to tell.
    expect(calls).toEqual([]);
  });

  it("are sent to a running adapter, sorted, the file's whole set each time", async () => {
    const { host, requests, say } = fakeHost();
    const session = dapSession(host, ROOT, "lldb");
    void session.start({ program: "a" });
    await settle();
    say({ event: "initialized" });
    await vi.waitFor(() => expect(session.snapshot().status).toBe("running"));
    await session.toggleBreakpoint(MAIN, 9);
    await session.toggleBreakpoint(MAIN, 3);
    expect(requests.at(-1)).toEqual({ command: "setBreakpoints", args: { source: { path: MAIN }, breakpoints: [{ line: 3 }, { line: 9 }] } });
  });
});

describe("a session", () => {
  it("launches without waiting for the answer, gives the breakpoints when the adapter is ready, then lets the program run", async () => {
    const { host, calls, requests, say } = fakeHost();
    const session = dapSession(host, ROOT, "lldb");
    await session.toggleBreakpoint(MAIN, 3);
    const started = session.start({ program: "/w/a.out" });
    await settle();
    expect(session.snapshot().status).toBe("starting");
    expect(calls).toEqual(["start", "launch"]);
    say({ event: "initialized" });
    await started;
    expect(calls).toEqual(["start", "launch", "setBreakpoints", "configurationDone"]);
    expect(requests[0]).toEqual({ command: "launch", args: { program: "/w/a.out" } });
    expect(session.snapshot().status).toBe("running");
  });

  it("fills the stack, the scopes and the first scope's variables when the program stops", async () => {
    const { host, requests, say } = fakeHost(STOP_ANSWERS);
    const session = dapSession(host, ROOT, "lldb");
    say({ event: "stopped", body: { reason: "breakpoint", threadId: 4 } });
    // Not started: events still count only once listened to.
    expect(session.snapshot().status).toBe("idle");
    void session.start({});
    await settle();
    say({ event: "initialized" });
    await vi.waitFor(() => expect(session.snapshot().status).toBe("running"));
    say({ event: "stopped", body: { reason: "breakpoint", threadId: 4 } });
    await vi.waitFor(() => expect(session.snapshot().variables.has(100)).toBe(true));
    const state = session.snapshot();
    expect(state.status).toBe("stopped");
    expect(state.stop).toEqual({ reason: "breakpoint", description: undefined, threadId: 4 });
    expect(state.frames.map((f) => f.name)).toEqual(["main", "start"]);
    expect(state.selectedFrameId).toBe(10);
    expect(state.scopes.map((s) => s.name)).toEqual(["Locals", "Registers"]);
    expect(state.variables.get(100)?.map((v) => v.name)).toEqual(["answer", "point"]);
    // The expensive scope waits to be asked for.
    expect(state.variables.has(200)).toBe(false);
    expect(session.pausedLineIn(MAIN)).toBe(3);
    expect(session.pausedLineIn("/w/src/other.rs")).toBeNull();
    expect(requests.find((r) => r.command === "stackTrace")?.args).toEqual({ threadId: 4, startFrame: 0, levels: 50 });

    await session.loadVariables(101);
    expect(session.snapshot().variables.get(101)?.[0]?.name).toBe("x");
    await session.selectFrame(11);
    expect(session.snapshot().selectedFrameId).toBe(11);
    expect(session.pausedLineIn(MAIN)).toBeNull();
  });

  it("steps and continues on the stopped thread, showing it running at once", async () => {
    const { host, requests, say } = fakeHost(STOP_ANSWERS);
    const session = dapSession(host, ROOT, "lldb");
    void session.start({});
    await settle();
    say({ event: "initialized" });
    await vi.waitFor(() => expect(session.snapshot().status).toBe("running"));
    // Running: a step has nothing to step.
    await session.stepOver();
    expect(requests.some((r) => r.command === "next")).toBe(false);

    for (const [run, command] of [
      [() => session.stepOver(), "next"],
      [() => session.stepIn(), "stepIn"],
      [() => session.stepOut(), "stepOut"],
      [() => session.resume(), "continue"],
    ] as const) {
      say({ event: "stopped", body: { reason: "step", threadId: 4 } });
      await vi.waitFor(() => expect(session.snapshot().status).toBe("stopped"));
      await run();
      expect(requests.at(-1)).toEqual({ command, args: { threadId: 4 } });
      expect(session.snapshot().status).toBe("running");
      expect(session.snapshot().frames).toEqual([]);
    }
  });

  it("pauses a running program on its first thread", async () => {
    const { host, requests, say } = fakeHost({ threads: { threads: [{ id: 9, name: "main" }] } });
    const session = dapSession(host, ROOT, "lldb");
    void session.start({});
    await settle();
    say({ event: "initialized" });
    await vi.waitFor(() => expect(session.snapshot().status).toBe("running"));
    await session.pauseExecution();
    expect(requests.at(-1)).toEqual({ command: "pause", args: { threadId: 9 } });
  });

  it("keeps the output and the exit code when the program ends, and lets the adapter go", async () => {
    const { host, calls, say } = fakeHost();
    const session = dapSession(host, ROOT, "lldb");
    await session.toggleBreakpoint(MAIN, 3);
    void session.start({});
    await settle();
    say({ event: "initialized" });
    await vi.waitFor(() => expect(session.snapshot().status).toBe("running"));
    say({ event: "output", body: { category: "stdout", output: "42\n" } });
    say({ event: "exited", body: { exitCode: 3 } });
    say({ event: "terminated", body: {} });
    await vi.waitFor(() => expect(session.snapshot().status).toBe("ended"));
    const state = session.snapshot();
    expect(state.output).toEqual([{ category: "stdout", text: "42\n" }]);
    expect(state.exitCode).toBe(3);
    expect(calls.at(-1)).toBe("stop");
    // The breakpoints outlive the run.
    expect([...session.breakpointsFor(MAIN)]).toEqual([3]);
  });

  it("ignores another session's events", async () => {
    const { host, say } = fakeHost();
    const session = dapSession(host, ROOT, "lldb");
    void session.start({});
    await settle();
    say({ event: "initialized" }, { root: "/other" });
    say({ event: "initialized" }, { adapter: "python" });
    await settle();
    expect(session.snapshot().status).toBe("starting");
    say({ event: "initialized" });
    await vi.waitFor(() => expect(session.snapshot().status).toBe("running"));
  });

  it("reports a launch that fails and ends the session", async () => {
    const { host, calls } = fakeHost({
      launch: () => {
        throw { text: "{adapter} couldn't do {command}: {error}", args: { adapter: "lldb-dap", command: "launch", error: "no such file" } };
      },
    });
    const session = dapSession(host, ROOT, "lldb");
    await session.start({ program: "/nope" });
    expect(reportError).toHaveBeenCalledOnce();
    expect(vi.mocked(reportError).mock.calls[0]?.[0]).toContain("no such file");
    expect(session.snapshot().status).toBe("idle");
    expect(calls.at(-1)).toBe("stop");
  });

  it("is the same session for every view asking, and stops on request", async () => {
    const { host } = fakeHost();
    const session = dapSession(host, ROOT, "lldb");
    expect(dapSession(host, ROOT, "lldb")).toBe(session);
    expect(dapSession(host, "/other", "lldb")).not.toBe(session);
    void session.start({});
    await settle();
    await session.stop();
    expect(session.snapshot().status).toBe("idle");
    expect(host.stop).toHaveBeenCalledWith(ROOT, "lldb");
  });
});

describe("dapExtension", () => {
  function editor(session: ReturnType<typeof dapSession>, doc: string) {
    const parent = document.createElement("div");
    document.body.appendChild(parent);
    return new EditorView({ state: EditorState.create({ doc, extensions: dapExtension(session, MAIN) }), parent });
  }

  /** The breakpoint dots on lines — not the hidden spacer that sizes the gutter. */
  const dots = (view: EditorView) =>
    [...view.dom.querySelectorAll<HTMLElement>(".sp-breakpoint-dot")].filter((dot) => dot.closest<HTMLElement>(".cm-gutterElement")?.style.visibility !== "hidden");

  it("draws the session's breakpoints and keeps them current", async () => {
    const { host } = fakeHost();
    const session = dapSession(host, ROOT, "lldb");
    await session.toggleBreakpoint(MAIN, 2);
    const view = editor(session, "fn main() {\n    let answer = 42;\n}\n");
    await settle();
    expect(dots(view)).toHaveLength(1);
    await session.toggleBreakpoint(MAIN, 3);
    expect(dots(view)).toHaveLength(2);
    await session.toggleBreakpoint(MAIN, 2);
    await session.toggleBreakpoint(MAIN, 3);
    expect(dots(view)).toHaveLength(0);
    view.destroy();
  });

  it("marks the line a paused program is at", async () => {
    const { host, say } = fakeHost(STOP_ANSWERS);
    const session = dapSession(host, ROOT, "lldb");
    const view = editor(session, "fn main() {\n    let answer = 42;\n    println!();\n}\n");
    void session.start({});
    await settle();
    say({ event: "initialized" });
    await vi.waitFor(() => expect(session.snapshot().status).toBe("running"));
    expect(view.dom.querySelector(".sp-execution-line")).toBeNull();
    say({ event: "stopped", body: { reason: "breakpoint", threadId: 1 } });
    await vi.waitFor(() => expect(view.dom.querySelectorAll(".sp-execution-line")).toHaveLength(1));
    expect(view.dom.querySelector(".sp-execution-line")?.textContent).toContain("println");
    view.destroy();
  });

  it("stops listening when the editor goes", async () => {
    const { host } = fakeHost();
    const session = dapSession(host, ROOT, "lldb");
    const view = editor(session, "fn main() {}");
    await settle();
    view.destroy();
    // A change after the editor is gone must not touch its view.
    await expect(session.toggleBreakpoint(MAIN, 1)).resolves.toBeUndefined();
  });
});

describe("DebugToolbar", () => {
  it("starts the program, then offers stop and the stepping buttons by what it is doing", async () => {
    const { host, say } = fakeHost(STOP_ANSWERS);
    const session = dapSession(host, ROOT, "lldb");
    const parent = document.createElement("div");
    document.body.appendChild(parent);
    await act(async () => createRoot(parent).render(createElement(DebugToolbar, { session, launch: () => ({ program: "/w/a.out" }) })));
    const button = (title: string) => parent.querySelector<HTMLButtonElement>(`button[title="${title}"]`);
    expect(button("Start Debugging")).not.toBeNull();
    expect(button("Step Over")?.disabled).toBe(true);

    await act(async () => button("Start Debugging")?.click());
    await act(async () => say({ event: "initialized" }));
    await vi.waitFor(() => expect(session.snapshot().status).toBe("running"));
    await act(async () => {});
    expect(button("Stop Debugging")).not.toBeNull();
    expect(button("Pause")?.disabled).toBe(false);

    await act(async () => say({ event: "stopped", body: { reason: "breakpoint", threadId: 1 } }));
    await vi.waitFor(() => expect(session.snapshot().status).toBe("stopped"));
    await act(async () => {});
    expect(button("Continue")).not.toBeNull();
    expect(button("Step Over")?.disabled).toBe(false);
  });
});
