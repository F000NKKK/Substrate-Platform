import { useEffect, useRef } from "react";
import { DockStrip } from "./DockStrip";
import { PanelSurface } from "./PanelSurface";
import type { ShellLayout } from "./useShellLayout";
import type { ToolWindowAnchor } from "./types";

export interface ToolWindowDockProps {
  anchor: ToolWindowAnchor;
  layout: ShellLayout;
}

const DEFAULT_FLOAT_POS = { x: 160, y: 120 };

/**
 * One edge's worth of VS-style tool windows. The collapsed tab strip always
 * sits at the true window edge; any panels pinned to this anchor sit
 * between it and the main area as their own grid slots, side by side —
 * pinning a second one does not evict the first. At most one more panel
 * can also be peeking open as a flyout, layered over the main area.
 * Any tab or panel header can be dragged to another dock to redock, onto
 * the center to join its tabs, or floated out via its header's pop-out
 * button.
 */
export function ToolWindowDock({ anchor, layout }: ToolWindowDockProps) {
  const panelIds = layout.idsByAnchor(anchor);
  const pinnedIds = layout.pinnedInAnchor(anchor);
  const flyoutId = layout.flyoutInAnchor(anchor);
  const flyoutPanel = flyoutId ? layout.panelsById[flyoutId] : null;

  const flyoutRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!flyoutId) return;
    const onPointerDown = (e: PointerEvent) => {
      if (!flyoutRef.current?.contains(e.target as Node)) layout.close(flyoutId);
    };
    document.addEventListener("pointerdown", onPointerDown);
    return () => document.removeEventListener("pointerdown", onPointerDown);
  }, [flyoutId, layout]);

  if (panelIds.length === 0) return null;

  // A pinned panel's own header already shows its title, so it doesn't also
  // need a redundant tab sitting in the collapsed strip.
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
    <div className={`sp-dock sp-dock--${anchor}`}>
      {anchor === "left" && strip}
      {pinnedPanels}
      {anchor !== "left" && strip}

      {flyoutPanel && (
        <div ref={flyoutRef} className={`sp-dock-flyout sp-dock-flyout--${anchor}`}>
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
        </div>
      )}
    </div>
  );
}
