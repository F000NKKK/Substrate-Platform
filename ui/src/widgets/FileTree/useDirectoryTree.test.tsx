import { act, createElement } from "react";
import { createRoot } from "react-dom/client";
import { beforeAll, describe, expect, it } from "vitest";
import { useDirectoryTree, type DirEntry } from "./useDirectoryTree";

beforeAll(() => {
  (globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
});

/** A fake disk: folder → its entries. */
const disk = new Map<string, DirEntry[]>();
const file = (path: string): DirEntry => ({ name: path.split("/").pop()!, path, kind: "file" });
const dir = (path: string): DirEntry => ({ name: path.split("/").pop()!, path, kind: "dir" });
const commands = { list: async (path: string) => disk.get(path) ?? [] };

type Tree = ReturnType<typeof useDirectoryTree>;

async function mount(): Promise<() => Tree> {
  let tree: Tree | undefined;
  function Probe() {
    tree = useDirectoryTree("/p", "p", commands);
    return null;
  }
  await act(async () => createRoot(document.createElement("div")).render(createElement(Probe)));
  return () => tree!;
}

const names = (tree: Tree, id: string) =>
  (function find(nodes = tree.nodes): string[] | undefined {
    for (const n of nodes) {
      if (n.id === id) return n.children?.map((c) => c.name);
      const inner = n.children && find(n.children);
      if (inner) return inner;
    }
  })();

describe("useDirectoryTree.refreshPaths", () => {
  it("re-lists the listed folders an outside change touched", async () => {
    disk.set("/p", [dir("/p/src"), file("/p/a.txt")]);
    disk.set("/p/src", [file("/p/src/lib.rs")]);
    const tree = await mount();
    expect([...tree().loadedFolders]).toEqual(["/p"]);
    await act(async () => tree().onExpandedChange(new Set(["/p", "/p/src"])));
    expect(names(tree(), "/p/src")).toEqual(["lib.rs"]);

    // A terminal adds a file in src/ and deletes a.txt.
    disk.set("/p/src", [file("/p/src/lib.rs"), file("/p/src/new.rs")]);
    disk.set("/p", [dir("/p/src")]);
    await act(() => tree().refreshPaths(["/p/src/new.rs", "/p/a.txt"]));
    expect(names(tree(), "/p/src")).toEqual(["lib.rs", "new.rs"]);
    expect(names(tree(), "/p")).toEqual(["src"]);
  });

  it("ignores folders it never listed", async () => {
    disk.set("/p", [dir("/p/target")]);
    let listed = 0;
    const counting = { list: async (path: string) => (listed++, disk.get(path) ?? []) };
    let tree: Tree | undefined;
    function Probe() {
      tree = useDirectoryTree("/p", "p", counting);
      return null;
    }
    await act(async () => createRoot(document.createElement("div")).render(createElement(Probe)));
    listed = 0;
    await act(() => tree!.refreshPaths(["/p/target/debug/x.o"]));
    expect(listed).toBe(0);
  });
});

describe("useDirectoryTree.expandSubtree / revealPath", () => {
  it("lists and opens every folder under one, skipping what it's told to", async () => {
    disk.set("/p", [dir("/p/src"), dir("/p/target")]);
    disk.set("/p/src", [dir("/p/src/ui"), file("/p/src/lib.rs")]);
    disk.set("/p/src/ui", [dir("/p/src/ui/widgets")]);
    disk.set("/p/src/ui/widgets", []);
    disk.set("/p/target", [dir("/p/target/debug")]);
    const tree = await mount();
    await act(() => tree().expandSubtree("/p", { skip: (path) => path.endsWith("/target") }));
    expect([...tree().expandedIds].sort()).toEqual(["/p", "/p/src", "/p/src/ui", "/p/src/ui/widgets"]);
    expect(names(tree(), "/p/src/ui")).toEqual(["widgets"]);
  });

  it("opens the folders down to a path, listing them as it goes", async () => {
    disk.set("/p", [dir("/p/src")]);
    disk.set("/p/src", [dir("/p/src/ui")]);
    disk.set("/p/src/ui", [file("/p/src/ui/Main.yui")]);
    const tree = await mount();
    let found = false;
    await act(async () => {
      found = await tree().revealPath("/p/src/ui/Main.yui");
    });
    expect(found).toBe(true);
    expect([...tree().expandedIds].sort()).toEqual(["/p", "/p/src", "/p/src/ui"]);
    expect(names(tree(), "/p/src/ui")).toEqual(["Main.yui"]);
    expect(await tree().revealPath("/elsewhere/x.rs")).toBe(false);
  });
});
