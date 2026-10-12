import { useEffect, useRef, useState, type PointerEvent as ReactPointerEvent, type ReactNode } from "react";
import { Icon } from "../../infra/icons";
import { ViewportPortal } from "../../infra/portal";
import { ContextMenu, type ContextMenuItem } from "../ContextMenu";
import { useTree } from "./useTree";
import type { FlatEntry } from "./useTree";
import "./Tree.css";

export interface TreeNode<T> {
  id: string;
  label: ReactNode;
  /** Present (even as `[]`) marks this node as an expandable branch; omit for a leaf. */
  children?: TreeNode<T>[];
  data: T;
}

export interface TreeMenuItem<T> {
  label: string;
  /** Not needed on an item that only opens a `submenu`. */
  onSelect?: (node: TreeNode<T>) => void;
  destructive?: boolean;
  disabled?: boolean;
  checked?: boolean;
  separatorBefore?: boolean;
  submenu?: TreeMenuItem<T>[];
  /** A keybinding hint shown in the row — display only. */
  shortcut?: string;
}

function toContextMenuItems<T>(items: TreeMenuItem<T>[], node: TreeNode<T>): ContextMenuItem[] {
  return items.map(({ onSelect, submenu, ...item }) => ({
    ...item,
    onSelect: onSelect && (() => onSelect(node)),
    submenu: submenu && toContextMenuItems(submenu, node),
  }));
}

export interface TreeProps<T> {
  nodes: TreeNode<T>[];
  /** Icon slot per node; omit for a plain indented list with no icons. */
  renderIcon?: (node: TreeNode<T>, expanded: boolean) => ReactNode;
  expandedIds?: Set<string>;
  defaultExpandedIds?: Set<string>;
  onExpandedChange?: (ids: Set<string>) => void;
  selectedIds?: Set<string>;
  defaultSelectedIds?: Set<string>;
  onSelectionChange?: (ids: Set<string>) => void;
  /** Enter (or double-click) on a leaf; branches expand/collapse instead of activating. */
  onActivate?: (node: TreeNode<T>) => void;
  /** Omit to disable the right-click dropdown entirely. */
  getMenuItems?: (node: TreeNode<T>) => TreeMenuItem<T>[];
  /**
   * Drag and drop within the tree: dragging a row (or the selection it's
   * part of) onto a branch — or a leaf, meaning its parent branch — calls
   * this with the dragged ids and the branch; `copy` while Ctrl is held.
   * Omit to make rows undraggable.
   */
  onDropNodes?: (ids: string[], target: TreeNode<T>, copy: boolean) => void;
  /** Whether a row can be dragged at all (default: every row). */
  canDrag?: (node: TreeNode<T>) => boolean;
  className?: string;
}

/** How far (px) the pointer travels before a press becomes a drag. */
const DRAG_THRESHOLD = 5;
/** Hovering a collapsed branch this long while dragging expands it. */
const DRAG_EXPAND_MS = 700;

interface TreeDrag {
  ids: string[];
  label: ReactNode;
  x: number;
  y: number;
  copy: boolean;
  /** The branch a drop would land in, if it's a valid one. */
  targetId: string | null;
}

/** `id` itself or anything under it among `nodes`. */
function subtreeContains<T>(nodes: TreeNode<T>[], rootId: string, id: string): boolean {
  const root = findTreeNode(nodes, rootId);
  return !!root && (root.id === id || !!findTreeNode(root.children ?? [], id));
}

function findTreeNode<T>(nodes: TreeNode<T>[], id: string): TreeNode<T> | null {
  for (const node of nodes) {
    if (node.id === id) return node;
    const inner = node.children ? findTreeNode(node.children, id) : null;
    if (inner) return inner;
  }
  return null;
}

/** The branch a drop on `id` means — the node itself, or a leaf's parent. */
function dropBranch<T>(nodes: TreeNode<T>[], id: string): TreeNode<T> | null {
  const node = findTreeNode(nodes, id);
  if (!node) return null;
  return node.children ? node : findParent(nodes, id);
}

function findParent<T>(nodes: TreeNode<T>[], id: string): TreeNode<T> | null {
  for (const node of nodes) {
    if (node.children?.some((c) => c.id === id)) return node;
    const inner = node.children ? findParent(node.children, id) : null;
    if (inner) return inner;
  }
  return null;
}

const ROW_HEIGHT = 22;
const INDENT = 16;

/**
 * A generic nested tree — the same nativeness level as `DataGrid`, just for
 * hierarchical rather than tabular data. Animated expand/collapse, a
 * right-click dropdown per node (only when `getMenuItems` is given), and
 * roving-tabindex keyboard nav (Left/Right expand/collapse or step to
 * parent/child, Up/Down move, Enter activates a leaf or toggles a branch).
 * Every branch's children stay mounted — collapse only animates their
 * height to zero — so this isn't virtualized; fine for a project tree, not
 * a listing in the tens of thousands. `FileTree` is a thin config layer on
 * top of this one.
 */
export function Tree<T>({ nodes, renderIcon, getMenuItems, onDropNodes, canDrag, className, ...rest }: TreeProps<T>) {
  const tree = useTree({ nodes, getMenuItems, ...rest });
  const [drag, setDrag] = useState<TreeDrag | null>(null);
  const dragRef = useRef<TreeDrag | null>(null);
  dragRef.current = drag;
  /** A drag just ended — swallow the click that follows its pointerup. */
  const suppressClickRef = useRef(false);
  const treeRef = useRef(tree);
  treeRef.current = tree;

  // Hovering a collapsed branch for a moment while dragging opens it, as in Explorer.
  const hoverId = drag?.targetId ?? null;
  useEffect(() => {
    if (!hoverId) return;
    const node = findTreeNode(nodes, hoverId);
    if (!node?.children || treeRef.current.activeExpanded.has(hoverId)) return;
    const timer = setTimeout(() => treeRef.current.toggleExpanded(hoverId), DRAG_EXPAND_MS);
    return () => clearTimeout(timer);
  }, [hoverId, nodes]);

  function handleRowPointerDown(node: TreeNode<T>, e: ReactPointerEvent<HTMLDivElement>) {
    if (!onDropNodes || e.button !== 0 || (canDrag && !canDrag(node))) return;
    const startX = e.clientX;
    const startY = e.clientY;
    const ids = tree.activeSelected.has(node.id) ? [...tree.activeSelected] : [node.id];
    const label = ids.length > 1 ? `${ids.length}` : node.label;
    let started = false;

    function targetAt(x: number, y: number): string | null {
      const row = document.elementFromPoint(x, y)?.closest<HTMLElement>(".sp-tree-row[data-node-id]");
      const branch = row ? dropBranch(nodes, row.dataset.nodeId!) : null;
      if (!branch) return null;
      // Not into itself or its own subtree, and not where it already is.
      if (ids.some((id) => subtreeContains(nodes, id, branch.id))) return null;
      if (ids.every((id) => findParent(nodes, id)?.id === branch.id)) return null;
      return branch.id;
    }
    function onMove(ev: PointerEvent) {
      if (!started) {
        if (Math.hypot(ev.clientX - startX, ev.clientY - startY) < DRAG_THRESHOLD) return;
        started = true;
      }
      setDrag({ ids, label, x: ev.clientX, y: ev.clientY, copy: ev.ctrlKey, targetId: targetAt(ev.clientX, ev.clientY) });
    }
    function onUp(ev: PointerEvent) {
      window.removeEventListener("pointermove", onMove);
      window.removeEventListener("pointerup", onUp);
      window.removeEventListener("keydown", onKey);
      const current = dragRef.current;
      setDrag(null);
      if (!started) return;
      suppressClickRef.current = true;
      setTimeout(() => (suppressClickRef.current = false), 0);
      const targetId = current ? targetAt(ev.clientX, ev.clientY) : null;
      const target = targetId ? findTreeNode(nodes, targetId) : null;
      if (target) onDropNodes?.(ids, target, ev.ctrlKey);
    }
    function onKey(ev: KeyboardEvent) {
      if (ev.key !== "Escape") return;
      // Cancel: forget the drag, keep listening for the pointerup to clean up.
      started = false;
      setDrag(null);
    }
    window.addEventListener("pointermove", onMove);
    window.addEventListener("pointerup", onUp);
    window.addEventListener("keydown", onKey);
  }

  function renderNodes(list: TreeNode<T>[], depth: number, parentId: string | null): ReactNode {
    return list.map((node) => {
      const expanded = !!node.children && tree.activeExpanded.has(node.id);
      const selected = tree.activeSelected.has(node.id);
      const entry: FlatEntry<T> = { node, depth, parentId };
      return (
        <div key={node.id} className="sp-tree-branch">
          <div
            className="sp-tree-row"
            role="treeitem"
            aria-selected={selected}
            aria-expanded={node.children ? expanded : undefined}
            data-selected={selected || undefined}
            data-node-id={node.id}
            data-drop-target={drag?.targetId === node.id || undefined}
            tabIndex={tree.activeId === node.id ? 0 : -1}
            style={{ height: ROW_HEIGHT, paddingLeft: depth * INDENT + 6 }}
            onFocus={() => tree.setActiveId(node.id)}
            onKeyDown={(e) => tree.handleKeyDown(e, entry)}
            onPointerDown={(e) => handleRowPointerDown(node, e)}
            onClick={(e) => {
              if (!suppressClickRef.current) tree.handleRowClick(node, e);
            }}
            onDoubleClick={() => {
              if (!node.children) rest.onActivate?.(node);
            }}
            onContextMenu={(e) => tree.openContextMenu(node, e)}
          >
            {node.children ? (
              <span className="sp-tree-chevron" data-expanded={expanded || undefined}>
                <Icon name="chevronRight" size={13} />
              </span>
            ) : (
              <span className="sp-tree-chevron-spacer" />
            )}
            {renderIcon && <span className="sp-tree-icon">{renderIcon(node, expanded)}</span>}
            <span className="sp-tree-label">{node.label}</span>
          </div>
          {node.children && node.children.length > 0 && (
            <div className="sp-tree-children" data-expanded={expanded || undefined}>
              <div className="sp-tree-children-inner">{renderNodes(node.children, depth + 1, node.id)}</div>
            </div>
          )}
        </div>
      );
    });
  }

  return (
    <div className={["sp-tree", className].filter(Boolean).join(" ")} role="tree" data-dragging={drag ? true : undefined}>
      {renderNodes(nodes, 0, null)}
      {drag && (
        <ViewportPortal>
          <div className="sp-tree-drag-ghost" style={{ left: drag.x + 12, top: drag.y + 8 }} data-invalid={drag.targetId ? undefined : true}>
            {drag.copy && drag.targetId && <span className="sp-tree-drag-copy">+</span>}
            {drag.label}
          </div>
        </ViewportPortal>
      )}
      <ContextMenu
        target={tree.contextMenu && tree.contextMenu.mode === "viewport" ? { mode: "viewport", x: tree.contextMenu.x, y: tree.contextMenu.y } : null}
        onClose={tree.closeContextMenu}
        items={
          tree.contextMenu
            ? toContextMenuItems(getMenuItems!(tree.contextMenu.node), tree.contextMenu.node)
            : []
        }
      />
    </div>
  );
}
