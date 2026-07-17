import type { ComponentType, ReactNode } from "react";

export type ToolWindowAnchor = "left" | "right" | "bottom";

/** Every place a panel can be docked — the three tool-window edges, or the tabbed center document zone. */
export type DockAnchor = ToolWindowAnchor | "center";

export interface PanelDef {
  id: string;
  title: string;
  component: ComponentType;
}

export interface PlatformShellProps {
  /** The center zone's first tab (e.g. a designer/editor canvas) — the center zone itself is a normal tabbed dock, so more panels can join it later by being dragged in. */
  main: PanelDef;
  toolWindows: Partial<Record<ToolWindowAnchor, PanelDef[]>>;
  /** Panel ids that start pinned (docked, taking layout space) rather than auto-hidden. */
  defaultPinned?: Partial<Record<ToolWindowAnchor, string>>;
  menu?: ReactNode;
}

export type DockMode = "hidden" | "flyout" | "pinned";

/** Where a panel currently lives — dragged freely between any dock, the center tab group, or out into a floating panel. */
export type PanelPlacement =
  | { anchor: DockAnchor; mode: DockMode }
  | { anchor: "float"; x: number; y: number; w: number; h: number };

export function isFloating(p: PanelPlacement): p is Extract<PanelPlacement, { anchor: "float" }> {
  return p.anchor === "float";
}
