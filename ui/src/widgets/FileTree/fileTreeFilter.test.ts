import { describe, expect, it } from "vitest";
import { fileTreeFromEntries, filterFileTree, textMatchRanges } from "./fileTreeFilter";
import type { DirEntry } from "./useDirectoryTree";

const file = (path: string): DirEntry => ({ name: path.split("/").pop()!, path, kind: "file" });
const dir = (path: string): DirEntry => ({ name: path.split("/").pop()!, path, kind: "dir" });

const ENTRIES = [
  file("/p/yog.toml"),
  dir("/p/src"),
  file("/p/src/main.rs"),
  dir("/p/src/ui"),
  file("/p/src/ui/Main.yui"),
  file("/p/src/ui/Main.rs"),
  dir("/p/resources"),
  file("/p/resources/Strings.yres"),
];

const shape = (nodes: ReturnType<typeof fileTreeFromEntries>): unknown =>
  nodes.map((n) => (n.children ? { [n.name]: shape(n.children) } : n.name));

describe("textMatchRanges", () => {
  it("finds every occurrence, ignoring case", () => {
    expect(textMatchRanges("MainMenu.main", "main")).toEqual([
      [0, 4],
      [9, 13],
    ]);
    expect(textMatchRanges("lib.rs", "x")).toBeNull();
    expect(textMatchRanges("lib.rs", "  ")).toBeNull();
  });
});

describe("fileTreeFromEntries", () => {
  it("builds the whole tree, folders first then by name", () => {
    expect(shape(fileTreeFromEntries("/p", "p", ENTRIES))).toEqual([
      { p: [{ resources: ["Strings.yres"] }, { src: [{ ui: ["Main.rs", "Main.yui"] }, "main.rs"] }, "yog.toml"] },
    ]);
  });
});

describe("filterFileTree", () => {
  const tree = fileTreeFromEntries("/p", "p", ENTRIES);

  it("keeps matches and the folders leading to them, opened, with the hits marked", () => {
    const result = filterFileTree(tree, (n) => textMatchRanges(n.name, "main"));
    expect(shape(result.nodes)).toEqual([{ p: [{ src: [{ ui: ["Main.rs", "Main.yui"] }, "main.rs"] }] }]);
    expect([...result.expandedIds].sort()).toEqual(["/p", "/p/src", "/p/src/ui"]);
    expect(result.highlights.get("/p/src/ui/Main.yui")).toEqual([[0, 4]]);
    expect(result.count).toBe(3);
  });

  it("keeps a matched folder's contents when nothing in it matches", () => {
    const result = filterFileTree(tree, (n) => textMatchRanges(n.name, "resou"));
    expect(shape(result.nodes)).toEqual([{ p: [{ resources: ["Strings.yres"] }] }]);
    expect(result.expandedIds.has("/p/resources")).toBe(false);
  });

  it("follows nested files and takes plain true matches", () => {
    const nestedTree = [{ id: "/p/a.yui", name: "a.yui", kind: "file" as const, nested: [{ id: "/p/a.rs", name: "a.rs", kind: "file" as const }] }];
    const result = filterFileTree(nestedTree, (n) => n.id === "/p/a.rs");
    expect(result.nodes[0].nested?.map((n) => n.id)).toEqual(["/p/a.rs"]);
    expect(result.expandedIds).toEqual(new Set(["/p/a.yui"]));
    expect(result.highlights.size).toBe(0);
  });

  it("leaves nothing when nothing matches", () => {
    expect(filterFileTree(tree, () => false).nodes).toEqual([]);
  });
});
