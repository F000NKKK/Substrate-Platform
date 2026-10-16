import type { FileTreeNode } from "./FileTree";
import type { DirEntry } from "./useDirectoryTree";

// Searching and filtering a file tree, as Solution Explorer's search box and
// its Pending Changes / Open Files filters do: the tree narrows to what
// matches plus the folders leading to it, opened, with the matched text
// marked. Pure functions — the product decides what "matches" means.

/** Highlighted spans of a name, `[start, end)` — what `HighlightedText` takes. */
export type TextRanges = [number, number][];

/** Every case-insensitive occurrence of `query` in `text`, or `null` when there's none (or `query` is blank). */
export function textMatchRanges(text: string, query: string): TextRanges | null {
  const needle = query.trim().toLowerCase();
  if (!needle) return null;
  const hay = text.toLowerCase();
  const ranges: TextRanges = [];
  for (let at = hay.indexOf(needle); at >= 0; at = hay.indexOf(needle, at + needle.length)) ranges.push([at, at + needle.length]);
  return ranges.length > 0 ? ranges : null;
}

/** Whether a node matches — `true`, or the spans of its name to mark; `false`/`null` when it doesn't. */
export type FileTreeMatch = (node: FileTreeNode) => TextRanges | boolean | null;

export interface FilteredFileTree {
  nodes: FileTreeNode[];
  /** The matched nodes' marked spans (none for a match given as plain `true`). */
  highlights: Map<string, TextRanges>;
  /** The folders (and files with nested ones) a match is under — open them to show it. */
  expandedIds: Set<string>;
  /** How many nodes matched. */
  count: number;
}

/**
 * Narrows `nodes` to the ones `match` accepts and their ancestors — folders
 * and files with nested files alike. A matched folder keeps its whole
 * contents (collapsed) when nothing under it matches, so it can still be
 * browsed; otherwise only the matching part of it is kept, opened.
 */
export function filterFileTree(nodes: FileTreeNode[], match: FileTreeMatch): FilteredFileTree {
  const highlights = new Map<string, TextRanges>();
  const expandedIds = new Set<string>();
  let count = 0;

  function walk(list: FileTreeNode[]): FileTreeNode[] {
    const out: FileTreeNode[] = [];
    for (const node of list) {
      const hit = match(node);
      if (hit) {
        count++;
        if (Array.isArray(hit)) highlights.set(node.id, hit);
      }
      const key = node.kind === "folder" ? "children" : "nested";
      const inner = node[key];
      const kept = inner ? walk(inner) : [];
      if (kept.length > 0) {
        expandedIds.add(node.id);
        out.push({ ...node, [key]: kept });
      } else if (hit) out.push(node);
    }
    return out;
  }

  return { nodes: walk(nodes), highlights, expandedIds, count };
}

function parentPath(path: string): string {
  return path.replace(/[/\\][^/\\]*$/, "");
}

/**
 * A whole tree at once from a flat listing of everything under `rootPath`
 * (a recursive walk) — what search and filters run over, where the lazily
 * listed tree only knows the folders opened so far. Entries whose folder
 * isn't in the listing are left out; folders come first, then files, each
 * by name ignoring case — the order `list_dir` gives.
 */
export function fileTreeFromEntries(rootPath: string, rootName: string, entries: DirEntry[]): FileTreeNode[] {
  const root = rootPath.replace(/[/\\]+$/, "");
  const byParent = new Map<string, DirEntry[]>();
  for (const entry of entries) {
    const parent = parentPath(entry.path);
    const list = byParent.get(parent);
    if (list) list.push(entry);
    else byParent.set(parent, [entry]);
  }
  const build = (folder: string): FileTreeNode[] =>
    (byParent.get(folder) ?? [])
      .slice()
      .sort((a, b) => (a.kind === b.kind ? a.name.toLowerCase().localeCompare(b.name.toLowerCase()) : a.kind === "dir" ? -1 : 1))
      .map((e) => (e.kind === "dir" ? { id: e.path, name: e.name, kind: "folder", children: build(e.path) } : { id: e.path, name: e.name, kind: "file" }));
  return [{ id: rootPath, name: rootName, kind: "folder", children: build(root) }];
}
