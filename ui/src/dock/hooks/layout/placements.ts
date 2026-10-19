// Where panels live: sizes per anchor, sane restored placements, a float's
// first spot, one flyout per anchor.

import { DOCK_ANCHORS, DOCK_MODES, type DockAnchor, type DockMode, type PanelPlacement, type ReopenPlacement, type ToolWindowAnchor } from "../../types";

/** What a restored floating placement must stay within: position, width and height (px). */
const SANE_FLOAT = { position: [-10_000, 10_000], width: [120, 4000], height: [80, 4000] } as const;
/** A float's first spot: the window width assumed outside a browser, its least left margin, and its top. */
const FALLBACK_WIDTH = 1280;
const FLOAT_MARGIN = 8;
const FLOAT_TOP = 48;

export const DEFAULT_FLOAT_SIZE = { w: 340, h: 280 };

/** Matches the `--sp-toolwindow-size`/`--sp-toolwindow-bottom-size` token defaults — the starting size before a user drags a dock's resize handle. */
export const DEFAULT_ANCHOR_SIZE: Record<ToolWindowAnchor, number> = { left: 260, right: 260, bottom: 220 };
/*
 * Per-anchor floor. Left/right constrain the panel's WIDTH, which must stay
 * wide enough for the header's title + its three action buttons; bottom
 * constrains HEIGHT, where the header is always full-width, so a much smaller
 * floor is fine.
 */
export const MIN_ANCHOR_SIZE: Record<ToolWindowAnchor, number> = { left: 200, right: 200, bottom: 110 };
export const MAX_ANCHOR_SIZE = 900;

/** Pinned and flyout are sized independently — a pinned panel holds permanent layout space so it's often sized differently than the same panel peeking open as a temporary overlay. */
export type SizeMode = "pinned" | "flyout";

export function sizeKey(panelId: string, anchor: ToolWindowAnchor, mode: SizeMode): string {
  return `${panelId}:${anchor}:${mode}`;
}

/** A restored placement is only trusted if it's well-formed — a corrupt or hand-edited entry (a zero/negative/huge floating size, an unknown anchor) falls back to the default instead of producing a degenerate panel. */
export function isSanePlacement(p: unknown): p is PanelPlacement {
  const v = p as Record<string, unknown> | null;
  if (!v || typeof v !== "object") return false;
  if (v.anchor === "float") {
    const inRange = (n: unknown, [min, max]: readonly [number, number]) => typeof n === "number" && Number.isFinite(n) && n >= min && n <= max;
    return inRange(v.x, SANE_FLOAT.position) && inRange(v.y, SANE_FLOAT.position) && inRange(v.w, SANE_FLOAT.width) && inRange(v.h, SANE_FLOAT.height);
  }
  return DOCK_ANCHORS.includes(v.anchor as DockAnchor) && DOCK_MODES.includes(v.mode as DockMode);
}

/** A transient panel's first floating spot: its own size, centered horizontally near the top of the shell. */
export function initialFloat(size: { w: number; h: number }): ReopenPlacement {
  const width = typeof window === "undefined" ? FALLBACK_WIDTH : window.innerWidth;
  return { anchor: "float", x: Math.max(FLOAT_MARGIN, Math.round((width - size.w) / 2)), y: FLOAT_TOP, ...size };
}

/** Only one flyout may peek open per anchor at a time — pinned panels are untouched by this (several can be pinned side by side). */
export function closeOtherFlyouts(
  placements: Record<string, PanelPlacement>,
  anchor: ToolWindowAnchor,
  exceptId: string
): Record<string, PanelPlacement> {
  const next = { ...placements };
  for (const [id, p] of Object.entries(placements)) {
    if (id !== exceptId && p.anchor === anchor && p.mode === "flyout") {
      next[id] = { anchor, mode: "hidden" };
    }
  }
  return next;
}
