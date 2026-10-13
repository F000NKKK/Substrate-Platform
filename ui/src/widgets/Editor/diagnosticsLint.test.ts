import { describe, expect, it } from "vitest";
import { EditorState } from "@codemirror/state";
import { toLintDiagnostics } from "./diagnosticsLint";

describe("toLintDiagnostics", () => {
  const state = EditorState.create({ doc: '<Screen>\n  <Label color="#FF0000"/>\n</Screen>' });
  it("underlines the word at the column, or the line's text without one", () => {
    const [word, line] = toLintDiagnostics(state, [
      { severity: "warning", message: "w", line: 2, column: 4, origin: "live" },
      { severity: "error", message: "e", line: 2, origin: "build" },
      { severity: "error", message: "no line", origin: "build" },
    ]);
    expect(state.sliceDoc(word.from, word.to)).toBe("Label");
    expect(state.sliceDoc(line.from, line.to)).toBe('<Label color="#FF0000"/>');
  });
  it("uses an explicit span and maps fixes to actions", () => {
    let applied = false;
    const [d] = toLintDiagnostics(state, [
      { severity: "message", message: "m", line: 2, column: 16, endColumn: 25, origin: "live", fixes: [{ label: "Extract", apply: () => (applied = true) }] },
    ]);
    expect(state.sliceDoc(d.from, d.to)).toBe('"#FF0000"');
    expect(d.severity).toBe("info");
    d.actions![0].apply(null as never, d.from, d.to);
    expect(applied).toBe(true);
  });
});
