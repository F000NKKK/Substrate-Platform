import { useLayoutEffect, useRef, useState, type CSSProperties, type RefObject } from "react";
import "./outline.css";

export interface OutlinePoint {
  x: number;
  y: number;
}

export interface OutlineRect {
  l: number;
  t: number;
  r: number;
  b: number;
}

export type OutlineTarget = RefObject<HTMLElement | null> | (() => HTMLElement | null);

/**
 * A contour: given the measured rects of the targets (in the outline's own
 * coordinate space) plus the styling params, return one SVG path string. This
 * is the public "merge" hook — a component describes how its own targets
 * combine into a contour, built from the exported primitives (`boxRing`,
 * `roundedPath`, …) or anything else, and the engine just strokes the result.
 * Nothing domain-specific (a dock's tab notch, a particular widget's shape)
 * lives in the engine — only here, in each consumer's own shape function.
 */
export type OutlineShape = (rects: OutlineRect[], radius: number, strokeWidth: number) => string;

function resolve(target: OutlineTarget): HTMLElement | null {
  return typeof target === "function" ? target() : target.current;
}

function rectOf(el: HTMLElement, origin: DOMRect): OutlineRect {
  const r = el.getBoundingClientRect();
  return { l: r.left - origin.left, t: r.top - origin.top, r: r.right - origin.left, b: r.bottom - origin.top };
}

function len(a: OutlinePoint, b: OutlinePoint): number {
  return Math.hypot(b.x - a.x, b.y - a.y);
}

/**
 * Primitive: closes an ordered ring of vertices into a rounded-corner SVG path
 * — one quadratic Bézier per vertex, clamped so a corner never eats more than
 * half its shorter neighbouring edge. Convex and concave corners round
 * identically, so any polygon (a plain box, a notched union, an L-shape) rounds
 * with this one routine. The main building block consumers compose shapes from.
 */
export function roundedPath(points: OutlinePoint[], radius: number): string {
  const n = points.length;
  if (n < 3) return "";
  let d = "";
  for (let i = 0; i < n; i++) {
    const prev = points[(i - 1 + n) % n];
    const cur = points[i];
    const next = points[(i + 1) % n];
    const rIn = Math.min(radius, len(prev, cur) / 2);
    const rOut = Math.min(radius, len(next, cur) / 2);
    const before: OutlinePoint = { x: cur.x + ((prev.x - cur.x) / len(prev, cur)) * rIn, y: cur.y + ((prev.y - cur.y) / len(prev, cur)) * rIn };
    const after: OutlinePoint = { x: cur.x + ((next.x - cur.x) / len(next, cur)) * rOut, y: cur.y + ((next.y - cur.y) / len(next, cur)) * rOut };
    d += i === 0 ? `M ${before.x} ${before.y} ` : `L ${before.x} ${before.y} `;
    d += `Q ${cur.x} ${cur.y} ${after.x} ${after.y} `;
  }
  return d + "Z";
}

/** Primitive: the four corners of a rect as a ring, ready for `roundedPath`. */
export function boxRing(r: OutlineRect): OutlinePoint[] {
  return [
    { x: r.l, y: r.t },
    { x: r.r, y: r.t },
    { x: r.r, y: r.b },
    { x: r.l, y: r.b },
  ];
}

/** The default shape — a plain rounded box around the first target. */
export const boxShape: OutlineShape = (rects, radius) => (rects[0] ? roundedPath(boxRing(rects[0]), radius) : "");

const DEFAULT_RADIUS = 8;

export interface OutlineProps {
  radius?: number;
  color?: string;
  strokeWidth?: number;
  className?: string;
  /**
   * Region mode — draw over `regionRef` (a positioned ancestor) and outline
   * the union of `targets`, measured in that region's coordinates and merged
   * by `shape`. Use for a contour spanning MORE than one element. Omit both
   * `regionRef` and `targets` for the default self mode below.
   */
  regionRef?: RefObject<HTMLElement | null>;
  targets?: OutlineTarget[];
  /** The contour (built from the exported primitives). Region mode only; defaults to `boxShape`. Self mode always draws a box filling the parent. */
  shape?: OutlineShape;
  /** Region mode only: bump on every value a driven resize derives from (e.g. the panel's live size) to recompute synchronously in the same commit, instead of a frame later via ResizeObserver. */
  revision?: unknown;
}

/**
 * The platform's one accent-outline renderer. Both modes compute their path
 * the same way — measure some rects, hand them to a `shape` (default
 * `boxShape`) — they differ only in WHAT they measure:
 *
 * • **Self mode** (default — no `regionRef`/`targets`): renders as a CHILD of
 *   the element it outlines and measures that element's own box (in its local
 *   coordinates). Being the element's own child, it inherits every move for
 *   free — a neighbour's resize that merely SHIFTS this element can never
 *   desync the outline, since the local box is unchanged and the child rides
 *   along. Pinned/floating dock panels use this, with `boxShape`.
 *
 * • **Region mode** (`regionRef` + `targets`): renders over a positioned
 *   region and measures several targets in that region's coordinates — the
 *   only way to trace a contour spanning more than one box (e.g. a flyout
 *   panel + its detached strip tab). The engine has no idea what the shape
 *   means; the consumer builds it from the exported primitives.
 */
export function Outline(props: OutlineProps) {
  if (props.regionRef && props.targets) return <RegionOutline {...props} regionRef={props.regionRef} targets={props.targets} />;
  return <SelfOutline {...props} />;
}

function SelfOutline({ shape = boxShape, radius = DEFAULT_RADIUS, color = "var(--sp-accent)", strokeWidth = 1.5, className }: OutlineProps) {
  const ref = useRef<SVGSVGElement>(null);
  const [path, setPath] = useState("");
  const shapeRef = useRef(shape);
  shapeRef.current = shape;

  useLayoutEffect(() => {
    const parent = ref.current?.parentElement;
    if (!parent) return;
    const inset = strokeWidth / 2;
    const compute = () => {
      const w = parent.clientWidth;
      const h = parent.clientHeight;
      // Inset by half the stroke so the stroke's outer edge lands on the
      // parent's own edge and isn't trimmed by the parent's overflow clip.
      const box: OutlineRect = { l: inset, t: inset, r: w - inset, b: h - inset };
      setPath(w > 0 && h > 0 ? shapeRef.current([box], radius, strokeWidth) : "");
    };
    compute();
    const ro = new ResizeObserver(compute);
    ro.observe(parent);
    return () => ro.disconnect();
  }, [radius, strokeWidth]);

  const insetStyle: CSSProperties = { inset: 0 };
  return (
    <svg ref={ref} className={["sp-outline", className].filter(Boolean).join(" ")} style={insetStyle} aria-hidden>
      {path && <path d={path} fill="none" stroke={color} strokeWidth={strokeWidth} vectorEffect="non-scaling-stroke" />}
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
  const shapeRef = useRef(shape);
  shapeRef.current = shape;

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
    setPath(shapeRef.current(rects.filter(Boolean) as OutlineRect[], radius, strokeWidth));
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
