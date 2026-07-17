import { useRef, type ReactNode, type RefObject } from "react";
import { Outline } from "../../infra/outline";
import type { ToolWindowAnchor } from "../types";

export interface FlyoutFrameProps {
  anchor: ToolWindowAnchor;
  /** The dock region the flyout lives in — used as the coordinate origin and to locate the active strip tab. */
  regionRef: RefObject<HTMLElement | null>;
  /** This panel's own remembered flyout width (left/right) or height (bottom). */
  size: number;
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
export function FlyoutFrame({ anchor, regionRef, size, children }: FlyoutFrameProps) {
  const panelRef = useRef<HTMLDivElement>(null);

  const sizeStyle = anchor === "bottom" ? { height: size } : { width: size };

  return (
    <>
      <div ref={panelRef} className={`sp-dock-flyout sp-dock-flyout--${anchor}`} style={sizeStyle}>
        {children}
      </div>
      <Outline
        regionRef={regionRef}
        targets={[panelRef, () => regionRef.current?.querySelector<HTMLElement>('.sp-dock-strip [data-active="true"]') ?? null]}
        notchSide={anchor}
        className="sp-flyout-border"
      />
    </>
  );
}
