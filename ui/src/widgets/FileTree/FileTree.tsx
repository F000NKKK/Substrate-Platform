import {
  useEffect,
  useMemo,
  useRef,
  useState,
  type KeyboardEvent,
  type MouseEvent as ReactMouseEvent,
  type ReactNode,
} from "react";
import { Icon } from "../icons";
import { Tab } from "./Tab";
import "./FileTree.css";

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
  /** Double-click (or Enter) on a file; folders expand/collapse instead of activating. */
  onActivate?: (node: FileTreeNode) => void;
  /** Omit to disable the right-click dropdown entirely. */
  getMenuItems?: (node: FileTreeNode) => FileTreeMenuItem[];
  className?: string;
}

interface FlatEntry {
  node: FileTreeNode;
  depth: number;
  parentId: string | null;
}

function flattenVisible(nodes: FileTreeNode[], expandedIds: Set<string>, depth: number, parentId: string | null): FlatEntry[] {
  const out: FlatEntry[] = [];
  for (const node of nodes) {
    out.push({ node, depth, parentId });
    if (node.kind === "folder" && node.children?.length && expandedIds.has(node.id)) {
      out.push(...flattenVisible(node.children, expandedIds, depth + 1, node.id));
    }
  }
  return out;
}

const ROW_HEIGHT = 22;
const INDENT = 16;

/**
 * A VS-style nested file tree: animated expand/collapse, a right-click
 * dropdown per node (only when `getMenuItems` is given), and roving-tabindex
 * keyboard navigation (Left/Right expand/collapse or step to parent/child,
 * Up/Down move, Enter activates a file or toggles a folder). Every folder's
 * children are always mounted — collapse only animates their height to
 * zero — so this isn't virtualized; fine for a project tree, not for a
 * directory listing in the tens of thousands.
 */
export function FileTree({
  nodes,
  expandedIds,
  defaultExpandedIds,
  onExpandedChange,
  selectedIds,
  defaultSelectedIds,
  onSelectionChange,
  onActivate,
  getMenuItems,
  className,
}: FileTreeProps) {
  const [internalExpanded, setInternalExpanded] = useState<Set<string>>(defaultExpandedIds ?? new Set());
  const [internalSelected, setInternalSelected] = useState<Set<string>>(defaultSelectedIds ?? new Set());
  const [activeId, setActiveId] = useState<string | null>(null);
  const [contextMenu, setContextMenu] = useState<{ node: FileTreeNode; x: number; y: number } | null>(null);
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

  function selectNode(node: FileTreeNode, opts: { toggle?: boolean; range?: boolean }) {
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

  function handleRowClick(node: FileTreeNode, e: ReactMouseEvent) {
    setActiveId(node.id);
    selectNode(node, { toggle: e.metaKey || e.ctrlKey, range: e.shiftKey });
    if (node.kind === "folder") toggleExpanded(node.id);
  }

  function handleKeyDown(e: KeyboardEvent, entry: FlatEntry) {
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
        if (entry.node.kind === "folder") {
          if (!activeExpanded.has(entry.node.id)) toggleExpanded(entry.node.id);
          else if (flat[index + 1]?.parentId === entry.node.id) setActiveId(flat[index + 1].node.id);
        }
        break;
      case "ArrowLeft":
        e.preventDefault();
        if (entry.node.kind === "folder" && activeExpanded.has(entry.node.id)) {
          toggleExpanded(entry.node.id);
        } else if (entry.parentId) {
          setActiveId(entry.parentId);
        }
        break;
      case "Enter":
        e.preventDefault();
        if (entry.node.kind === "folder") toggleExpanded(entry.node.id);
        else onActivate?.(entry.node);
        break;
      case " ":
        e.preventDefault();
        selectNode(entry.node, { toggle: true });
        break;
    }
  }

  function handleContextMenu(e: ReactMouseEvent, node: FileTreeNode) {
    if (!getMenuItems) return;
    e.preventDefault();
    setContextMenu({ node, x: e.clientX, y: e.clientY });
  }

  function renderNodes(list: FileTreeNode[], depth: number, parentId: string | null): ReactNode {
    return list.map((node) => {
      const expanded = node.kind === "folder" && activeExpanded.has(node.id);
      const selected = activeSelected.has(node.id);
      const entry: FlatEntry = { node, depth, parentId };
      return (
        <div key={node.id} className="sp-filetree-branch">
          <div
            className="sp-filetree-row"
            role="treeitem"
            aria-selected={selected}
            aria-expanded={node.kind === "folder" ? expanded : undefined}
            data-selected={selected || undefined}
            tabIndex={activeId === node.id ? 0 : -1}
            style={{ height: ROW_HEIGHT, paddingLeft: depth * INDENT + 6 }}
            onFocus={() => setActiveId(node.id)}
            onKeyDown={(e) => handleKeyDown(e, entry)}
            onClick={(e) => handleRowClick(node, e)}
            onDoubleClick={() => {
              if (node.kind === "file") onActivate?.(node);
            }}
            onContextMenu={(e) => handleContextMenu(e, node)}
          >
            {node.kind === "folder" ? (
              <span className="sp-filetree-chevron" data-expanded={expanded || undefined}>
                <Icon name="chevronRight" size={13} />
              </span>
            ) : (
              <span className="sp-filetree-chevron-spacer" />
            )}
            <span className="sp-filetree-icon">
              {node.icon ?? <Icon name={node.kind === "folder" ? (expanded ? "folderOpen" : "folder") : "file"} size={15} />}
            </span>
            <span className="sp-filetree-label">{node.name}</span>
          </div>
          {node.kind === "folder" && node.children && node.children.length > 0 && (
            <div className="sp-filetree-children" data-expanded={expanded || undefined}>
              <div className="sp-filetree-children-inner">{renderNodes(node.children, depth + 1, node.id)}</div>
            </div>
          )}
        </div>
      );
    });
  }

  return (
    <div className={["sp-filetree", className].filter(Boolean).join(" ")} role="tree">
      {renderNodes(nodes, 0, null)}
      {contextMenu && (
        <div
          className="sp-filetree-contextmenu"
          style={{ left: contextMenu.x, top: contextMenu.y }}
          onPointerDown={(e) => e.stopPropagation()}
        >
          {getMenuItems!(contextMenu.node).map((item) => (
            <Tab
              key={item.label}
              orientation="list"
              className={item.destructive ? "sp-filetree-menuitem--destructive" : undefined}
              onClick={() => {
                item.onSelect(contextMenu.node);
                setContextMenu(null);
              }}
            >
              {item.label}
            </Tab>
          ))}
        </div>
      )}
    </div>
  );
}
