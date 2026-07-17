import { useLayoutEffect, useRef, useState, type ReactNode } from "react";
import { createPortal } from "react-dom";

/** A click point, or a trigger element's rect (e.g. a submenu opening beside its row). */
export type ViewportAnchor = { x: number; y: number } | DOMRect;

export interface UseViewportPositionOptions {
  /** Minimum gap kept from the viewport edge. Default 4. */
  margin?: number;
  /** Rect anchors only: prefer opening to the anchor's right, flipping to its left when there isn't room — the way a submenu opens beside its trigger row. Point anchors are just clamped, never flipped. */
  flip?: boolean;
}

function isRect(anchor: ViewportAnchor): anchor is DOMRect {
  return "left" in anchor;
}

function anchorKeyOf(anchor: ViewportAnchor): string {
  return isRect(anchor)
    ? `${anchor.left}:${anchor.top}:${anchor.right}:${anchor.bottom}`
    : `${anchor.x}:${anchor.y}`;
}

function initialPosition(anchor: ViewportAnchor): { left: number; top: number } {
  return isRect(anchor) ? { left: anchor.right, top: anchor.top } : { left: anchor.x, top: anchor.y };
}

/**
 * Positions a portaled element (menu, submenu, tooltip — anything that must
 * escape an `overflow: hidden`/`backdrop-filter` ancestor via `createPortal`)
 * against a point or a trigger's rect, then clamps it to stay fully inside the
 * viewport. This is the one measure-after-mount-and-clamp implementation for
 * every such popup in the platform, instead of each consumer re-deriving it —
 * pair with `ViewportPortal` to mount to `document.body`.
 */
export function useViewportPosition(anchor: ViewportAnchor | null, options: UseViewportPositionOptions = {}) {
  const { margin = 4, flip = false } = options;
  const ref = useRef<HTMLDivElement>(null);
  const [pos, setPos] = useState(() => (anchor ? initialPosition(anchor) : { left: 0, top: 0 }));
  const anchorKey = anchor ? anchorKeyOf(anchor) : null;

  useLayoutEffect(() => {
    const el = ref.current;
    if (!anchor || !el) return;
    const rect = el.getBoundingClientRect();

    let left: number;
    let top: number;
    if (isRect(anchor)) {
      const fitsRight = anchor.right + rect.width + margin <= window.innerWidth;
      left = flip && !fitsRight ? Math.max(margin, anchor.left - rect.width) : anchor.right;
      top = anchor.top;
    } else {
      left = anchor.x;
      top = anchor.y;
    }

    const maxLeft = Math.max(margin, window.innerWidth - rect.width - margin);
    const maxTop = Math.max(margin, window.innerHeight - rect.height - margin);
    setPos({ left: Math.min(left, maxLeft), top: Math.min(top, maxTop) });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [anchorKey, margin, flip]);

  return { ref, style: { left: pos.left, top: pos.top } };
}

/** Portals children to `document.body` — pair with `useViewportPosition` for anything that needs to escape a clipping/backdrop-filter ancestor and sit at a viewport-relative position. */
export function ViewportPortal({ children }: { children: ReactNode }) {
  return createPortal(children, document.body);
}
