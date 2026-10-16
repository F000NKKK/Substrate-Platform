import { act, createElement, useState } from "react";
import { createRoot } from "react-dom/client";
import { beforeAll, describe, expect, it } from "vitest";
import { Tree, type TreeNode } from "./Tree";
import { flattenVisible } from "./useTree";
import { branchIdsUnder, navigationTarget, rangeIds, typeAheadIndex } from "./treeNav";

beforeAll(() => {
  (globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
});

const leaf = (id: string): TreeNode<null> => ({ id, label: id, data: null });
const branch = (id: string, children: TreeNode<null>[]): TreeNode<null> => ({ id, label: id, data: null, children });

const NODES = [branch("src", [leaf("alpha.rs"), leaf("apple.rs"), branch("ui", [leaf("main.yui")]), leaf("beta.rs")]), leaf("yog.toml")];

describe("typeAheadIndex", () => {
  const flat = flattenVisible(NODES, new Set(["src"]));
  const ids = flat.map((e) => e.node.id);

  it("jumps to the next row starting with the letter, wrapping around", () => {
    expect(ids[typeAheadIndex(flat, 0, "y")]).toBe("yog.toml");
    expect(ids[typeAheadIndex(flat, ids.indexOf("yog.toml"), "s")]).toBe("src");
  });

  it("cycles through the rows when one letter repeats", () => {
    expect(ids[typeAheadIndex(flat, ids.indexOf("alpha.rs"), "aa")]).toBe("apple.rs");
    expect(ids[typeAheadIndex(flat, ids.indexOf("apple.rs"), "aaa")]).toBe("alpha.rs");
  });

  it("keeps the current row while a longer prefix still fits it", () => {
    expect(ids[typeAheadIndex(flat, ids.indexOf("apple.rs"), "ap")]).toBe("apple.rs");
    expect(ids[typeAheadIndex(flat, ids.indexOf("alpha.rs"), "ap")]).toBe("apple.rs");
    expect(typeAheadIndex(flat, 0, "zz-none")).toBe(-1);
  });
});

describe("tree navigation helpers", () => {
  it("expands every branch under a node with *", () => {
    expect(branchIdsUnder(NODES[0])).toEqual(["src", "ui"]);
    expect(branchIdsUnder(NODES[1])).toEqual([]);
  });

  it("moves by arrows, Home/End and pages, clamped", () => {
    expect(navigationTarget("ArrowDown", 4, 5, 10)).toBe(4);
    expect(navigationTarget("ArrowUp", 0, 5, 10)).toBe(0);
    expect(navigationTarget("End", 1, 5, 10)).toBe(4);
    expect(navigationTarget("PageUp", 3, 5, 2)).toBe(1);
    expect(navigationTarget("x", 3, 5, 2)).toBeNull();
  });

  it("selects a Shift range either way", () => {
    const flat = flattenVisible(NODES, new Set());
    expect(rangeIds(flat, 1, 0)).toEqual(["src", "yog.toml"]);
  });
});

/** A rendered tree, selection and expansion owned here — keyboard driven. */
async function mountTree(onExpandAll?: (node: TreeNode<null>) => void) {
  const host = document.createElement("div");
  document.body.appendChild(host);
  const state = { selected: new Set<string>(), expanded: new Set<string>() };
  function Harness() {
    const [selected, setSelected] = useState(new Set<string>());
    const [expanded, setExpanded] = useState(new Set<string>());
    state.selected = selected;
    state.expanded = expanded;
    return createElement(Tree<null>, {
      nodes: NODES,
      selectedIds: selected,
      onSelectionChange: setSelected,
      expandedIds: expanded,
      onExpandedChange: setExpanded,
      onExpandAll,
    });
  }
  await act(async () => createRoot(host).render(createElement(Harness)));
  const row = (id: string) => [...host.querySelectorAll<HTMLElement>(".sp-tree-row")].find((r) => r.dataset.nodeId === id)!;
  const press = async (id: string, key: string, init: KeyboardEventInit = {}) =>
    act(async () => {
      row(id).dispatchEvent(new KeyboardEvent("keydown", { key, bubbles: true, cancelable: true, ...init }));
    });
  return { state, row, press };
}

describe("Tree keyboard", () => {
  it("arrows select and move the focus; Right/Left expand, enter and leave a branch", async () => {
    const t = await mountTree();
    await t.press("src", "ArrowDown");
    expect([...t.state.selected]).toEqual(["yog.toml"]);
    expect(document.activeElement).toBe(t.row("yog.toml"));
    await t.press("yog.toml", "ArrowUp");
    await t.press("src", "ArrowRight");
    expect(t.state.expanded.has("src")).toBe(true);
    await t.press("src", "ArrowRight");
    expect([...t.state.selected]).toEqual(["alpha.rs"]);
    await t.press("alpha.rs", "ArrowLeft");
    expect([...t.state.selected]).toEqual(["src"]);
    await t.press("src", "ArrowLeft");
    expect(t.state.expanded.has("src")).toBe(false);
  });

  it("Shift extends the selection, Ctrl only moves the focus", async () => {
    const t = await mountTree();
    await t.press("src", "ArrowRight");
    await t.press("src", "ArrowDown");
    await t.press("alpha.rs", "ArrowDown", { shiftKey: true });
    expect([...t.state.selected].sort()).toEqual(["alpha.rs", "apple.rs"]);
    await t.press("apple.rs", "ArrowDown", { ctrlKey: true });
    expect([...t.state.selected].sort()).toEqual(["alpha.rs", "apple.rs"]);
    expect(document.activeElement).toBe(t.row("ui"));
  });

  it("* expands everything under the row, type-ahead jumps by name", async () => {
    const t = await mountTree();
    await t.press("src", "*");
    expect([...t.state.expanded].sort()).toEqual(["src", "ui"]);
    await t.press("src", "b");
    expect([...t.state.selected]).toEqual(["beta.rs"]);
  });

  it("* goes to onExpandAll when the tree has one", async () => {
    const seen: string[] = [];
    const t = await mountTree((node) => seen.push(node.id));
    await t.press("src", "*");
    expect(seen).toEqual(["src"]);
  });
});
