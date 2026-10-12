import type { ComponentType } from "react";
import type { ContextMenuItem } from "../widgets/ContextMenu";

export type ToolWindowAnchor = "left" | "right" | "bottom";

/** Every place a panel can be docked — the three tool-window edges, or the tabbed center document zone. */
export type DockAnchor = ToolWindowAnchor | "center";

export interface PanelDef {
  id: string;
  title: string;
  component: ComponentType;
  /** Renders no tab button in whatever dock it's active in, while still rendering its content when active — for a permanent "empty state" center panel that shouldn't clutter the tab strip once real content (e.g. dynamically-opened file tabs) exists alongside it. */
  hidden?: boolean;
  /** Hover text for the panel's tab (e.g. a file tab's full path). */
  tooltip?: string;
  /** A center tab kept at the front of the strip, drawn with a pin the user clicks to unpin (`onUnpin`) instead of a close button. */
  pinned?: boolean;
  onUnpin?: () => void;
  /** Items for the tab's right-click menu — `centerIds` is the center strip's tab order as shown, for "Close Tabs to the Right" and the like. */
  contextMenu?: (context: TabMenuContext) => ContextMenuItem[];
}

export interface TabMenuContext {
  centerIds: string[];
}

export type DockMode = "hidden" | "flyout" | "pinned";

/** Where a panel currently lives — dragged freely between any dock, the center tab group, or out into a floating panel. */
export type PanelPlacement =
  | { anchor: DockAnchor; mode: DockMode }
  | { anchor: "float"; x: number; y: number; w: number; h: number };

export function isFloating(p: PanelPlacement): p is Extract<PanelPlacement, { anchor: "float" }> {
  return p.anchor === "float";
}
