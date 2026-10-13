import { describe, expect, it } from "vitest";
import type { Diagnostic } from "../../infra/diagnostics";
import { DEFAULT_ERROR_LIST_FILTER, filterDiagnostics } from "./errorListFilter";

const ds: Diagnostic[] = [
  { severity: "error", message: "cannot find value", code: "E0425", file: "/p/a.rs", line: 3, origin: "build" },
  { severity: "warning", message: "unused variable", file: "/p/b.rs", line: 1, origin: "build" },
  { severity: "message", message: "note", origin: "live" },
  { severity: "error", message: "bad attribute", file: "/p/s.yui", line: 9, origin: "live" },
];

describe("filterDiagnostics", () => {
  it("counts every severity but lists only the toggled ones", () => {
    const f = { ...DEFAULT_ERROR_LIST_FILTER, shown: { error: true, warning: false, message: false } };
    expect(filterDiagnostics(ds, f, "")).toEqual({ rows: [0, 3], counts: { error: 2, warning: 1, message: 1 } });
  });

  it("narrows by scope, origin and search", () => {
    expect(filterDiagnostics(ds, { ...DEFAULT_ERROR_LIST_FILTER, scope: "current" }, "", { activeFile: "/p/b.rs" }).rows).toEqual([1]);
    expect(filterDiagnostics(ds, { ...DEFAULT_ERROR_LIST_FILTER, scope: "open" }, "", { openFiles: ["/p/a.rs", "/p/s.yui"] }).rows).toEqual([0, 3]);
    expect(filterDiagnostics(ds, { ...DEFAULT_ERROR_LIST_FILTER, origins: "live" }, "").rows).toEqual([2, 3]);
    expect(filterDiagnostics(ds, DEFAULT_ERROR_LIST_FILTER, "e0425").rows).toEqual([0]);
  });
});
