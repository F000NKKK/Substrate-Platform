import { describe, expect, it } from "vitest";
import { nestFiles, type FileTreeNode } from "./FileTree";
import { flattenVisible } from "../Tree/useTree";

const file = (name: string): FileTreeNode => ({ id: name, name, kind: "file" });
const parentOf = (name: string, siblings: Set<string>) => {
  const base = `${name.split(".")[0]}.yui`;
  return siblings.has(base.toLowerCase()) ? base : null;
};

describe("nestFiles", () => {
  it("nests dependents under their parent in order, leaving the rest", () => {
    const nested = nestFiles([file("Main.yui"), file("Main.g.rs"), file("lib.rs"), file("Main.ru.yres")], parentOf);
    expect(nested.map((n) => [n.name, n.nested?.map((c) => c.name)])).toEqual([
      ["Main.yui", ["Main.g.rs", "Main.ru.yres"]],
      ["lib.rs", undefined],
    ]);
  });
});

describe("flattenVisible", () => {
  it("lists expanded children depth-first with their depth and parent", () => {
    const nodes = [{ id: "a", label: "a", data: 0, children: [{ id: "b", label: "b", data: 0, children: [{ id: "c", label: "c", data: 0 }] }] }, { id: "d", label: "d", data: 0 }];
    expect(flattenVisible(nodes, new Set(["a"])).map((e) => [e.node.id, e.depth, e.parentId])).toEqual([
      ["a", 0, null],
      ["b", 1, "a"],
      ["d", 0, null],
    ]);
  });
});
