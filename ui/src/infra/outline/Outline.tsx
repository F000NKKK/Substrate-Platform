import { useLayoutEffect, useRef, useState, type CSSProperties } from "react";
import "./outline.css";

export type OutlineShape = (box: { w: number; h: number }, radius: number, strokeWidth: number) => string;

export interface OutlineProps {
  radius?: number;
  color?: string;
  strokeWidth?: number;
  className?: string;
  /**
   * Advanced: a custom contour, as an SVG path in the parent's own local px
   * coordinates, given the parent's measured size. Omit for the default — a
   * rounded rectangle filling the parent — which is drawn measurement-free
   * (a plain SVG `<rect width="100%" height="100%">`) so it can NEVER lag its
   * parent by even a frame, however the parent is moved or resized. A moved
   * parent (e.g. a docked panel shifted by a neighbour's resize) carries this
   * outline along automatically, because the outline is the parent's own
   * child, not a separate overlay chasing it across a shared coordinate space.
   */
  shape?: OutlineShape;
}

const DEFAULT_RADIUS = 8;

/** A rounded-rectangle path for custom-shape authors and the measured path fallback. */
export function roundedRectPath(w: number, h: number, radius: number, inset = 0): string {
  const x = inset;
  const y = inset;
  const right = w - inset;
  const bottom = h - inset;
  const r = Math.max(0, Math.min(radius, (right - x) / 2, (bottom - y) / 2));
  return [
    `M ${x + r} ${y}`,
    `H ${right - r}`,
    `A ${r} ${r} 0 0 1 ${right} ${y + r}`,
    `V ${bottom - r}`,
    `A ${r} ${r} 0 0 1 ${right - r} ${bottom}`,
    `H ${x + r}`,
    `A ${r} ${r} 0 0 1 ${x} ${bottom - r}`,
    `V ${y + r}`,
    `A ${r} ${r} 0 0 1 ${x + r} ${y}`,
    "Z",
  ].join(" ");
}

/**
 * The platform's one accent-outline renderer — an SVG rendered as a CHILD of
 * the element it outlines (its parent must be `position: relative`). Because
 * it lives inside that element, it inherits every move and resize for free:
 * no measuring the element's position, no chasing it across a region, and so
 * nothing to fall out of sync when a sibling's resize shifts it. The default
 * (a rounded rectangle) needs no measurement at all — an SVG `<rect>` sized
 * `100%` tracks the parent purely in the render/paint layer. Pass `shape` for
 * a non-rectangular contour; only then is the parent measured, so custom
 * shapes stay correct across resizes too.
 */
export function Outline({ radius = DEFAULT_RADIUS, color = "var(--sp-accent)", strokeWidth = 1.5, className, shape }: OutlineProps) {
  const ref = useRef<SVGSVGElement>(null);
  const [box, setBox] = useState<{ w: number; h: number }>({ w: 0, h: 0 });

  useLayoutEffect(() => {
    if (!shape) return;
    const parent = ref.current?.parentElement;
    if (!parent) return;
    const measure = () => setBox({ w: parent.clientWidth, h: parent.clientHeight });
    measure();
    const ro = new ResizeObserver(measure);
    ro.observe(parent);
    return () => ro.disconnect();
  }, [shape]);

  const classes = ["sp-outline", className].filter(Boolean).join(" ");

  if (shape) {
    const d = box.w > 0 && box.h > 0 ? shape(box, radius, strokeWidth) : "";
    return (
      <svg ref={ref} className={classes} aria-hidden>
        {d && <path d={d} fill="none" stroke={color} strokeWidth={strokeWidth} />}
      </svg>
    );
  }

  // Default rounded rect: inset the SVG by half the stroke so the stroke's
  // outer edge lands exactly on the parent's edge (crisp, and not trimmed by
  // the parent's own overflow clip), and round the corners a hair tighter to
  // match.
  const inset = strokeWidth / 2;
  const insetStyle: CSSProperties = { inset };
  return (
    <svg ref={ref} className={classes} style={insetStyle} aria-hidden>
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
