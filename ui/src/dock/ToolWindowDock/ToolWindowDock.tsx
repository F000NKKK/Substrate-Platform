import { useEffect, useRef, type CSSProperties, type PointerEvent as ReactPointerEvent } from "react";
import { DockStrip } from "../DockStrip/DockStrip";
import { PanelSurface } from "../PanelSurface/PanelSurface";
import { FlyoutFrame } from "../FlyoutFrame/FlyoutFrame";
import type { ShellLayout } from "../hooks/useShellLayout";
import type { ToolWindowAnchor } from "../types";

export interface ToolWindowDockProps {
  anchor: ToolWindowAnchor;
  layout: ShellLayout;
}

const DEFAULT_FLOAT_POS = { x: 160, y: 120 };

/**
 * Drag handle on one panel's edge facing the main area (or the next pinned
 * panel) — resizes that specific panel, in its current pinned/flyout mode,
 * independently of every other panel or mode. Left/right drag horizontally;
 * bottom drags its top edge upward to grow, matching how every other docking
 * IDE resizes a bottom panel.
 */
function ResizeHandle({
  anchor,
  panelId,
  mode,
  layout,
  className,
}: {
  anchor: ToolWindowAnchor;
  panelId: string;
  mode: "pinned" | "flyout";
  layout: ShellLayout;
  className?: string;
}) {
  function handlePointerDown(e: ReactPointerEvent) {
    e.preventDefault();
    const startSize = layout.anchorSize(panelId, anchor, mode);
    const startX = e.clientX;
    const startY = e.clientY;

    function onMove(ev: PointerEvent) {
      if (anchor === "left") layout.setAnchorSize(panelId, anchor, mode, startSize + (ev.clientX - startX));
      else if (anchor === "right") layout.setAnchorSize(panelId, anchor, mode, startSize - (ev.clientX - startX));
      else layout.setAnchorSize(panelId, anchor, mode, startSize - (ev.clientY - startY));
    }
    function onUp() {
      window.removeEventListener("pointermove", onMove);
      window.removeEventListener("pointerup", onUp);
    }
    window.addEventListener("pointermove", onMove);
    window.addEventListener("pointerup", onUp);
  }

  return (
    <div className={["sp-dock-resize", `sp-dock-resize--${anchor}`, className].filter(Boolean).join(" ")} onPointerDown={handlePointerDown} />
  );
}

function sizeStyle(anchor: ToolWindowAnchor, size: number): CSSProperties {
  return anchor === "bottom" ? { height: size } : { width: size };
}

/**
 * One edge's worth of VS-style tool windows. The collapsed tab strip always
 * sits at the true window edge; any panels pinned to this anchor sit between
 * it and the main area as their own slots, side by side — pinning a second one
 * does not evict the first, and each remembers its own size independently of
 * every other panel and of its own flyout size. At most one more panel can be
 * peeking open as a flyout, which floats OVER everything — at the exact same
 * offset from the strip a pinned panel would sit at, so nothing visibly jumps
 * when a panel is pinned or unpinned — right next to the strip, wrapped
 * together with its still-visible strip tab by a single continuous accent
 * outline. Any tab or header can be dragged to another dock to redock, onto
 * the center to join its tabs, or floated out.
 */
export function ToolWindowDock({ anchor, layout }: ToolWindowDockProps) {
  const panelIds = layout.idsByAnchor(anchor);
  const pinnedIds = layout.pinnedInAnchor(anchor);
  const flyoutId = layout.flyoutInAnchor(anchor);
  const flyoutPanel = flyoutId ? layout.panelsById[flyoutId] : null;

  const regionRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!flyoutId) return;
    const onPointerDown = (e: PointerEvent) => {
      if (!regionRef.current?.contains(e.target as Node)) layout.close(flyoutId);
    };
    document.addEventListener("pointerdown", onPointerDown);
    return () => document.removeEventListener("pointerdown", onPointerDown);
  }, [flyoutId, layout]);

  if (panelIds.length === 0) return null;

  // A pinned panel's own header already shows its title, so it doesn't also
  // need a redundant tab in the strip. A flyout's tab, on the other hand,
  // stays in the strip — it's the notched part of the continuous outline.
  const stripIds = panelIds.filter((id) => !pinnedIds.includes(id));

  const pinnedSlots = pinnedIds.map((id) => {
    const panel = layout.panelsById[id];
    const Component = panel?.component;
    if (!panel || !Component) return null;
    return (
      <div key={id} className={`sp-dock-pinned-slot sp-dock-pinned-slot--${anchor}`}>
        <PanelSurface
          panelId={id}
          title={panel.title}
          pinned
          style={sizeStyle(anchor, layout.anchorSize(id, anchor, "pinned"))}
          onTogglePin={() => layout.unpin(id)}
          onFloat={() => layout.floatAt(id, DEFAULT_FLOAT_POS.x, DEFAULT_FLOAT_POS.y)}
          onClose={() => layout.close(id)}
        >
          <Component />
        </PanelSurface>
        <ResizeHandle anchor={anchor} panelId={id} mode="pinned" layout={layout} />
      </div>
    );
  });

  const strip = (
    <DockStrip
      anchor={anchor}
      panelIds={stripIds}
      panelsById={layout.panelsById}
      activeId={flyoutId}
      onSelect={layout.toggle}
      onDropPanel={layout.dockTo}
    />
  );

  return (
    <div className={`sp-dock sp-dock--${anchor}`} ref={regionRef}>
      {anchor === "left" && strip}
      {pinnedSlots}
      {anchor !== "left" && strip}

      {flyoutPanel && (
        <FlyoutFrame key={flyoutId} anchor={anchor} regionRef={regionRef} size={layout.anchorSize(flyoutId!, anchor, "flyout")}>
          <PanelSurface
            panelId={flyoutId!}
            title={flyoutPanel.title}
            pinned={false}
            onTogglePin={() => layout.pin(flyoutId!)}
            onFloat={() => layout.floatAt(flyoutId!, DEFAULT_FLOAT_POS.x, DEFAULT_FLOAT_POS.y)}
            onClose={() => layout.close(flyoutId!)}
          >
            {(() => {
              const Component = flyoutPanel.component;
              return <Component />;
            })()}
          </PanelSurface>
          <ResizeHandle anchor={anchor} panelId={flyoutId!} mode="flyout" layout={layout} className="sp-dock-resize--flyout" />
        </FlyoutFrame>
      )}
    </div>
  );
}
