import { useLayoutEffect, useRef, useState, type ReactNode, type RefObject } from "react";
import type { ToolWindowAnchor } from "./types";

interface Pt {
  x: number;
  y: number;
}
interface Rect {
  l: number;
  t: number;
  r: number;
  b: number;
}

/**
 * The union outline of the flyout panel plus its still-visible strip tab, as
 * an ordered ring of vertices. The tab pokes out of the panel's anchor-facing
 * edge, so the ring has 8 corners: 6 convex (the outer box + the tab's tip)
 * and 2 concave (where the tab's sides meet the panel's edge) — exactly the
 * single continuous, notched outline VS draws around an auto-hide flyout.
 */
function unionRing(anchor: ToolWindowAnchor, panel: Rect, tab: Rect): Pt[] {
  switch (anchor) {
    case "left":
      // tab on the left, panel on its right; seam at x = panel.l (= tab.r)
      return [
        { x: tab.l, y: tab.t },
        { x: panel.l, y: tab.t }, // concave
        { x: panel.l, y: panel.t },
        { x: panel.r, y: panel.t },
        { x: panel.r, y: panel.b },
        { x: panel.l, y: panel.b },
        { x: panel.l, y: tab.b }, // concave
        { x: tab.l, y: tab.b },
      ];
    case "right":
      // panel on the left, tab on its right; seam at x = panel.r (= tab.l)
      return [
        { x: panel.l, y: panel.t },
        { x: panel.r, y: panel.t },
        { x: panel.r, y: tab.t }, // concave
        { x: tab.r, y: tab.t },
        { x: tab.r, y: tab.b },
        { x: panel.r, y: tab.b }, // concave
        { x: panel.r, y: panel.b },
        { x: panel.l, y: panel.b },
      ];
    case "bottom":
      // panel on top, tab below it; seam at y = panel.b (= tab.t)
      return [
        { x: panel.l, y: panel.t },
        { x: panel.r, y: panel.t },
        { x: panel.r, y: panel.b },
        { x: tab.r, y: panel.b }, // concave
        { x: tab.r, y: tab.b },
        { x: tab.l, y: tab.b },
        { x: tab.l, y: panel.b }, // concave
        { x: panel.l, y: panel.b },
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
 * and concave corners round identically, so all 8 corners share one radius.
 */
function roundedPath(points: Pt[], r: number): string {
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

const CORNER_RADIUS = 8;

export interface FlyoutFrameProps {
  anchor: ToolWindowAnchor;
  /** The dock region the flyout lives in — used as the coordinate origin and to locate the active strip tab. */
  regionRef: RefObject<HTMLElement | null>;
  children: ReactNode;
}

/**
 * A peeked-open auto-hide flyout: its panel, laid over the main area next to
 * the strip, wrapped together with its strip tab by ONE continuous accent
 * outline drawn as an SVG path (see {@link unionRing}). The outline is
 * measured from the live DOM rects of the tab and panel, so it always traces
 * the two as a single notched shape rather than two disconnected boxes.
 */
export function FlyoutFrame({ anchor, regionRef, children }: FlyoutFrameProps) {
  const panelRef = useRef<HTMLDivElement>(null);
  const [path, setPath] = useState<string | null>(null);

  useLayoutEffect(() => {
    const region = regionRef.current;
    const panelEl = panelRef.current;
    if (!region || !panelEl) return;

    const compute = () => {
      const tabEl = region.querySelector<HTMLElement>('.sp-dock-strip [data-active="true"]');
      const origin = region.getBoundingClientRect();
      const p = panelEl.getBoundingClientRect();
      const panel: Rect = {
        l: p.left - origin.left,
        t: p.top - origin.top,
        r: p.right - origin.left,
        b: p.bottom - origin.top,
      };
      if (!tabEl) {
        setPath(roundedPath(
          [
            { x: panel.l, y: panel.t },
            { x: panel.r, y: panel.t },
            { x: panel.r, y: panel.b },
            { x: panel.l, y: panel.b },
          ],
          CORNER_RADIUS
        ));
        return;
      }
      const t = tabEl.getBoundingClientRect();
      const tab: Rect = {
        l: t.left - origin.left,
        t: t.top - origin.top,
        r: t.right - origin.left,
        b: t.bottom - origin.top,
      };
      setPath(roundedPath(unionRing(anchor, panel, tab), CORNER_RADIUS));
    };

    compute();
    const ro = new ResizeObserver(compute);
    ro.observe(region);
    ro.observe(panelEl);
    window.addEventListener("resize", compute);
    return () => {
      ro.disconnect();
      window.removeEventListener("resize", compute);
    };
  }, [anchor, regionRef]);

  return (
    <>
      <div ref={panelRef} className={`sp-dock-flyout sp-dock-flyout--${anchor}`}>
        {children}
      </div>
      {path && (
        <svg className="sp-flyout-border" aria-hidden>
          <path d={path} fill="none" stroke="var(--sp-accent)" strokeWidth={1.5} vectorEffect="non-scaling-stroke" />
        </svg>
      )}
    </>
  );
}
