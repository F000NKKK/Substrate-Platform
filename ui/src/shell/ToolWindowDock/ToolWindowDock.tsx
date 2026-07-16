import { useEffect, useRef } from "react";
import { DockStrip } from "./DockStrip";
import { PanelSurface } from "./PanelSurface";
import { FlyoutFrame } from "./FlyoutFrame";
import type { ShellLayout } from "./useShellLayout";
import type { ToolWindowAnchor } from "./types";

export interface ToolWindowDockProps {
  anchor: ToolWindowAnchor;
  layout: ShellLayout;
}

const DEFAULT_FLOAT_POS = { x: 160, y: 120 };

/**
 * One edge's worth of VS-style tool windows. The collapsed tab strip always
 * sits at the true window edge; any panels pinned to this anchor sit between
 * it and the main area as their own slots, side by side — pinning a second one
 * does not evict the first. At most one more panel can be peeking open as a
 * flyout, which floats OVER everything (its own size, never joining the pinned
 * row) right next to the strip, wrapped together with its still-visible strip
 * tab by a single continuous accent outline. Any tab or header can be dragged
 * to another dock to redock, onto the center to join its tabs, or floated out.
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

  const pinnedPanels = pinnedIds.map((id) => {
    const panel = layout.panelsById[id];
    const Component = panel?.component;
    if (!panel || !Component) return null;
    return (
      <PanelSurface
        key={id}
        panelId={id}
        title={panel.title}
        pinned
        onTogglePin={() => layout.unpin(id)}
        onFloat={() => layout.floatAt(id, DEFAULT_FLOAT_POS.x, DEFAULT_FLOAT_POS.y)}
        onClose={() => layout.close(id)}
      >
        <Component />
      </PanelSurface>
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
      {pinnedPanels}
      {anchor !== "left" && strip}

      {flyoutPanel && (
        <FlyoutFrame key={flyoutId} anchor={anchor} regionRef={regionRef}>
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
        </FlyoutFrame>
      )}
    </div>
  );
}
