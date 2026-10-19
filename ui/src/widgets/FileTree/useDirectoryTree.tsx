import { useCallback, useEffect, useRef, useState } from "react";
import type { FileTreeNode } from "./FileTree";
import { useStateStore } from "../../infra/state";

export interface DirEntry {
  name: string;
  path: string;
  kind: "file" | "dir";
}

/** Every operation is injected — `hostFiles` serves them through the host plugin. Omit a mutating callback to leave that action unavailable (e.g. a read-only browse mode). */
export interface DirectoryTreeCommands {
  list: (path: string) => Promise<DirEntry[]>;
  createFile?: (path: string) => Promise<void>;
  createDir?: (path: string) => Promise<void>;
  rename?: (from: string, to: string) => Promise<void>;
  remove?: (path: string) => Promise<void>;
}

/** How many folders `expandSubtree` opens at most. */
const EXPAND_SUBTREE_LIMIT = 500;

function joinPath(parent: string, name: string): string {
  return `${parent.replace(/[/\\]+$/, "")}/${name}`;
}

function toNode(entry: DirEntry): FileTreeNode {
  return { id: entry.path, name: entry.name, kind: entry.kind === "dir" ? "folder" : "file" };
}

/** Sets `id`'s children to a fresh listing; a child that was there before keeps what's loaded beneath it. */
function replaceChildren(nodes: FileTreeNode[], id: string, children: FileTreeNode[]): FileTreeNode[] {
  return nodes.map((node) => {
    if (node.id === id) {
      const before = new Map((node.children ?? []).map((c) => [c.id, c]));
      return {
        ...node,
        children: children.map((c) => {
          const old = before.get(c.id);
          return old?.children && old.kind === c.kind ? { ...c, children: old.children } : c;
        }),
      };
    }
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
 *
 * With `persistKey`, the expanded folders are remembered under that key in
 * the current `StateStore` (relative to the root) and re-expanded the next
 * time this root is shown — any that no longer exist are silently skipped.
 */
export function useDirectoryTree(
  rootPath: string,
  rootName: string,
  commands: DirectoryTreeCommands,
  onError: (error: unknown, path: string) => void = defaultOnError,
  persistKey?: string
) {
  const [nodes, setNodes] = useState<FileTreeNode[]>([{ id: rootPath, name: rootName, kind: "folder" }]);
  const [expandedIds, setExpandedIds] = useState<Set<string>>(new Set([rootPath]));
  const [loadedIds, setLoadedIds] = useState<Set<string>>(new Set());
  const loadedRef = useRef(loadedIds);
  loadedRef.current = loadedIds;
  const [parentById, setParentById] = useState<Record<string, string>>({});
  const rootRef = useRef(rootPath);
  rootRef.current = rootPath;
  const onErrorRef = useRef(onError);
  onErrorRef.current = onError;
  const store = useStateStore();
  // Whether the persisted expansion has been re-applied for the current root
  // — until then, what's expanded isn't worth saving (it'd overwrite the
  // saved set with just the root).
  const restoredRef = useRef(false);

  /** The listing (`null` if it failed — reported to `onError` unless `silent` — or belongs to a root no longer shown). */
  const listFolder = useCallback(
    async (id: string, silent = false): Promise<DirEntry[] | null> => {
      const root = rootRef.current;
      let entries: DirEntry[];
      try {
        entries = await commands.list(id);
      } catch (err) {
        if (!silent) onErrorRef.current(err, id);
        return null;
      }
      // The root changed while this listing was in flight — it belongs to a
      // tree that no longer exists.
      if (rootRef.current !== root) return null;
      setNodes((prev) => replaceChildren(prev, id, entries.map(toNode)));
      setParentById((prev) => {
        const next = { ...prev };
        for (const entry of entries) next[entry.path] = id;
        return next;
      });
      setLoadedIds((prev) => new Set(prev).add(id));
      loadedRef.current = new Set(loadedRef.current).add(id);
      return entries;
    },
    [commands]
  );

  /** Resolves `false` if the listing failed (reported to `onError` unless `silent`) or belongs to a root no longer shown. */
  const loadFolder = useCallback(async (id: string, silent = false): Promise<boolean> => (await listFolder(id, silent)) !== null, [listFolder]);

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
    restoredRef.current = false;
    const saved = persistKey ? store.get(persistKey) : undefined;
    const relative = Array.isArray(saved) ? saved.filter((p): p is string => typeof p === "string" && p !== "") : [];
    (async () => {
      if (!(await loadFolder(rootPath))) return;
      // Parents before children (a child's node only exists once its parent
      // is listed); a folder that's gone, or whose parent didn't come back,
      // just stays collapsed.
      const restored = new Set<string>();
      for (const rel of [...relative].sort((a, b) => a.length - b.length)) {
        const id = joinPath(rootPath, rel);
        const parent = id.replace(/[/\\][^/\\]*$/, "");
        if (parent !== rootPath.replace(/[/\\]+$/, "") && !restored.has(parent)) continue;
        if (rootRef.current !== rootPath) return;
        if (await loadFolder(id, true)) restored.add(id);
      }
      if (rootRef.current !== rootPath) return;
      if (restored.size > 0) setExpandedIds((prev) => new Set([...prev, ...restored]));
      restoredRef.current = true;
    })();
    // Also re-run when the store switches (its saved state for this root
    // only just became available, e.g. a workspace file finished loading).
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [rootPath, store]);

  useEffect(() => {
    if (!persistKey || !restoredRef.current) return;
    const base = rootPath.replace(/[/\\]+$/, "");
    const relative = [...expandedIds]
      .filter((id) => id !== rootPath && isWithin(id, base))
      .map((id) => id.slice(base.length + 1));
    store.set(persistKey, relative);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [expandedIds]);

  // Renaming the root's display name alone doesn't need a reload.
  useEffect(() => {
    setNodes((prev) => prev.map((node) => (node.id === rootPath ? { ...node, name: rootName } : node)));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [rootName]);

  const onExpandedChange = useCallback(
    (next: Set<string>) => {
      setExpandedIds(next);
      for (const id of next) {
        // A file can be expanded too — to show the files nested under it.
        if (!loadedIds.has(id) && kindOf(nodes, id) !== "file") loadFolder(id);
      }
    },
    [loadedIds, loadFolder, nodes]
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

  /**
   * Something changed these paths outside the tree (a file watcher saw
   * it): re-lists every listed folder they're in, or are — silently, since
   * a folder that's gone simply drops out of its parent's listing.
   */
  const refreshPaths = useCallback(
    async (paths: string[]) => {
      const folders = new Set<string>();
      for (const path of paths) {
        if (loadedRef.current.has(path)) folders.add(path);
        const parent = path.replace(/[/\\][^/\\]*$/, "");
        if (loadedRef.current.has(parent)) folders.add(parent);
      }
      for (const folder of folders) await loadFolder(folder, true);
    },
    [loadFolder]
  );

  /**
   * `*` on a folder: lists every folder under `id` not listed yet (up to
   * `limit` of them, skipping the ones `skip` names — a build output folder
   * nobody wants opened wholesale) and opens them all.
   */
  async function expandSubtree(id: string, options: { limit?: number; skip?: (path: string) => boolean } = {}) {
    const limit = options.limit ?? EXPAND_SUBTREE_LIMIT;
    const opened = new Set<string>([id]);
    const queue = [id];
    while (queue.length > 0 && opened.size < limit) {
      const folder = queue.shift()!;
      const entries = await listFolder(folder, true);
      if (!entries) continue;
      for (const entry of entries) {
        if (entry.kind !== "dir" || options.skip?.(entry.path) || opened.size >= limit) continue;
        opened.add(entry.path);
        queue.push(entry.path);
      }
    }
    setExpandedIds((prev) => new Set([...prev, ...opened]));
  }

  /**
   * Sync with Active Document: lists and opens the folders down to `path`
   * so its row exists. Resolves whether `path` is under the root and its
   * folders could be listed.
   */
  async function revealPath(path: string): Promise<boolean> {
    const base = rootPath.replace(/[/\\]+$/, "");
    if (path === rootPath || !isWithin(path, base)) return path === rootPath;
    const chain: string[] = [rootPath];
    const rest = path.slice(base.length + 1).split(/[/\\]/);
    rest.pop();
    let at = base;
    for (const part of rest) {
      at = joinPath(at, part);
      chain.push(at);
    }
    for (const folder of chain) {
      if (!loadedRef.current.has(folder) && !(await loadFolder(folder, true))) return false;
    }
    setExpandedIds((prev) => new Set([...prev, ...chain]));
    return true;
  }

  /** Refresh: re-lists every folder listed so far. */
  async function refreshAll() {
    for (const folder of [...loadedRef.current]) await loadFolder(folder, true);
  }

  return {
    nodes,
    expandedIds,
    /** The folders listed so far — the ones worth watching for outside changes. */
    loadedFolders: loadedIds,
    refreshPaths,
    onExpandedChange,
    createFileIn,
    createDirIn,
    renameNode,
    removeNode,
    reload: (id: string) => loadFolder(id).then(() => {}),
    expandSubtree,
    revealPath,
    refreshAll,
  };
}

function kindOf(nodes: FileTreeNode[], id: string): FileTreeNode["kind"] | undefined {
  for (const node of nodes) {
    if (node.id === id) return node.kind;
    const inner = node.children ? kindOf(node.children, id) : undefined;
    if (inner) return inner;
  }
  return undefined;
}

function defaultOnError(error: unknown, path: string) {
  console.error(`failed to list ${path}:`, error);
}
