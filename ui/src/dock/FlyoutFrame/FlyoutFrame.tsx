import { useEffect, useRef, type ReactNode, type RefObject } from "react";
import { Outline, roundedPath, boxRing, type OutlineRect, type OutlineShape } from "../../infra/outline";
import type { ToolWindowAnchor } from "../types";

export interface FlyoutFrameProps {
  anchor: ToolWindowAnchor;
  /** The dock region — the outline's coordinate origin and where this flyout's own strip tab is found. */
  regionRef: RefObject<HTMLElement | null>;
  /** This panel's own remembered flyout width (left/right) or height (bottom). */
  size: number;
  /** Rendered as a sibling of the flyout box, not inside it — the box clips its content, which would cut off a resize handle meant to poke out past its edge. */
  resizeHandle?: ReactNode;
  /**
   * Fires on any pointerdown outside this flyout — its own box, its own strip
   * tab, and its own resize handle are the only exceptions. Deliberately NOT
   * scoped to the whole dock region: a panel pinned on the same anchor lives in
   * that same region, and clicking it should close the flyout like any other
   * outside click.
   */
  onOutsideClick: () => void;
  children: ReactNode;
}

/**
 * The 8-corner ring wrapping the flyout panel (`panel`) together with its
 * still-visible strip tab (`tab`) poking out of the panel's strip-facing edge
 * — the dock's own domain shape, composed from the outline engine's generic
 * primitives (`boxRing`/`roundedPath`) rather than baked into the engine.
 */
function notchedUnionShape(anchor: ToolWindowAnchor): OutlineShape {
  return (rects, radius) => {
    const panel = rects[0];
    const tab = rects[1];
    if (!panel) return "";
    if (!tab) return roundedPath(boxRing(panel), radius);
    const ring = notchedRing(panel, tab, anchor);
    return roundedPath(ring, radius);
  };
}

function notchedRing(panel: OutlineRect, tab: OutlineRect, side: ToolWindowAnchor) {
  switch (side) {
    case "left":
      return [
        { x: tab.l, y: tab.t },
        { x: panel.l, y: tab.t },
        { x: panel.l, y: panel.t },
        { x: panel.r, y: panel.t },
        { x: panel.r, y: panel.b },
        { x: panel.l, y: panel.b },
        { x: panel.l, y: tab.b },
        { x: tab.l, y: tab.b },
      ];
    case "right":
      return [
        { x: panel.l, y: panel.t },
        { x: panel.r, y: panel.t },
        { x: panel.r, y: tab.t },
        { x: tab.r, y: tab.t },
        { x: tab.r, y: tab.b },
        { x: panel.r, y: tab.b },
        { x: panel.r, y: panel.b },
        { x: panel.l, y: panel.b },
      ];
    case "bottom":
      return [
        { x: panel.l, y: panel.t },
        { x: panel.r, y: panel.t },
        { x: panel.r, y: panel.b },
        { x: tab.r, y: panel.b },
        { x: tab.r, y: tab.b },
        { x: tab.l, y: tab.b },
        { x: tab.l, y: panel.b },
        { x: panel.l, y: panel.b },
      ];
  }
}

/**
 * A peeked-open auto-hide flyout: its panel laid over the main area flush
 * against the strip, wrapped together with its still-visible strip tab by ONE
 * continuous accent outline — a region-mode `Outline` using the dock's own
 * notched-union shape. (A pinned panel, having no visible tab, just draws the
 * plain box half of the same idea via `PanelSurface`'s self outline.)
 */
export function FlyoutFrame({ anchor, regionRef, size, resizeHandle, onOutsideClick, children }: FlyoutFrameProps) {
  const boxRef = useRef<HTMLDivElement>(null);
  const handleRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const onPointerDown = (e: PointerEvent) => {
      const target = e.target as Node;
      const tabEl = regionRef.current?.querySelector<HTMLElement>('.sp-dock-strip [data-active="true"]');
      if (boxRef.current?.contains(target) || tabEl?.contains(target) || handleRef.current?.contains(target)) return;
      onOutsideClick();
    };
    document.addEventListener("pointerdown", onPointerDown);
    return () => document.removeEventListener("pointerdown", onPointerDown);
  }, [regionRef, onOutsideClick]);

  const sizeStyle = anchor === "bottom" ? { height: size } : { width: size };

  return (
    <>
      <div ref={boxRef} className={`sp-dock-flyout sp-dock-flyout--${anchor}`} style={sizeStyle}>
        {children}
      </div>
      <div ref={handleRef} style={{ display: "contents" }}>
        {resizeHandle}
      </div>
      <Outline
        regionRef={regionRef}
        targets={[boxRef, () => regionRef.current?.querySelector<HTMLElement>('.sp-dock-strip [data-active="true"]') ?? null]}
        shape={notchedUnionShape(anchor)}
        revision={size}
      />
    </>
  );
}
