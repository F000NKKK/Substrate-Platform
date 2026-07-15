import type { ComponentType, ReactNode } from "react";

export type ToolWindowAnchor = "left" | "right" | "bottom";

export interface PanelDef {
  id: string;
  title: string;
  component: ComponentType;
}

export interface PlatformShellProps {
  /** The one always-visible, non-collapsible content area (e.g. a designer/editor canvas). */
  main: PanelDef;
  toolWindows: Partial<Record<ToolWindowAnchor, PanelDef[]>>;
  /** Panel ids that start pinned (docked, taking layout space) rather than auto-hidden. */
  defaultPinned?: Partial<Record<ToolWindowAnchor, string>>;
  menu?: ReactNode;
}

export type DockMode = "hidden" | "flyout" | "pinned";

/** Where a tool window currently lives — dragged freely between any dock or out into a floating panel. */
export type PanelPlacement =
  | { anchor: ToolWindowAnchor; mode: DockMode }
  | { anchor: "float"; x: number; y: number; w: number; h: number };

export function isFloating(p: PanelPlacement): p is Extract<PanelPlacement, { anchor: "float" }> {
  return p.anchor === "float";
}
