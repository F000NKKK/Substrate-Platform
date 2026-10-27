import { afterEach, describe, expect, it, vi } from "vitest";
import { EditorState, Text } from "@codemirror/state";
import { EditorView } from "@codemirror/view";

vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn(), emit: vi.fn(() => Promise.resolve()) }));

import { getDiagnostics, clearDiagnostics } from "../infra/diagnostics";
import { onOpenFileRequest, type FileOpenRequest } from "../documents/fileOpenBus";
import { markupText, toCompletions, toDiagnostics, toOffset, toOpenRequest, toPosition } from "./convert";
import { CHANGE_DEBOUNCE_MS, lspExtension } from "./lspExtension";
import { languageFor, type LanguageInfo, type LspHost, type LspNotification } from "./lspHost";
import { lspSession } from "./lspSession";
import { pathFromUri, uriFromPath } from "./uri";

/** A host that records every call and answers requests from `answers`. */
function fakeHost(answers: Record<string, unknown> = {}) {
  const calls: string[] = [];
  let listener: ((n: LspNotification) => void) | undefined;
  const record = (name: string) =>
    vi.fn(async (...args: unknown[]) => {
      calls.push(`${name} ${args.filter((a) => typeof a !== "object").join(" ")}`);
    });
  const host: LspHost = {
    languages: async () => [],
    start: vi.fn(async () => ({ hoverProvider: true })),
    request: async <T,>(_root: string, _language: string, method: string) => {
      calls.push(`request ${method}`);
      return answers[method] as T;
    },
    notify: record("notify"),
    didOpen: record("didOpen"),
    didChange: record("didChange"),
    didSave: record("didSave"),
    didClose: record("didClose"),
    stop: record("stop"),
    onNotification: async (handler) => {
      listener = handler;
      return () => {};
    },
    onOutput: async () => () => {},
  };
  return { host, calls, notify: (n: LspNotification) => listener?.(n) };
}

function editor(host: LspHost, doc: string) {
  const session = lspSession(host, "/w", "rust");
  const parent = document.createElement("div");
  document.body.appendChild(parent);
  return new EditorView({ state: EditorState.create({ doc, extensions: lspExtension(session, "/w/src/main.rs") }), parent });
}

const settle = () => new Promise((resolve) => setTimeout(resolve, 0));

afterEach(() => {
  vi.useRealTimers();
  clearDiagnostics(() => true);
});

describe("uri", () => {
  it("encodes what a path can't keep and round-trips", () => {
    const path = "/home/me/my project/ü#1.rs";
    expect(uriFromPath(path)).toBe("file:///home/me/my%20project/%C3%BC%231.rs");
    expect(pathFromUri(uriFromPath(path))).toBe(path);
  });

  it("turns a Windows drive into a Windows path", () => {
    expect(uriFromPath("C:\\Users\\me\\a.rs")).toBe("file:///C:/Users/me/a.rs");
    expect(pathFromUri("file:///C:/Users/me/a.rs")).toBe("C:\\Users\\me\\a.rs");
    expect(pathFromUri("https://example.com/a")).toBeUndefined();
    expect(pathFromUri("file:///bad%zz")).toBeUndefined();
  });
});

describe("convert", () => {
  const doc = Text.of(["fn main() {", "    let x = 1;", "}"]);

  it("maps positions to offsets and back, clamped", () => {
    expect(toOffset(doc, { line: 1, character: 8 })).toBe(20);
    expect(toPosition(doc, 20)).toEqual({ line: 1, character: 8 });
    expect(toOffset(doc, { line: 9, character: 99 })).toBe(doc.length);
  });

  it("makes Error List entries 1-based with an exclusive end", () => {
    const [d] = toDiagnostics(
      "/w/a.rs",
      [{ range: { start: { line: 1, character: 8 }, end: { line: 1, character: 9 } }, severity: 2, code: "unused", message: "unused x" }],
      "rust"
    );
    expect(d).toEqual({
      severity: "warning",
      message: "unused x",
      code: "unused",
      codeHref: undefined,
      file: "/w/a.rs",
      line: 2,
      column: 9,
      endLine: 2,
      endColumn: 10,
      tool: "rust",
      origin: "live",
    });
  });

  it("reads completion lists and items' edits", () => {
    const result = toCompletions(
      doc,
      {
        items: [
          { label: "println!", kind: 3, documentation: { kind: "markdown", value: "Prints." } },
          { label: "x", kind: 6, textEdit: { newText: "x", range: { start: { line: 1, character: 7 }, end: { line: 1, character: 8 } } } },
        ],
      },
      20
    );
    expect(result.from).toBe(19);
    expect(result.options.map((o) => [o.label, o.type, o.info, o.apply])).toEqual([
      ["println!", "function", "Prints.", "println!"],
      ["x", "variable", undefined, "x"],
    ]);
  });

  it("joins hover contents and finds a definition's target", () => {
    expect(markupText([{ language: "rust", value: "fn main()" }, "Entry point."])).toBe("```rust\nfn main()\n```\n\nEntry point.");
    const range = { start: { line: 4, character: 2 }, end: { line: 4, character: 6 } };
    expect(toOpenRequest([{ targetUri: "file:///w/lib.rs", targetRange: range }])).toEqual({ path: "/w/lib.rs", line: 5, column: 3 });
    expect(toOpenRequest(null)).toBeUndefined();
  });
});

describe("languageFor", () => {
  const languages: LanguageInfo[] = [
    { id: "rust", extensions: ["rs"], fileNames: [], hasServer: true },
    { id: "toml", extensions: ["toml"], fileNames: ["Cargo.lock"], hasServer: false },
  ];

  it("picks by file name before extension", () => {
    expect(languageFor(languages, "/w/src/MAIN.RS")?.id).toBe("rust");
    expect(languageFor(languages, "C:\\w\\Cargo.lock")?.id).toBe("toml");
    expect(languageFor(languages, "/w/README")).toBeUndefined();
  });
});

describe("lspExtension", () => {
  it("opens the document, sends changes after a pause, closes with the editor", async () => {
    vi.useFakeTimers();
    const { host, calls } = fakeHost();
    const view = editor(host, "fn main() {}");
    await vi.runAllTimersAsync();
    view.dispatch({ changes: { from: 0, insert: "pub " } });
    view.dispatch({ changes: { from: 0, insert: "/// Doc\n" } });
    await vi.advanceTimersByTimeAsync(CHANGE_DEBOUNCE_MS);
    view.destroy();
    await vi.runAllTimersAsync();
    expect(calls).toEqual(["didOpen /w rust /w/src/main.rs 1 fn main() {}", "didChange /w rust /w/src/main.rs 2 /// Doc\npub fn main() {}", "didClose /w rust /w/src/main.rs"]);
  });

  it("puts a server's diagnostics in the Error List", async () => {
    const { host, notify } = fakeHost();
    const view = editor(host, "fn main() {}");
    await settle();
    notify({
      root: "/w",
      language: "rust",
      method: "textDocument/publishDiagnostics",
      params: {
        uri: "file:///w/src/main.rs",
        diagnostics: [{ range: { start: { line: 0, character: 3 }, end: { line: 0, character: 7 } }, message: "unused" }],
      },
    });
    expect(getDiagnostics().map((d) => [d.file, d.line, d.column, d.message, d.origin])).toEqual([["/w/src/main.rs", 1, 4, "unused", "live"]]);
    view.destroy();
  });

  it("opens a definition on F12", async () => {
    const { host, calls } = fakeHost({
      "textDocument/definition": { uri: "file:///w/src/lib.rs", range: { start: { line: 2, character: 4 }, end: { line: 2, character: 8 } } },
    });
    const opened: FileOpenRequest[] = [];
    const stop = onOpenFileRequest((r) => opened.push(r));
    const view = editor(host, "fn main() { helper(); }");
    view.dispatch({ selection: { anchor: 13 } });
    view.contentDOM.dispatchEvent(new KeyboardEvent("keydown", { key: "F12", bubbles: true }));
    await vi.waitFor(() => expect(opened).toEqual([{ path: "/w/src/lib.rs", line: 3, column: 5 }]));
    expect(calls).toContain("request textDocument/definition");
    stop();
    view.destroy();
  });
});
