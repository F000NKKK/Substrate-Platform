import type { ReactNode } from "react";
import type { PanelDef, ToolWindowAnchor } from "../dock/types";

export interface PlatformShellProps {
  /** The center zone's first tab (e.g. a designer/editor canvas) — the center zone itself is a normal tabbed dock, so more panels can join it later by being dragged in. */
  main: PanelDef;
  toolWindows: Partial<Record<ToolWindowAnchor, PanelDef[]>>;
  /** Panel ids that start pinned (docked, taking layout space) rather than auto-hidden. */
  defaultPinned?: Partial<Record<ToolWindowAnchor, string>>;
  menu?: ReactNode;
}
