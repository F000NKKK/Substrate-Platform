// Per-keystroke / per-event hot paths of the generic widgets, on inputs the
// size an IDE actually produces (a big project tree, a 20k-row grid, a long
// build log). `npm run bench`.
import { test } from "vitest";
import { flattenVisible } from "./Tree/useTree";
import type { TreeNode } from "./Tree";
import { nestFiles, type FileTreeNode } from "./FileTree";
import { buildDisplayItems, filterRows, sortRows } from "./DataGrid/useDataGrid";
import type { DataGridColumn } from "./DataGrid";
import { appendOutputLine, clearOutputBuffer } from "./OutputPanel/outputBuffer";

function tree(depth: number, fanout: number, prefix = "n"): TreeNode<null>[] {
  return Array.from({ length: fanout }, (_, i) => {
    const id = `${prefix}/${i}`;
    return { id, label: id, data: null, children: depth > 1 ? tree(depth - 1, fanout, id) : undefined };
  });
}
const bigTree = tree(5, 8); // 37k nodes
const allExpanded = new Set<string>();
(function collect(nodes: TreeNode<null>[]) {
  for (const n of nodes) if (n.children) (allExpanded.add(n.id), collect(n.children));
})(bigTree);
const deepTree = tree(400, 1); // a 400-deep chain

test("tree", async ({ bench }) => {
  await bench.compare(
    bench("flattenVisible 37k expanded", () => {
      flattenVisible(bigTree, allExpanded, 0, null);
    }),
    bench("flattenVisible 400 deep", () => {
      flattenVisible(deepTree, new Set(Array.from({ length: 400 }, (_, i) => "n" + "/0".repeat(i + 1))), 0, null);
    }),
  );
});

// A flat folder of generated/nested files: Foo.yui + Foo.g.rs + Foo.ru.yres ...
const folder: FileTreeNode[] = Array.from({ length: 3000 }, (_, i) => ({
  id: `/p/${i}`,
  name: i % 3 === 0 ? `Screen${Math.floor(i / 3)}.yui` : `Screen${Math.floor(i / 3)}.${i % 3}.g.rs`,
  kind: "file" as const,
}));
const crowded: FileTreeNode[] = Array.from({ length: 3000 }, (_, i) => ({ id: `/q/${i}`, name: i === 0 ? "Main.yui" : `Main.${i}.yres`, kind: "file" as const }));
const parentOf = (name: string, siblings: Set<string>) => {
  const base = name.split(".")[0] + ".yui";
  return siblings.has(base.toLowerCase()) ? base : null;
};

test("file tree", async ({ bench }) => {
  await bench.compare(
    bench("nestFiles 3000 files, 1000 parents", () => {
      nestFiles(folder, parentOf);
    }),
    bench("nestFiles 3000 files under one parent", () => {
      nestFiles(crowded, parentOf);
    }),
  );
});

interface Row {
  id: number;
  name: string;
  size: number;
  kind: string;
}
const rows: Row[] = Array.from({ length: 20_000 }, (_, i) => ({
  id: i,
  name: `File_${(i * 7919) % 20_000}.rs`,
  size: (i * 104729) % 65_536,
  kind: ["Source", "Resource", "Screen", "Content"][i % 4],
}));
const columns: DataGridColumn<Row>[] = [
  { key: "name", header: "Name", render: (r) => r.name, sortValue: (r) => r.name, filterValue: (r) => r.name },
  { key: "size", header: "Size", render: (r) => r.size, sortValue: (r) => r.size, summary: "sum", summaryValue: (r) => r.size },
  { key: "kind", header: "Kind", render: (r) => r.kind, sortValue: (r) => r.kind },
];
const byKey = new Map(columns.map((c) => [c.key, c]));

test("data grid 20k rows", async ({ bench }) => {
  await bench.compare(
    bench("sort by name (text)", () => {
      sortRows(rows, [{ key: "name", direction: "asc" }], byKey);
    }),
    bench("sort by kind, then size", () => {
      sortRows(rows, [{ key: "kind", direction: "asc" }, { key: "size", direction: "desc" }], byKey);
    }),
    bench("filter name contains", () => {
      filterRows(rows, { name: "File_1" }, byKey);
    }),
    bench("group by kind with sums", () => {
      buildDisplayItems(rows, ["kind"], byKey, new Set());
    }),
  );
});

const logLine = "   Compiling app-core v0.40.0 (/home/user/Projects/app/crates/app-core)";
test("output buffer", async ({ bench }) => {
  await bench.compare(
    bench("append 20k lines at the 10k cap", { beforeEach: () => clearOutputBuffer("bench") }, () => {
      for (let i = 0; i < 20_000; i++) appendOutputLine("bench", logLine, 10_000);
    }),
  );
});
