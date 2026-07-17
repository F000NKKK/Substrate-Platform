import { useCallback, useState } from "react";
import type { FileTreeNode } from "./FileTree";

export interface DirEntry {
  name: string;
  path: string;
  kind: "file" | "dir";
}

/** Every operation is injected — the product supplies its own `invoke("dir_*", ...)` calls, the same "commands as props" shape `TerminalPanel` already uses. Omit a mutating callback to leave that action unavailable (e.g. a read-only browse mode). */
export interface DirectoryTreeCommands {
  list: (path: string) => Promise<DirEntry[]>;
  createFile?: (path: string) => Promise<void>;
  createDir?: (path: string) => Promise<void>;
  rename?: (from: string, to: string) => Promise<void>;
  remove?: (path: string) => Promise<void>;
}

function joinPath(parent: string, name: string): string {
  return `${parent.replace(/[/\\]+$/, "")}/${name}`;
}

function toNode(entry: DirEntry): FileTreeNode {
  return { id: entry.path, name: entry.name, kind: entry.kind === "dir" ? "folder" : "file" };
}

function replaceChildren(nodes: FileTreeNode[], id: string, children: FileTreeNode[]): FileTreeNode[] {
  return nodes.map((node) => {
    if (node.id === id) return { ...node, children };
    if (node.children) return { ...node, children: replaceChildren(node.children, id, children) };
    return node;
  });
}

/**
 * Backs `FileTree` with a real directory instead of a fully-materialized
 * tree: lists a folder's contents lazily, the first time it's expanded (via
 * `FileTree`'s controlled `expandedIds`/`onExpandedChange`), and re-lists a
 * folder after any create/rename/remove that touches it. `rootPath` becomes
 * the tree's single top-level node's id.
 */
export function useDirectoryTree(rootPath: string, rootName: string, commands: DirectoryTreeCommands) {
  const [nodes, setNodes] = useState<FileTreeNode[]>([{ id: rootPath, name: rootName, kind: "folder" }]);
  const [expandedIds, setExpandedIds] = useState<Set<string>>(new Set());
  const [loadedIds, setLoadedIds] = useState<Set<string>>(new Set());
  const [parentById, setParentById] = useState<Record<string, string>>({});

  const loadFolder = useCallback(
    async (id: string) => {
      const entries = await commands.list(id);
      setNodes((prev) => replaceChildren(prev, id, entries.map(toNode)));
      setParentById((prev) => {
        const next = { ...prev };
        for (const entry of entries) next[entry.path] = id;
        return next;
      });
      setLoadedIds((prev) => new Set(prev).add(id));
    },
    [commands]
  );

  const onExpandedChange = useCallback(
    (next: Set<string>) => {
      setExpandedIds(next);
      for (const id of next) {
        if (!loadedIds.has(id)) loadFolder(id);
      }
    },
    [loadedIds, loadFolder]
  );

  async function createFileIn(folderId: string, name: string) {
    if (!commands.createFile) return;
    await commands.createFile(joinPath(folderId, name));
    await loadFolder(folderId);
  }

  async function createDirIn(folderId: string, name: string) {
    if (!commands.createDir) return;
    await commands.createDir(joinPath(folderId, name));
    await loadFolder(folderId);
  }

  async function renameNode(node: FileTreeNode, newName: string) {
    if (!commands.rename) return;
    const parent = parentById[node.id];
    if (!parent) return;
    await commands.rename(node.id, joinPath(parent, newName));
    await loadFolder(parent);
  }

  async function removeNode(node: FileTreeNode) {
    if (!commands.remove) return;
    await commands.remove(node.id);
    const parent = parentById[node.id];
    if (parent) await loadFolder(parent);
  }

  return {
    nodes,
    expandedIds,
    onExpandedChange,
    createFileIn,
    createDirIn,
    renameNode,
    removeNode,
    reload: loadFolder,
  };
}
