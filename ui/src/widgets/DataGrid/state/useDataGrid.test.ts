import { describe, expect, it } from "vitest";
import { buildDisplayItems, filterRows, sortRows } from "./useDataGrid";
import type { DataGridColumn, DataGridSortState } from "../DataGrid";

interface Row {
  id: number;
  name: string;
  size: number;
}
const rows: Row[] = Array.from({ length: 500 }, (_, i) => ({ id: i, name: `File${(i * 37) % 120}.RS`, size: (i * 7) % 13 }));
const columns: DataGridColumn<Row>[] = [
  { key: "name", header: "Name", sortValue: (r) => r.name, filterValue: (r) => r.name },
  { key: "size", header: "Size", sortValue: (r) => r.size, summary: "sum", summaryValue: (r) => r.size },
  { key: "plain", header: "Plain" },
];
const byKey = new Map(columns.map((c) => [c.key, c]));

/** The comparator-per-pair sort this replaced — the reference order. */
function referenceSort(sort: DataGridSortState[]): Row[] {
  return [...rows].sort((a, b) => {
    for (const s of sort) {
      const column = byKey.get(s.key);
      if (!column) continue;
      const av = column.sortValue ? column.sortValue(a) : "";
      const bv = column.sortValue ? column.sortValue(b) : "";
      const cmp =
        (typeof av === "number" && typeof bv === "number" ? av - bv : String(av).localeCompare(String(bv), undefined, { numeric: true, sensitivity: "base" })) *
        (s.direction === "asc" ? 1 : -1);
      if (cmp !== 0) return cmp;
    }
    return 0;
  });
}

describe("DataGrid row pipeline", () => {
  it.each<DataGridSortState[]>([
    [{ key: "name", direction: "asc" }],
    [{ key: "name", direction: "desc" }],
    [{ key: "size", direction: "desc" }, { key: "name", direction: "asc" }],
    [{ key: "plain", direction: "asc" }, { key: "missing", direction: "asc" }],
  ])("sorts like the reference comparator (%j)", (...sort) => {
    expect(sortRows(rows, sort, byKey)).toEqual(referenceSort(sort));
  });

  it("sorts numbers inside text naturally", () => {
    const sorted = sortRows(rows, [{ key: "name", direction: "asc" }], byKey).map((r) => r.name);
    expect(sorted.indexOf("File2.RS")).toBeLessThan(sorted.indexOf("File10.RS"));
  });

  it("filters case-insensitively, ignoring blank and unknown filters", () => {
    const hits = filterRows(rows, { name: "file1.rs", size: "  ", missing: "x" }, byKey);
    expect(hits.map((r) => r.id)).toEqual(rows.filter((r) => r.name === "File1.RS").map((r) => r.id));
  });

  it("groups with per-group sums", () => {
    const items = buildDisplayItems(rows, ["size"], byKey, new Set());
    const groups = items.filter((i) => i.kind === "group");
    expect(groups.map((g) => g.label)).toEqual(Array.from({ length: 13 }, (_, i) => String(i)));
    for (const g of groups) if (g.kind === "group") expect(g.aggregates.size).toBe(Number(g.label) * g.count);
  });
});
