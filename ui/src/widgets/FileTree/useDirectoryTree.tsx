import { useCallback, useEffect, useRef, useState } from "react";
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

/** Whether `id` is `ancestor` itself or anywhere beneath it. */
function isWithin(id: string, ancestor: string): boolean {
  if (id === ancestor) return true;
  const base = ancestor.replace(/[/\\]+$/, "");
  return id.startsWith(`${base}/`) || id.startsWith(`${base}\\`);
}

function withoutSubtree(set: Set<string>, root: string): Set<string> {
  return new Set([...set].filter((id) => !isWithin(id, root)));
}

/**
 * Backs `FileTree` with a real directory instead of a fully-materialized
 * tree: lists a folder's contents lazily, the first time it's expanded (via
 * `FileTree`'s controlled `expandedIds`/`onExpandedChange`), and re-lists a
 * folder after any create/rename/remove that touches it. `rootPath` becomes
 * the tree's single top-level node's id; changing it starts over with a
 * fresh tree.
 *
 * A failed listing never rejects out of the hook's own effects — it's
 * reported to `onError` (default: `console.error`) and the folder stays
 * unloaded, so expanding it again retries.
 */
export function useDirectoryTree(
  rootPath: string,
  rootName: string,
  commands: DirectoryTreeCommands,
  onError: (error: unknown, path: string) => void = defaultOnError
) {
  const [nodes, setNodes] = useState<FileTreeNode[]>([{ id: rootPath, name: rootName, kind: "folder" }]);
  const [expandedIds, setExpandedIds] = useState<Set<string>>(new Set([rootPath]));
  const [loadedIds, setLoadedIds] = useState<Set<string>>(new Set());
  const [parentById, setParentById] = useState<Record<string, string>>({});
  const rootRef = useRef(rootPath);
  rootRef.current = rootPath;
  const onErrorRef = useRef(onError);
  onErrorRef.current = onError;

  const loadFolder = useCallback(
    async (id: string) => {
      const root = rootRef.current;
      let entries: DirEntry[];
      try {
        entries = await commands.list(id);
      } catch (err) {
        onErrorRef.current(err, id);
        return;
      }
      // The root changed while this listing was in flight — it belongs to a
      // tree that no longer exists.
      if (rootRef.current !== root) return;
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

  // Resets to a fresh single-node tree whenever the root changes, then loads
  // it — the root starts pre-expanded, so there's no `onExpandedChange`
  // coming for it to trigger the load itself.
  const initialRootRef = useRef(rootPath);
  useEffect(() => {
    if (initialRootRef.current !== rootPath) {
      initialRootRef.current = rootPath;
      setNodes([{ id: rootPath, name: rootName, kind: "folder" }]);
      setExpandedIds(new Set([rootPath]));
      setLoadedIds(new Set());
      setParentById({});
    }
    loadFolder(rootPath);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [rootPath]);

  // Renaming the root's display name alone doesn't need a reload.
  useEffect(() => {
    setNodes((prev) => prev.map((node) => (node.id === rootPath ? { ...node, name: rootName } : node)));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [rootName]);

  const onExpandedChange = useCallback(
    (next: Set<string>) => {
      setExpandedIds(next);
      for (const id of next) {
        if (!loadedIds.has(id)) loadFolder(id);
      }
    },
    [loadedIds, loadFolder]
  );

  /** Forgets everything known about `id`'s subtree — its path no longer exists (removed, or renamed away). */
  function forgetSubtree(id: string) {
    setExpandedIds((prev) => withoutSubtree(prev, id));
    setLoadedIds((prev) => withoutSubtree(prev, id));
    setParentById((prev) => Object.fromEntries(Object.entries(prev).filter(([child]) => !isWithin(child, id))));
  }

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
    forgetSubtree(node.id);
    await loadFolder(parent);
  }

  async function removeNode(node: FileTreeNode) {
    if (!commands.remove) return;
    await commands.remove(node.id);
    const parent = parentById[node.id];
    forgetSubtree(node.id);
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

function defaultOnError(error: unknown, path: string) {
  console.error(`failed to list ${path}:`, error);
}
