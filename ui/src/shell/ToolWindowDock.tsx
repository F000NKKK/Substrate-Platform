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
 * sits at the true window edge, with the docked panel (when pinned or open
 * as a flyout) between it and the main area — never the other way round.
 * Any tab or panel header can be dragged to another dock to redock, onto
 * the center to join its tabs, or floated out via its header's pop-out
 * button.
 */
export function ToolWindowDock({ anchor, layout }: ToolWindowDockProps) {
  const panelIds = layout.idsByAnchor(anchor);
  const activeId = layout.activeInAnchor(anchor);
  const active = activeId ? layout.panelsById[activeId] : null;
  const placement = activeId ? layout.placements[activeId] : null;
  const pinned = !!placement && placement.anchor !== "float" && placement.mode === "pinned";
  const flyout = !!placement && placement.anchor !== "float" && placement.mode === "flyout";

  const flyoutRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!flyout || !activeId) return;
    const onPointerDown = (e: PointerEvent) => {
      if (!flyoutRef.current?.contains(e.target as Node)) layout.close(activeId);
    };
    document.addEventListener("pointerdown", onPointerDown);
    return () => document.removeEventListener("pointerdown", onPointerDown);
  }, [flyout, activeId, layout]);

  if (panelIds.length === 0) return null;

  const ActiveComponent = active?.component ?? null;

  // A pinned panel's own header already shows its title, so it doesn't also
  // need a redundant tab sitting in the collapsed strip.
  const stripIds = panelIds.filter((id) => !(id === activeId && pinned));

  const dockedPanel = active && ActiveComponent && (pinned || flyout) && (
    <PanelSurface
      panelId={activeId!}
      title={active.title}
      pinned={pinned}
      onTogglePin={() => (pinned ? layout.unpin(activeId!) : layout.pin(activeId!))}
      onFloat={() => layout.floatAt(activeId!, DEFAULT_FLOAT_POS.x, DEFAULT_FLOAT_POS.y)}
      onClose={() => layout.close(activeId!)}
    >
      <ActiveComponent />
    </PanelSurface>
  );

  const strip = (
    <DockStrip
      anchor={anchor}
      panelIds={stripIds}
      panelsById={layout.panelsById}
      activeId={flyout ? activeId : null}
      onSelect={layout.toggle}
      onDropPanel={layout.dockTo}
    />
  );

  return (
    <div className={`sp-dock sp-dock--${anchor}`}>
      {anchor === "left" && strip}
      {pinned && dockedPanel}
      {anchor !== "left" && strip}

      {flyout && (
        <div ref={flyoutRef} className={`sp-dock-flyout sp-dock-flyout--${anchor}`}>
          {dockedPanel}
        </div>
      )}
    </div>
  );
}
