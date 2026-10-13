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
  /** Buttons at the right end of the center tab strip while this panel is the active tab — VS Code's editor title actions ("Open Preview to the Side", ...). */
  actions?: PanelAction[];
  /**
   * A transient tool window, like Visual Studio's Find and Replace or Code
   * Search: the first time it's shown it opens floating at this size (centered
   * near the top), and closing it takes it off every dock — no tab is left in
   * an edge strip. Docked somewhere (an edge, or the center as a tab), it
   * reopens there; closed while floating, it reopens where it floated.
   */
  floatSize?: { w: number; h: number };
}

export interface PanelAction {
  id: string;
  /** An `Icon` name. */
  icon: string;
  /** Tooltip — include the shortcut, if any. */
  title: string;
  /** Drawn pressed (e.g. the view currently shown). */
  active?: boolean;
  onClick: () => void;
}

export interface TabMenuContext {
  centerIds: string[];
}

/** `closed` — a tool window that isn't on screen anywhere, not even as a strip tab (closed with its × — View brings it back). */
export type DockMode = "hidden" | "flyout" | "pinned" | "closed";

export interface FloatRect {
  x: number;
  y: number;
  w: number;
  h: number;
}

/** Where a closed panel comes back when shown again, if not pinned on its `anchor` — `flyout`: it was auto-hidden, so it comes back as a strip tab, slid open. */
export type ReopenPlacement = { anchor: "center" } | { anchor: "flyout" } | ({ anchor: "float" } & FloatRect);

/** Where a panel currently lives — dragged freely between any dock, the center tab group, or out into a floating panel. */
export type PanelPlacement =
  | { anchor: DockAnchor; mode: DockMode; reopen?: ReopenPlacement }
  | ({ anchor: "float" } & FloatRect);
