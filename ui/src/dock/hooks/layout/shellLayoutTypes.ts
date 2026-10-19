import type { DockAnchor, PanelDef, PanelPlacement, ToolWindowAnchor } from "../../types";
import type { SizeMode } from "./placements";

export interface ShellLayout {
  panelsById: Record<string, PanelDef>;
  mainId: string;
  homeAnchor: Record<string, DockAnchor>;
  idsByAnchor: (anchor: ToolWindowAnchor) => string[];
  /** All simultaneously pinned panels on this anchor, in the order they were pinned — each gets its own grid slot, side by side. */
  pinnedInAnchor: (anchor: ToolWindowAnchor) => string[];
  /** The one panel currently peeking open as a flyout on this anchor, if any. */
  flyoutInAnchor: (anchor: ToolWindowAnchor) => string | null;
  centerIds: string[];
  centerActiveId: string;
  setCenterActive: (id: string) => void;
  floatingIds: string[];
  placements: Record<string, PanelPlacement>;
  toggle: (id: string) => void;
  pin: (id: string) => void;
  unpin: (id: string) => void;
  close: (id: string) => void;
  /** Brings a tool window into view wherever it currently lives — pins it open on its edge if it was hidden, or makes it the active tab if it's docked in the center. A no-op for a floating panel (already visible). */
  show: (id: string) => void;
  /** Hides a tool window — stays on its current edge (just collapsed to its strip), or goes back to its home edge if it was docked in the center or floating. */
  hide: (id: string) => void;
  /** Whether a panel is currently on screen (pinned, peeking as a flyout, floating, or a center tab) rather than collapsed to its strip. */
  isVisible: (id: string) => boolean;
  dockTo: (id: string, anchor: DockAnchor) => void;
  floatAt: (id: string, x: number, y: number) => void;
  /** Moves a floating panel (no-op for one that isn't floating). */
  moveFloat: (id: string, x: number, y: number) => void;
  /** Remembers a floating panel's new size (no-op for a panel that isn't floating) — persisted along with every other placement. */
  resizeFloat: (id: string, w: number, h: number) => void;
  /**
   * A panel's own remembered width (left/right) or height (bottom) — tracked
   * per panel, per anchor, AND per pinned/flyout mode (6 independent slots
   * total across the 3 anchors, since a panel keeps a separate size for each
   * one it's visited, in each mode). Falls back to the anchor's default until
   * that specific combination has been resized at least once.
   */
  anchorSize: (panelId: string, anchor: ToolWindowAnchor, mode: SizeMode) => number;
  setAnchorSize: (panelId: string, anchor: ToolWindowAnchor, mode: SizeMode, size: number) => void;
}

/** What `useShellLayout` lays out, and how its layout is kept. */
export interface ShellLayoutOptions {
  main: PanelDef;
  toolWindows: Partial<Record<ToolWindowAnchor, PanelDef[]>>;
  defaultPinned?: Partial<Record<ToolWindowAnchor, string>>;
  /**
   * When set, panel placements/sizes/the active center tab survive a reload
   * — stored in the current `StateStore` (see `StateStoreProvider`) under
   * `shell.layout:<persistKey>`, so a product whose store is per-project gets
   * a per-project layout. Re-applied whenever the store's scope changes.
   */
  persistKey?: string;
  /**
   * Panels docked to the center beyond `main`, registered dynamically at
   * runtime (e.g. one per open file in an editor) rather than fixed at
   * startup — a product owns this array as its own state (adding/removing
   * from it directly, e.g. as files open/close) and just hands it in; the
   * shell keeps its own placement/active-tab state in sync with whatever's
   * currently in it, with no separate "register a panel" call needed.
   */
  extraCenterPanels?: PanelDef[];
  /**
   * Called when the user closes one of `extraCenterPanels`'s tabs — the
   * shell doesn't own that array, so it can't remove the entry itself; the
   * product is expected to remove it in response, which this effect then
   * picks up on the next render.
   */
  onCloseDynamicPanel?: (id: string) => void;
}
