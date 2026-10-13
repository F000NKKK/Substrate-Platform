import { useMemo, type ReactNode } from "react";
import { Tree, type TreeMenuItem, type TreeNode } from "../Tree";
import { Icon } from "../../infra/icons";

export interface FileTreeNode {
  id: string;
  name: string;
  kind: "folder" | "file";
  children?: FileTreeNode[];
  /** A file's dependent files, shown under it (see `nestFiles`). */
  nested?: FileTreeNode[];
  /** Overrides the default folder/folderOpen/file glyph for this node. */
  icon?: ReactNode;
  /** A generated file (Visual Studio's `MainWindow.g.cs`) — shown greyed and in italics. */
  generated?: boolean;
}

export interface FileTreeMenuItem {
  label: string;
  /** Not needed on an item that only opens a `submenu`. */
  onSelect?: (node: FileTreeNode) => void;
  destructive?: boolean;
  disabled?: boolean;
  checked?: boolean;
  separatorBefore?: boolean;
  submenu?: FileTreeMenuItem[];
  /** A keybinding hint shown in the row — display only. */
  shortcut?: string;
}

function toTreeMenuItems(items: FileTreeMenuItem[]): TreeMenuItem<FileTreeNode>[] {
  return items.map(({ onSelect, submenu, ...item }) => ({
    ...item,
    onSelect: onSelect && ((node: TreeNode<FileTreeNode>) => onSelect(node.data)),
    submenu: submenu && toTreeMenuItems(submenu),
  }));
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
  /** Files/folders (by id) dragged onto `folder` — moved, or copied with Ctrl. Omit to disable drag and drop. */
  onDropNodes?: (ids: string[], folder: FileTreeNode, copy: boolean) => void;
  /** Whether a node can be dragged (default: every one). */
  canDrag?: (node: FileTreeNode) => boolean;
  className?: string;
}

function toTreeNodes(nodes: FileTreeNode[]): TreeNode<FileTreeNode>[] {
  return nodes.map((node) => ({
    id: node.id,
    label: node.generated ? <span style={{ fontStyle: "italic", opacity: 0.65 }}>{node.name}</span> : node.name,
    data: node,
    children: node.kind === "folder" ? toTreeNodes(node.children ?? []) : node.nested?.length ? toTreeNodes(node.nested) : undefined,
    openable: node.kind === "file" && !!node.nested?.length,
  }));
}

/**
 * File nesting, as Visual Studio does for `Form1.cs` ▸ `Form1.Designer.cs`
 * and `Form1.resx`: every file `parentOf` names a sibling for (by name —
 * `parentOf(name, siblings)`, `siblings` lowercased) moves under that file's
 * `nested`. Folders are left alone; recurses into their children.
 */
export function nestFiles(nodes: FileTreeNode[], parentOf: (name: string, siblings: Set<string>) => string | null): FileTreeNode[] {
  const files = new Map(nodes.filter((n) => n.kind === "file").map((n) => [n.name.toLowerCase(), n]));
  const siblings = new Set(files.keys());
  const nestedUnder = new Map<string, FileTreeNode[]>();
  const top: FileTreeNode[] = [];
  for (const node of nodes) {
    if (node.kind === "folder") {
      top.push(node.children ? { ...node, children: nestFiles(node.children, parentOf) } : node);
      continue;
    }
    const parent = parentOf(node.name, siblings)?.toLowerCase();
    if (parent && parent !== node.name.toLowerCase() && files.has(parent)) {
      const nested = nestedUnder.get(parent);
      if (nested) nested.push(node);
      else nestedUnder.set(parent, [node]);
    } else top.push(node);
  }
  return top.map((node) => {
    const nested = node.kind === "file" ? nestedUnder.get(node.name.toLowerCase()) : undefined;
    return nested ? { ...node, nested } : node;
  });
}

/** `Tree` configured for a file explorer: folder/file (open/closed) icons on top of its generic expand/collapse, selection, and context-menu behavior. */
export function FileTree({ nodes, onActivate, getMenuItems, onDropNodes, canDrag, ...rest }: FileTreeProps) {
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
        ((node: TreeNode<FileTreeNode>) => toTreeMenuItems(getMenuItems(node.data)))
      }
      onDropNodes={onDropNodes && ((ids, target, copy) => onDropNodes(ids, target.data, copy))}
      canDrag={canDrag && ((node) => canDrag(node.data))}
      canDropInto={(node) => node.data.kind === "folder"}
      {...rest}
    />
  );
}
