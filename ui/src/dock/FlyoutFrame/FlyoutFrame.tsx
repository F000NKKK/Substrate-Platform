import { useEffect, useRef, type ReactNode, type RefObject } from "react";
import type { ToolWindowAnchor } from "../types";

export interface FlyoutFrameProps {
  anchor: ToolWindowAnchor;
  /** The dock region the flyout lives in — used only to locate this flyout's own strip tab (so clicking that tab isn't treated as an outside click). */
  regionRef: RefObject<HTMLElement | null>;
  /** This panel's own remembered flyout width (left/right) or height (bottom). */
  size: number;
  /** Rendered as a sibling of the flyout box, not inside it — the box clips its content (rounded corners + overflow), which would cut off a resize handle meant to poke out past its edge. */
  resizeHandle?: ReactNode;
  /**
   * Fires on any pointerdown outside this flyout — its own box, its own strip
   * tab, and its own resize handle are the only exceptions. Deliberately NOT
   * scoped to the whole dock region: a panel pinned on the same anchor (e.g.
   * Properties pinned next to a Solution Explorer flyout) lives in that same
   * region, and clicking it should close the flyout like any other outside
   * click.
   */
  onOutsideClick: () => void;
  children: ReactNode;
}

/**
 * A peeked-open auto-hide flyout: its panel laid over the main area next to
 * the strip. The panel's accent outline comes from `PanelSurface` itself (the
 * same one a pinned panel draws), so a panel looks identical pinned or
 * peeking — this frame only handles the flyout's positioning, its resize
 * handle, and dismiss-on-outside-click.
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
    </>
  );
}
