import type { ComponentType } from "react";

export type ToolWindowAnchor = "left" | "right" | "bottom";

/** Every place a panel can be docked — the three tool-window edges, or the tabbed center document zone. */
export type DockAnchor = ToolWindowAnchor | "center";

export interface PanelDef {
  id: string;
  title: string;
  component: ComponentType;
}

export type DockMode = "hidden" | "flyout" | "pinned";

/** Where a panel currently lives — dragged freely between any dock, the center tab group, or out into a floating panel. */
export type PanelPlacement =
  | { anchor: DockAnchor; mode: DockMode }
  | { anchor: "float"; x: number; y: number; w: number; h: number };

export function isFloating(p: PanelPlacement): p is Extract<PanelPlacement, { anchor: "float" }> {
  return p.anchor === "float";
}
