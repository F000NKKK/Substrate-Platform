import { useEffect, useMemo, useRef, useState, type KeyboardEvent, type MouseEvent as ReactMouseEvent } from "react";
import type { TreeNode, TreeProps } from "./Tree";

export interface FlatEntry<T> {
  node: TreeNode<T>;
  depth: number;
  parentId: string | null;
}

function flattenVisible<T>(
  nodes: TreeNode<T>[],
  expandedIds: Set<string>,
  depth: number,
  parentId: string | null
): FlatEntry<T>[] {
  const out: FlatEntry<T>[] = [];
  for (const node of nodes) {
    out.push({ node, depth, parentId });
    if (node.children?.length && expandedIds.has(node.id)) {
      out.push(...flattenVisible(node.children, expandedIds, depth + 1, node.id));
    }
  }
  return out;
}

/**
 * All of Tree's state and behavior — expand/collapse, selection (click,
 * ctrl/cmd-toggle, shift-range), roving-tabindex keyboard nav, and the
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
}: TreeProps<T>) {
  const [internalExpanded, setInternalExpanded] = useState<Set<string>>(defaultExpandedIds ?? new Set());
  const [internalSelected, setInternalSelected] = useState<Set<string>>(defaultSelectedIds ?? new Set());
  const [activeId, setActiveId] = useState<string | null>(null);
  const [contextMenu, setContextMenu] = useState<{ node: TreeNode<T>; x: number; y: number } | null>(null);
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
    if (activeId === null && flat.length > 0) setActiveId(flat[0].node.id);
  }, [activeId, flat]);

  useEffect(() => {
    if (!contextMenu) return;
    function close(e: Event) {
      if (e instanceof KeyboardEvent && e.key !== "Escape") return;
      setContextMenu(null);
    }
    window.addEventListener("pointerdown", close);
    window.addEventListener("keydown", close);
    window.addEventListener("scroll", close, true);
    window.addEventListener("resize", close);
    return () => {
      window.removeEventListener("pointerdown", close);
      window.removeEventListener("keydown", close);
      window.removeEventListener("scroll", close, true);
      window.removeEventListener("resize", close);
    };
  }, [contextMenu]);

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

  function selectNode(node: TreeNode<T>, opts: { toggle?: boolean; range?: boolean }) {
    let next: Set<string>;
    if (opts.range && lastAnchorRef.current !== null) {
      const anchorIndex = flatIndexById.get(lastAnchorRef.current) ?? 0;
      const targetIndex = flatIndexById.get(node.id) ?? 0;
      const [lo, hi] = [anchorIndex, targetIndex].sort((a, b) => a - b);
      next = new Set(activeSelected);
      for (let i = lo; i <= hi; i++) next.add(flat[i].node.id);
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
    if (node.children) toggleExpanded(node.id);
  }

  function handleKeyDown(e: KeyboardEvent, entry: FlatEntry<T>) {
    const index = flatIndexById.get(entry.node.id) ?? 0;
    switch (e.key) {
      case "ArrowDown": {
        e.preventDefault();
        const next = flat[Math.min(index + 1, flat.length - 1)];
        if (next) setActiveId(next.node.id);
        break;
      }
      case "ArrowUp": {
        e.preventDefault();
        const prev = flat[Math.max(index - 1, 0)];
        if (prev) setActiveId(prev.node.id);
        break;
      }
      case "ArrowRight":
        e.preventDefault();
        if (entry.node.children) {
          if (!activeExpanded.has(entry.node.id)) toggleExpanded(entry.node.id);
          else if (flat[index + 1]?.parentId === entry.node.id) setActiveId(flat[index + 1].node.id);
        }
        break;
      case "ArrowLeft":
        e.preventDefault();
        if (entry.node.children && activeExpanded.has(entry.node.id)) {
          toggleExpanded(entry.node.id);
        } else if (entry.parentId) {
          setActiveId(entry.parentId);
        }
        break;
      case "Enter":
        e.preventDefault();
        if (entry.node.children) toggleExpanded(entry.node.id);
        else onActivate?.(entry.node);
        break;
      case " ":
        e.preventDefault();
        selectNode(entry.node, { toggle: true });
        break;
    }
  }

  function openContextMenu(node: TreeNode<T>, e: ReactMouseEvent) {
    if (!getMenuItems) return;
    e.preventDefault();
    setContextMenu({ node, x: e.clientX, y: e.clientY });
  }

  return {
    flat,
    activeId,
    setActiveId,
    activeExpanded,
    activeSelected,
    toggleExpanded,
    handleRowClick,
    handleKeyDown,
    contextMenu,
    openContextMenu,
    closeContextMenu: () => setContextMenu(null),
  };
}
