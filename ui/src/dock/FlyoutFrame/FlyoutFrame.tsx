import { useEffect, useRef, type ReactNode, type RefObject } from "react";
import { Outline } from "../../infra/outline";
import type { ToolWindowAnchor } from "../types";

export interface FlyoutFrameProps {
  anchor: ToolWindowAnchor;
  /** The dock region the flyout lives in — used as the coordinate origin and to locate the active strip tab. */
  regionRef: RefObject<HTMLElement | null>;
  /** This panel's own remembered flyout width (left/right) or height (bottom). */
  size: number;
  /** Rendered as a sibling of the flyout box, not inside it — the box clips its content (rounded corners), which would cut off a resize handle meant to poke out past its edge. */
  resizeHandle?: ReactNode;
  /**
   * Fires on any pointerdown outside this flyout's own panel + its strip
   * tab. Deliberately scoped to just those two elements rather than the
   * whole dock region: a pinned panel sharing the same anchor (e.g. Properties
   * pinned next to Solution Explorer peeking open) lives in that same region
   * too, and clicking it isn't "still interacting with the flyout" — it
   * should close it like clicking anywhere else outside would.
   */
  onOutsideClick: () => void;
  children: ReactNode;
}

/**
 * A peeked-open auto-hide flyout: its panel, laid over the main area next to
 * the strip, wrapped together with its strip tab by ONE continuous accent
 * outline — the shared `Outline` engine's notched-union case (panel +
 * still-visible strip tab), the same renderer a pinned panel's own outline
 * goes through (see ToolWindowDock.tsx), so a panel never visibly changes
 * shape between the two states.
 */
export function FlyoutFrame({ anchor, regionRef, size, resizeHandle, onOutsideClick, children }: FlyoutFrameProps) {
  const panelRef = useRef<HTMLDivElement>(null);
  const handleRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const onPointerDown = (e: PointerEvent) => {
      const target = e.target as Node;
      const tabEl = regionRef.current?.querySelector<HTMLElement>('.sp-dock-strip [data-active="true"]');
      // Only THIS flyout's own resize handle counts as "still interacting
      // with it" — a different panel's handle (e.g. Properties pinned next
      // to this Solution Explorer flyout) is unrelated and should close it
      // like any other outside click, same as clicking that panel itself.
      if (panelRef.current?.contains(target) || tabEl?.contains(target) || handleRef.current?.contains(target)) return;
      onOutsideClick();
    };
    document.addEventListener("pointerdown", onPointerDown);
    return () => document.removeEventListener("pointerdown", onPointerDown);
  }, [regionRef, onOutsideClick]);

  const sizeStyle = anchor === "bottom" ? { height: size } : { width: size };

  return (
    <>
      <div ref={panelRef} className={`sp-dock-flyout sp-dock-flyout--${anchor}`} style={sizeStyle}>
        {children}
      </div>
      <div ref={handleRef} style={{ display: "contents" }}>
        {resizeHandle}
      </div>
      <Outline
        regionRef={regionRef}
        targets={[panelRef, () => regionRef.current?.querySelector<HTMLElement>('.sp-dock-strip [data-active="true"]') ?? null]}
        notchSide={anchor}
        className="sp-flyout-border"
        syncWith={size}
      />
    </>
  );
}
