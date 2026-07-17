import { useLayoutEffect, useState, type RefObject } from "react";
import "./outline.css";

export interface Rect {
  l: number;
  t: number;
  r: number;
  b: number;
}

interface Pt {
  x: number;
  y: number;
}

function rectOf(el: HTMLElement, origin: DOMRect): Rect {
  const r = el.getBoundingClientRect();
  return { l: r.left - origin.left, t: r.top - origin.top, r: r.right - origin.left, b: r.bottom - origin.top };
}

/** A plain rounded rectangle — the ring for the single-target case (e.g. a pinned panel with no notch to trace). */
function boxRing(r: Rect): Pt[] {
  return [
    { x: r.l, y: r.t },
    { x: r.r, y: r.t },
    { x: r.r, y: r.b },
    { x: r.l, y: r.b },
  ];
}

/**
 * The union outline of a primary rect (e.g. a flyout panel) plus a second,
 * smaller rect poking out of one of its edges (e.g. its still-visible strip
 * tab), as an ordered ring of vertices — 8 corners: 6 convex (the outer box +
 * the tab's tip) and 2 concave (where the tab's sides meet the panel's edge).
 * `side` is which edge of `primary` the `secondary` rect pokes out of.
 */
function notchedRing(primary: Rect, secondary: Rect, side: "left" | "right" | "top" | "bottom"): Pt[] {
  switch (side) {
    case "left":
      // secondary pokes out of primary's left edge; seam at x = primary.l (= secondary.r)
      return [
        { x: secondary.l, y: secondary.t },
        { x: primary.l, y: secondary.t }, // concave
        { x: primary.l, y: primary.t },
        { x: primary.r, y: primary.t },
        { x: primary.r, y: primary.b },
        { x: primary.l, y: primary.b },
        { x: primary.l, y: secondary.b }, // concave
        { x: secondary.l, y: secondary.b },
      ];
    case "right":
      return [
        { x: primary.l, y: primary.t },
        { x: primary.r, y: primary.t },
        { x: primary.r, y: secondary.t }, // concave
        { x: secondary.r, y: secondary.t },
        { x: secondary.r, y: secondary.b },
        { x: primary.r, y: secondary.b }, // concave
        { x: primary.r, y: primary.b },
        { x: primary.l, y: primary.b },
      ];
    case "top":
      return [
        { x: secondary.l, y: secondary.t },
        { x: secondary.r, y: secondary.t },
        { x: secondary.r, y: primary.t }, // concave
        { x: primary.r, y: primary.t },
        { x: primary.r, y: primary.b },
        { x: primary.l, y: primary.b },
        { x: primary.l, y: primary.t }, // concave
        { x: secondary.l, y: primary.t },
      ];
    case "bottom":
      return [
        { x: primary.l, y: primary.t },
        { x: primary.r, y: primary.t },
        { x: primary.r, y: primary.b },
        { x: secondary.r, y: primary.b }, // concave
        { x: secondary.r, y: secondary.b },
        { x: secondary.l, y: secondary.b },
        { x: secondary.l, y: primary.b }, // concave
        { x: primary.l, y: primary.b },
      ];
  }
}

function len(a: Pt, b: Pt): number {
  return Math.hypot(b.x - a.x, b.y - a.y);
}

/**
 * A closed SVG path that rounds every corner of `points` by radius `r` — a
 * quadratic Bézier per vertex (control point = the sharp corner), clamped so a
 * corner never eats more than half of its shorter neighbouring edge. Convex
 * and concave corners round identically, so a plain box and a notched union
 * share one rounding routine.
 */
export function roundedPath(points: Pt[], r: number): string {
  const n = points.length;
  let d = "";
  for (let i = 0; i < n; i++) {
    const prev = points[(i - 1 + n) % n];
    const cur = points[i];
    const next = points[(i + 1) % n];
    const rIn = Math.min(r, len(prev, cur) / 2);
    const rOut = Math.min(r, len(next, cur) / 2);
    const before: Pt = {
      x: cur.x + ((prev.x - cur.x) / len(prev, cur)) * rIn,
      y: cur.y + ((prev.y - cur.y) / len(prev, cur)) * rIn,
    };
    const after: Pt = {
      x: cur.x + ((next.x - cur.x) / len(next, cur)) * rOut,
      y: cur.y + ((next.y - cur.y) / len(next, cur)) * rOut,
    };
    d += i === 0 ? `M ${before.x} ${before.y} ` : `L ${before.x} ${before.y} `;
    d += `Q ${cur.x} ${cur.y} ${after.x} ${after.y} `;
  }
  return d + "Z";
}

export type OutlineTarget = RefObject<HTMLElement | null> | (() => HTMLElement | null);

function resolve(target: OutlineTarget): HTMLElement | null {
  return typeof target === "function" ? target() : target.current;
}

export interface OutlineProps {
  /** Positioning + measurement origin — the outline is an absolutely-positioned overlay covering this element, so give it `position: relative`. */
  regionRef: RefObject<HTMLElement | null>;
  /** One target draws a plain rounded rectangle around it. Two draw their notched union (e.g. a flyout panel + its strip tab) — the second is treated as the notch poking out of the first. */
  targets: [OutlineTarget] | [OutlineTarget, OutlineTarget];
  /** Which edge of the first target the second pokes out of — required (and only meaningful) for the two-target case. */
  notchSide?: "left" | "right" | "top" | "bottom";
  radius?: number;
  color?: string;
  strokeWidth?: number;
  className?: string;
  /**
   * Pass whatever value the target's own live size is derived from (e.g. the
   * pinned width a drag handle is actively changing) so the outline
   * recomputes synchronously in the same commit as that resize, instead of
   * waiting for ResizeObserver's own callback — which fires a frame later
   * and reads as a visible lag/jitter during a live drag. ResizeObserver
   * stays wired up too, as a fallback for size changes this component isn't
   * driving itself (e.g. a window resize).
   */
  syncWith?: unknown;
}

const DEFAULT_RADIUS = 8;

/**
 * The one continuous-outline renderer in the platform — a rounded-corner SVG
 * path traced live around one element (a plain border substitute) or the
 * notched union of two (a panel plus a tab poking out of its edge). Draw
 * every accent outline through this rather than a per-case CSS `border` and
 * a one-off SVG, so a pinned panel and its own flyout state render
 * *identically* — same radius, same stroke, same rounding — instead of
 * silently drifting apart the way a CSS border and a hand-rolled SVG outline
 * always eventually do.
 */
export function Outline({
  regionRef,
  targets,
  notchSide,
  radius = DEFAULT_RADIUS,
  color = "var(--sp-accent)",
  strokeWidth = 1.5,
  className,
  syncWith,
}: OutlineProps) {
  const [path, setPath] = useState<string | null>(null);
  const targetsKey = targets.length;

  function compute() {
    const region = regionRef.current;
    if (!region) return;
    const origin = region.getBoundingClientRect();
    const primaryEl = resolve(targets[0]);
    if (!primaryEl) {
      setPath(null);
      return;
    }
    const primary = rectOf(primaryEl, origin);
    const secondaryEl = targets[1] ? resolve(targets[1]) : null;
    const ring = secondaryEl && notchSide ? notchedRing(primary, rectOf(secondaryEl, origin), notchSide) : boxRing(primary);
    setPath(roundedPath(ring, radius));
  }

  // Runs synchronously (before paint) whenever `syncWith` changes — the fast
  // path for a resize this component itself drives.
  useLayoutEffect(() => {
    compute();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [syncWith]);

  useLayoutEffect(() => {
    const region = regionRef.current;
    if (!region) return;

    compute();
    const ro = new ResizeObserver(compute);
    ro.observe(region);
    const primaryEl = resolve(targets[0]);
    if (primaryEl) ro.observe(primaryEl);
    const secondaryEl = targets[1] ? resolve(targets[1]) : null;
    if (secondaryEl) ro.observe(secondaryEl);
    window.addEventListener("resize", compute);
    return () => {
      ro.disconnect();
      window.removeEventListener("resize", compute);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [regionRef, notchSide, radius, targetsKey]);

  if (!path) return null;

  return (
    <svg className={["sp-outline", className].filter(Boolean).join(" ")} aria-hidden>
      <path d={path} fill="none" stroke={color} strokeWidth={strokeWidth} vectorEffect="non-scaling-stroke" />
    </svg>
  );
}
