import { act, createElement, type KeyboardEvent } from "react";
import { createRoot } from "react-dom/client";
import { beforeAll, describe, expect, it } from "vitest";
import { useDataGrid } from "./useDataGrid";

beforeAll(() => {
  (globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
});

interface Row {
  id: string;
}
const rows: Row[] = ["a", "b", "c"].map((id) => ({ id }));

describe("DataGrid Ctrl+A", () => {
  it("selects every row of a row-selected grid, under any keyboard layout", async () => {
    let selected: Set<string> | undefined;
    let grid: ReturnType<typeof useDataGrid<Row>> | undefined;
    function Probe() {
      grid = useDataGrid<Row>({
        columns: [{ key: "id", header: "Id" }],
        rows,
        getRowId: (r) => r.id,
        selectable: true,
        onSelectionChange: (s) => (selected = new Set([...s].map(String))),
      });
      return null;
    }
    await act(async () => createRoot(document.createElement("div")).render(createElement(Probe)));
    let prevented = false;
    const e = { key: "ф", code: "KeyA", ctrlKey: true, metaKey: false, altKey: false, shiftKey: false, defaultPrevented: false, preventDefault: () => (prevented = true) };
    await act(async () => grid!.handleItemKeyDown(e as unknown as KeyboardEvent, 0));
    expect(prevented).toBe(true);
    expect(selected).toEqual(new Set(["a", "b", "c"]));
  });
});
