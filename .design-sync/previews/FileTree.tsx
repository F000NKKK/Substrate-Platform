import React from "react";
import { FileTree, type FileTreeNode } from "substrate-platform-ui";

const NODES: FileTreeNode[] = [
  {
    id: "src",
    name: "src",
    kind: "folder",
    children: [
      { id: "widgets", name: "widgets", kind: "folder", children: [{ id: "button", name: "Button.tsx", kind: "file" }] },
      { id: "index", name: "index.tsx", kind: "file" },
    ],
  },
  { id: "pkg", name: "package.json", kind: "file" },
  { id: "readme", name: "README.md", kind: "file" },
];

function Frame({ children }: { children: React.ReactNode }) {
  return (
    <div style={{
      background: "var(--sp-surface-1)", padding: "var(--sp-space-md)",
      borderRadius: "var(--sp-radius-md)", width: 260,
    }}>{children}</div>
  );
}

/** Tree configured for files: folder/open-folder/file icons and a project-shaped node tree. */
export function ProjectTree() {
  return (
    <Frame>
      <FileTree nodes={NODES} defaultExpandedIds={new Set(["src", "widgets"])} />
    </Frame>
  );
}

/** The right-click dropdown — `getMenuItems` turns any node into a context menu. */
export function WithContextMenu() {
  return (
    <Frame>
      <FileTree
        nodes={NODES}
        defaultExpandedIds={new Set(["src"])}
        getMenuItems={() => [
          { label: "Rename", onSelect: () => {} },
          { label: "Delete", destructive: true, onSelect: () => {} },
        ]}
      />
    </Frame>
  );
}
