import { useLayoutEffect, useRef, useState, type CSSProperties, type RefObject } from "react";
import "./outline.css";

export interface OutlineRect {
  l: number;
  t: number;
  r: number;
  b: number;
}

interface Pt {
  x: number;
  y: number;
}

export type OutlineTarget = RefObject<HTMLElement | null> | (() => HTMLElement | null);

/**
 * A contour: given the measured rects of the targets (in the outline's own
 * coordinate space) plus the styling params, return one SVG path string. This
 * is the public "merge" hook — a component describes how its targets combine
 * into an outline (a plain box, a notched union of two, anything) and the
 * engine just strokes the result.
 */
export type OutlineShape = (rects: OutlineRect[], radius: number, strokeWidth: number) => string;

function resolve(target: OutlineTarget): HTMLElement | null {
  return typeof target === "function" ? target() : target.current;
}

function rectOf(el: HTMLElement, origin: DOMRect): OutlineRect {
  const r = el.getBoundingClientRect();
  return { l: r.left - origin.left, t: r.top - origin.top, r: r.right - origin.left, b: r.bottom - origin.top };
}

function len(a: Pt, b: Pt): number {
  return Math.hypot(b.x - a.x, b.y - a.y);
}

/** Closes `points` into a rounded-corner SVG path — one quadratic Bézier per vertex, clamped so a corner never eats more than half its shorter edge. Convex and concave corners round identically. */
export function roundedPath(points: Pt[], radius: number): string {
  const n = points.length;
  if (n < 3) return "";
  let d = "";
  for (let i = 0; i < n; i++) {
    const prev = points[(i - 1 + n) % n];
    const cur = points[i];
    const next = points[(i + 1) % n];
    const rIn = Math.min(radius, len(prev, cur) / 2);
    const rOut = Math.min(radius, len(next, cur) / 2);
    const before: Pt = { x: cur.x + ((prev.x - cur.x) / len(prev, cur)) * rIn, y: cur.y + ((prev.y - cur.y) / len(prev, cur)) * rIn };
    const after: Pt = { x: cur.x + ((next.x - cur.x) / len(next, cur)) * rOut, y: cur.y + ((next.y - cur.y) / len(next, cur)) * rOut };
    d += i === 0 ? `M ${before.x} ${before.y} ` : `L ${before.x} ${before.y} `;
    d += `Q ${cur.x} ${cur.y} ${after.x} ${after.y} `;
  }
  return d + "Z";
}

function boxRing(r: OutlineRect): Pt[] {
  return [
    { x: r.l, y: r.t },
    { x: r.r, y: r.t },
    { x: r.r, y: r.b },
    { x: r.l, y: r.b },
  ];
}

/** The 8-corner ring for a panel (`primary`) plus a tab (`secondary`) poking out of the given edge — 6 convex corners + 2 concave, exactly VS's continuous auto-hide-flyout outline. */
function notchedRing(primary: OutlineRect, secondary: OutlineRect, side: "left" | "right" | "top" | "bottom"): Pt[] {
  switch (side) {
    case "left":
      return [
        { x: secondary.l, y: secondary.t },
        { x: primary.l, y: secondary.t },
        { x: primary.l, y: primary.t },
        { x: primary.r, y: primary.t },
        { x: primary.r, y: primary.b },
        { x: primary.l, y: primary.b },
        { x: primary.l, y: secondary.b },
        { x: secondary.l, y: secondary.b },
      ];
    case "right":
      return [
        { x: primary.l, y: primary.t },
        { x: primary.r, y: primary.t },
        { x: primary.r, y: secondary.t },
        { x: secondary.r, y: secondary.t },
        { x: secondary.r, y: secondary.b },
        { x: primary.r, y: secondary.b },
        { x: primary.r, y: primary.b },
        { x: primary.l, y: primary.b },
      ];
    case "top":
      return [
        { x: secondary.l, y: secondary.t },
        { x: secondary.r, y: secondary.t },
        { x: secondary.r, y: primary.t },
        { x: primary.r, y: primary.t },
        { x: primary.r, y: primary.b },
        { x: primary.l, y: primary.b },
        { x: primary.l, y: primary.t },
        { x: secondary.l, y: primary.t },
      ];
    case "bottom":
      return [
        { x: primary.l, y: primary.t },
        { x: primary.r, y: primary.t },
        { x: primary.r, y: primary.b },
        { x: secondary.r, y: primary.b },
        { x: secondary.r, y: secondary.b },
        { x: secondary.l, y: secondary.b },
        { x: secondary.l, y: primary.b },
        { x: primary.l, y: primary.b },
      ];
  }
}

/** Built-in shape: a plain rounded box around the first target. */
export const boxShape: OutlineShape = (rects, radius) => (rects[0] ? roundedPath(boxRing(rects[0]), radius) : "");

/** Built-in shape factory: a panel (target 0) merged with a tab (target 1) poking out of `side` into one continuous notched ring. Falls back to a plain box if the tab is absent. */
export function notchedUnionShape(side: "left" | "right" | "top" | "bottom"): OutlineShape {
  return (rects, radius) => {
    if (!rects[0]) return "";
    if (!rects[1]) return roundedPath(boxRing(rects[0]), radius);
    return roundedPath(notchedRing(rects[0], rects[1], side), radius);
  };
}

const DEFAULT_RADIUS = 8;

export interface OutlineProps {
  radius?: number;
  color?: string;
  strokeWidth?: number;
  className?: string;
  /**
   * Region mode — draw over `regionRef` (a positioned ancestor) and outline
   * the union of `targets`, measured in that region's coordinates and merged
   * by `shape`. Use for a contour spanning MORE than one element (e.g. a
   * flyout panel + its separate strip tab). Omit both `regionRef` and
   * `targets` for the default self mode below.
   */
  regionRef?: RefObject<HTMLElement | null>;
  targets?: OutlineTarget[];
  /** The contour. In region mode defaults to `boxShape`; in self mode ignored (self mode always draws a box filling the parent). */
  shape?: OutlineShape;
  /** Region mode only: bump on every value a driven resize derives from (e.g. the panel's live size) to recompute synchronously in the same commit, instead of a frame later via ResizeObserver. */
  revision?: unknown;
}

/**
 * The platform's one accent-outline renderer, with two modes:
 *
 * • **Self mode** (default — no `regionRef`/`targets`): renders as a CHILD of
 *   the element it outlines and draws a rounded box filling it, measurement-
 *   free (an SVG `<rect>` sized `100%`). Because it's the element's own child,
 *   it inherits every move and resize for free — a neighbour's resize that
 *   merely SHIFTS this element can never desync the outline, the exact bug the
 *   old chase-it-across-a-region approach had. Pinned dock panels use this.
 *
 * • **Region mode** (`regionRef` + `targets`): renders over a positioned
 *   region and outlines the merged rects of several elements via `shape` — the
 *   only way to trace a contour spanning more than one box, e.g. a flyout
 *   panel plus its detached strip tab as one notched shape. The flyout is
 *   absolutely positioned and so never shifted by a sibling's resize, so
 *   region mode is safe there.
 */
export function Outline(props: OutlineProps) {
  if (props.regionRef && props.targets) return <RegionOutline {...props} regionRef={props.regionRef} targets={props.targets} />;
  return <SelfOutline radius={props.radius} color={props.color} strokeWidth={props.strokeWidth} className={props.className} />;
}

function SelfOutline({ radius = DEFAULT_RADIUS, color = "var(--sp-accent)", strokeWidth = 1.5, className }: OutlineProps) {
  const inset = strokeWidth / 2;
  const insetStyle: CSSProperties = { inset };
  return (
    <svg className={["sp-outline", className].filter(Boolean).join(" ")} style={insetStyle} aria-hidden>
      <rect
        x={0}
        y={0}
        width="100%"
        height="100%"
        rx={Math.max(0, radius - inset)}
        ry={Math.max(0, radius - inset)}
        fill="none"
        stroke={color}
        strokeWidth={strokeWidth}
      />
    </svg>
  );
}

function RegionOutline({
  regionRef,
  targets,
  shape = boxShape,
  radius = DEFAULT_RADIUS,
  color = "var(--sp-accent)",
  strokeWidth = 1.5,
  className,
  revision,
}: OutlineProps & { regionRef: RefObject<HTMLElement | null>; targets: OutlineTarget[] }) {
  const [path, setPath] = useState("");

  function compute() {
    const region = regionRef.current;
    if (!region) return;
    const origin = region.getBoundingClientRect();
    const rects = targets.map((t) => {
      const el = resolve(t);
      return el ? rectOf(el, origin) : null;
    });
    if (!rects[0]) {
      setPath("");
      return;
    }
    setPath(shape(rects.filter(Boolean) as OutlineRect[], radius, strokeWidth));
  }

  useLayoutEffect(() => {
    compute();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [revision]);

  useLayoutEffect(() => {
    const region = regionRef.current;
    if (!region) return;
    compute();
    const ro = new ResizeObserver(compute);
    ro.observe(region);
    for (const t of targets) {
      const el = resolve(t);
      if (el) ro.observe(el);
    }
    window.addEventListener("resize", compute);
    return () => {
      ro.disconnect();
      window.removeEventListener("resize", compute);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [regionRef, radius, strokeWidth]);

  if (!path) return null;

  return (
    <svg className={["sp-outline", className].filter(Boolean).join(" ")} aria-hidden>
      <path d={path} fill="none" stroke={color} strokeWidth={strokeWidth} vectorEffect="non-scaling-stroke" />
    </svg>
  );
}
