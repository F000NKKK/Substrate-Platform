import React from "react";
import { Tree, type TreeNode } from "substrate-platform-ui";

interface CategoryData {
  label: string;
}

const NODES: TreeNode<CategoryData>[] = [
  {
    id: "layout",
    label: "Layout",
    data: { label: "Layout" },
    children: [
      { id: "flex", label: "Flexbox", data: { label: "Flexbox" } },
      { id: "grid", label: "Grid", data: { label: "Grid" } },
    ],
  },
  {
    id: "typography",
    label: "Typography",
    data: { label: "Typography" },
    children: [{ id: "headings", label: "Headings", data: { label: "Headings" } }],
  },
  { id: "colors", label: "Colors", data: { label: "Colors" } },
];

function Frame({ children }: { children: React.ReactNode }) {
  return (
    <div style={{
      background: "var(--sp-surface-1)", padding: "var(--sp-space-md)",
      borderRadius: "var(--sp-radius-md)", width: 260,
    }}>{children}</div>
  );
}

/** A plain nested category list with no icon slot — the generic base every domain-specific tree (like FileTree) configures further. */
export function CategoryTree() {
  return (
    <Frame>
      <Tree nodes={NODES} defaultExpandedIds={new Set(["layout"])} />
    </Frame>
  );
}
