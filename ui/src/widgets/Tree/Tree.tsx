import type { ReactNode } from "react";
import { Icon } from "../../infra/icons";
import { ContextMenu } from "../ContextMenu";
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
  onSelect: (node: TreeNode<T>) => void;
  destructive?: boolean;
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
  className?: string;
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
export function Tree<T>({ nodes, renderIcon, getMenuItems, className, ...rest }: TreeProps<T>) {
  const tree = useTree({ nodes, getMenuItems, ...rest });

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
            tabIndex={tree.activeId === node.id ? 0 : -1}
            style={{ height: ROW_HEIGHT, paddingLeft: depth * INDENT + 6 }}
            onFocus={() => tree.setActiveId(node.id)}
            onKeyDown={(e) => tree.handleKeyDown(e, entry)}
            onClick={(e) => tree.handleRowClick(node, e)}
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
    <div className={["sp-tree", className].filter(Boolean).join(" ")} role="tree">
      {renderNodes(nodes, 0, null)}
      <ContextMenu
        target={tree.contextMenu && tree.contextMenu.mode === "viewport" ? { mode: "viewport", x: tree.contextMenu.x, y: tree.contextMenu.y } : null}
        onClose={tree.closeContextMenu}
        items={
          tree.contextMenu
            ? getMenuItems!(tree.contextMenu.node).map((item) => ({
                label: item.label,
                destructive: item.destructive,
                onSelect: () => item.onSelect(tree.contextMenu!.node),
              }))
            : []
        }
      />
    </div>
  );
}
