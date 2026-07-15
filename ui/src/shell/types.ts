import type { ComponentType, ReactNode } from "react";

export type ToolWindowAnchor = "left" | "right" | "bottom";

export interface PanelDef {
  id: string;
  title: string;
  /** Short glyph shown in the collapsed edge strip (e.g. a single emoji or letter). */
  icon: string;
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

export interface DockState {
  activeId: string | null;
  mode: DockMode;
}
