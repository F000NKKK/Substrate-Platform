import { useEffect, useMemo, useRef, useState, type KeyboardEvent, type MouseEvent as ReactMouseEvent } from "react";
import type { TreeNode, TreeProps } from "./Tree";
import { useContextMenu } from "../ContextMenu";
import { eventKey } from "../../infra/keys";
import { branchIdsUnder, navigationTarget, rangeIds, typeAheadIndex } from "./treeNav";

/** Keys typed within this long of each other make one type-ahead prefix. */
const TYPE_AHEAD_MS = 700;
/** How far PageUp/PageDown move. */
const PAGE_ROWS = 10;

export interface FlatEntry<T> {
  node: TreeNode<T>;
  depth: number;
  parentId: string | null;
}

/** The rows a tree shows: every node, and the children of expanded ones, depth-first. */
export function flattenVisible<T>(nodes: TreeNode<T>[], expandedIds: Set<string>, depth = 0, parentId: string | null = null, out: FlatEntry<T>[] = []): FlatEntry<T>[] {
  for (const node of nodes) {
    out.push({ node, depth, parentId });
    if (node.children?.length && expandedIds.has(node.id)) flattenVisible(node.children, expandedIds, depth + 1, node.id, out);
  }
  return out;
}

/**
 * All of Tree's state and behavior — expand/collapse, selection (click,
 * ctrl/cmd-toggle, shift-range), roving-tabindex keyboard nav as in
 * Windows' and Visual Studio's trees (arrows select, Shift extends, Ctrl
 * only moves the focus; Home/End/PageUp/PageDown; Left/Right collapse,
 * expand or step to parent/child; `*` expands everything under a node;
 * Ctrl+A; type-ahead by name), and the
 * right-click context menu — with no rendering. `Tree.tsx` is the view over
 * this; a product that needs a differently-rendered tree can consume this
 * hook directly instead of forking the component.
 */
export function useTree<T>({
  nodes,
  expandedIds,
  defaultExpandedIds,
  onExpandedChange,
  selectedIds,
  defaultSelectedIds,
  onSelectionChange,
  onActivate,
  getMenuItems,
  onExpandAll,
}: TreeProps<T>) {
  const [internalExpanded, setInternalExpanded] = useState<Set<string>>(defaultExpandedIds ?? new Set());
  const [internalSelected, setInternalSelected] = useState<Set<string>>(defaultSelectedIds ?? new Set());
  const [activeId, setActiveId] = useState<string | null>(null);
  const menu = useContextMenu<TreeNode<T>>();
  const lastAnchorRef = useRef<string | null>(null);

  const activeExpanded = expandedIds ?? internalExpanded;
  const activeSelected = selectedIds ?? internalSelected;

  const flat = useMemo(() => flattenVisible(nodes, activeExpanded, 0, null), [nodes, activeExpanded]);
  const flatIndexById = useMemo(() => {
    const map = new Map<string, number>();
    flat.forEach((entry, i) => map.set(entry.node.id, i));
    return map;
  }, [flat]);

  useEffect(() => {
    // The first row — or, once the active one is gone (filtered out, its folder collapsed or deleted), the nearest selected one.
    if (activeId !== null && flatIndexById.has(activeId)) return;
    const selected = flat.find((e) => activeSelected.has(e.node.id));
    const next = selected ?? flat[0];
    if (next && next.node.id !== activeId) setActiveId(next.node.id);
  }, [activeId, flat, flatIndexById, activeSelected]);

  /** A keyboard move wants the DOM focus on the new active row — `Tree` moves it once that row renders. */
  const focusRequestRef = useRef<string | null>(null);
  /** Type-ahead's typed prefix, and when its last key came. */
  const typedRef = useRef({ text: "", at: 0 });

  function commitExpanded(next: Set<string>) {
    if (expandedIds === undefined) setInternalExpanded(next);
    onExpandedChange?.(next);
  }

  function commitSelected(next: Set<string>) {
    if (selectedIds === undefined) setInternalSelected(next);
    onSelectionChange?.(next);
  }

  function toggleExpanded(id: string) {
    const next = new Set(activeExpanded);
    if (next.has(id)) next.delete(id);
    else next.add(id);
    commitExpanded(next);
  }

  function setExpanded(id: string, open: boolean) {
    if (activeExpanded.has(id) !== open) toggleExpanded(id);
  }

  /** `*`: the node and every branch under it open — the product's own way (`onExpandAll`, e.g. loading folders not listed yet) when it has one. */
  function expandAllUnder(node: TreeNode<T>) {
    if (onExpandAll) return onExpandAll(node);
    commitExpanded(new Set([...activeExpanded, ...branchIdsUnder(node)]));
  }

  function selectNode(node: TreeNode<T>, opts: { toggle?: boolean; range?: boolean }) {
    let next: Set<string>;
    if (opts.range && lastAnchorRef.current !== null) {
      const anchorIndex = flatIndexById.get(lastAnchorRef.current) ?? 0;
      const targetIndex = flatIndexById.get(node.id) ?? 0;
      // A Ctrl+Shift range adds to the selection; a plain Shift one replaces it.
      next = new Set([...(opts.toggle ? activeSelected : []), ...rangeIds(flat, anchorIndex, targetIndex)]);
    } else if (opts.toggle) {
      next = new Set(activeSelected);
      if (next.has(node.id)) next.delete(node.id);
      else next.add(node.id);
      lastAnchorRef.current = node.id;
    } else {
      next = new Set([node.id]);
      lastAnchorRef.current = node.id;
    }
    commitSelected(next);
  }

  function handleRowClick(node: TreeNode<T>, e: ReactMouseEvent) {
    setActiveId(node.id);
    selectNode(node, { toggle: e.metaKey || e.ctrlKey, range: e.shiftKey });
    if (node.children && !node.openable && !e.shiftKey && !(e.metaKey || e.ctrlKey)) toggleExpanded(node.id);
  }

  /** Makes row `index` the active one — selecting it (Shift: the range up to it) unless `focusOnly` (Ctrl, as in Windows' trees). */
  function moveTo(index: number, opts: { focusOnly?: boolean; range?: boolean } = {}) {
    const target = flat[index];
    if (!target) return;
    focusRequestRef.current = target.node.id;
    setActiveId(target.node.id);
    if (opts.focusOnly) return;
    selectNode(target.node, { range: opts.range });
  }

  /** Type-ahead: a printable key jumps to the next row starting with what's been typed lately. */
  function typeAhead(e: KeyboardEvent, index: number): boolean {
    if (e.key.length !== 1 || e.ctrlKey || e.metaKey || e.altKey) return false;
    const now = Date.now();
    const typed = typedRef.current;
    const text = now - typed.at < TYPE_AHEAD_MS ? typed.text + e.key : e.key;
    if (text.trim() === "") return false;
    typedRef.current = { text, at: now };
    const found = typeAheadIndex(flat, index, text);
    if (found >= 0) moveTo(found);
    return true;
  }

  function handleKeyDown(e: KeyboardEvent, entry: FlatEntry<T>) {
    const index = flatIndexById.get(entry.node.id) ?? 0;
    const ctrl = e.ctrlKey || e.metaKey;
    const target = navigationTarget(e.key, index, flat.length, PAGE_ROWS);
    if (target !== null && !e.altKey) {
      e.preventDefault();
      moveTo(target, { focusOnly: ctrl && !e.shiftKey, range: e.shiftKey });
      return;
    }
    const branch = !!entry.node.children;
    const expanded = branch && activeExpanded.has(entry.node.id);
    switch (e.key) {
      case "ArrowRight":
        e.preventDefault();
        if (branch && !expanded) setExpanded(entry.node.id, true);
        else if (expanded && flat[index + 1]?.parentId === entry.node.id) moveTo(index + 1);
        return;
      case "ArrowLeft":
        e.preventDefault();
        if (expanded) setExpanded(entry.node.id, false);
        else if (entry.parentId) moveTo(flatIndexById.get(entry.parentId) ?? index);
        return;
      case "Enter":
        if (e.altKey || ctrl || e.shiftKey) return;
        e.preventDefault();
        if (branch && !entry.node.openable) toggleExpanded(entry.node.id);
        else onActivate?.(entry.node);
        return;
      case " ":
        e.preventDefault();
        selectNode(entry.node, { toggle: ctrl });
        return;
      case "*":
        e.preventDefault();
        expandAllUnder(entry.node);
        return;
    }
    // The numeric keypad's + / − open and close a branch (the main row's are type-ahead text).
    if (e.code === "NumpadAdd" || e.code === "NumpadSubtract") {
      e.preventDefault();
      if (branch) setExpanded(entry.node.id, e.code === "NumpadAdd");
      return;
    }
    if (ctrl && !e.shiftKey && !e.altKey && eventKey(e) === "a") {
      e.preventDefault();
      commitSelected(new Set(flat.map((f) => f.node.id)));
      return;
    }
    if (typeAhead(e, index)) e.preventDefault();
  }

  function openContextMenu(node: TreeNode<T>, e: ReactMouseEvent) {
    if (!getMenuItems) return;
    menu.openAtPoint(node, e);
  }

  return {
    flat,
    activeId,
    setActiveId,
    activeExpanded,
    activeSelected,
    toggleExpanded,
    expandAllUnder,
    focusRequestRef,
    handleRowClick,
    handleKeyDown,
    contextMenu: menu.target,
    openContextMenu,
    closeContextMenu: menu.close,
  };
}
