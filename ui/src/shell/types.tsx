import type { ReactNode } from "react";
import type { PanelDef, ToolWindowAnchor } from "../dock/types";

export interface PlatformShellProps {
  /** The center zone's first tab (e.g. a designer/editor canvas) — the center zone itself is a normal tabbed dock, so more panels can join it later by being dragged in. */
  main: PanelDef;
  toolWindows: Partial<Record<ToolWindowAnchor, PanelDef[]>>;
  /** Panel ids that start pinned (docked, taking layout space) rather than auto-hidden. */
  defaultPinned?: Partial<Record<ToolWindowAnchor, string>>;
  /** When set, the dock layout (panel placements/sizes/active center tab) survives a reload via `localStorage`, keyed by this string (make it unique per app). */
  persistKey?: string;
  /** Panels docked to the center beyond `main`, registered dynamically at runtime — e.g. one per open file in an editor, with no wrapper panel around them. The product owns this array as its own state; the shell just keeps its tab bar/placements in sync with it. */
  extraCenterPanels?: PanelDef[];
  /** Fired when the user closes one of `extraCenterPanels`'s tabs — remove it from that array in response. */
  onCloseDynamicPanel?: (id: string) => void;
  menu?: ReactNode;
}
