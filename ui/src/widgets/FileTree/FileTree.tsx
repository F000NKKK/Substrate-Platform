import { useMemo, type ReactNode } from "react";
import { Tree, type TreeNode } from "../Tree";
import { Icon } from "../../infra/icons";

export interface FileTreeNode {
  id: string;
  name: string;
  kind: "folder" | "file";
  children?: FileTreeNode[];
  /** Overrides the default folder/folderOpen/file glyph for this node. */
  icon?: ReactNode;
}

export interface FileTreeMenuItem {
  label: string;
  onSelect: (node: FileTreeNode) => void;
  destructive?: boolean;
}

export interface FileTreeProps {
  nodes: FileTreeNode[];
  expandedIds?: Set<string>;
  defaultExpandedIds?: Set<string>;
  onExpandedChange?: (ids: Set<string>) => void;
  selectedIds?: Set<string>;
  defaultSelectedIds?: Set<string>;
  onSelectionChange?: (ids: Set<string>) => void;
  onActivate?: (node: FileTreeNode) => void;
  getMenuItems?: (node: FileTreeNode) => FileTreeMenuItem[];
  className?: string;
}

function toTreeNodes(nodes: FileTreeNode[]): TreeNode<FileTreeNode>[] {
  return nodes.map((node) => ({
    id: node.id,
    label: node.name,
    data: node,
    children: node.kind === "folder" ? toTreeNodes(node.children ?? []) : undefined,
  }));
}

/** `Tree` configured for a file explorer: folder/file (open/closed) icons on top of its generic expand/collapse, selection, and context-menu behavior. */
export function FileTree({ nodes, onActivate, getMenuItems, ...rest }: FileTreeProps) {
  const treeNodes = useMemo(() => toTreeNodes(nodes), [nodes]);

  return (
    <Tree
      nodes={treeNodes}
      renderIcon={(node, expanded) =>
        node.data.icon ?? <Icon name={node.data.kind === "folder" ? (expanded ? "folderOpen" : "folder") : "file"} size={15} />
      }
      onActivate={(node) => onActivate?.(node.data)}
      getMenuItems={
        getMenuItems &&
        ((node: TreeNode<FileTreeNode>) =>
          getMenuItems(node.data).map((item) => ({
            label: item.label,
            destructive: item.destructive,
            onSelect: () => item.onSelect(node.data),
          })))
      }
      {...rest}
    />
  );
}
