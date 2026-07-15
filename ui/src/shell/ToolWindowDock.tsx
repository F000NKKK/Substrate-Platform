import { useEffect, useRef } from "react";
import { DockStrip } from "./DockStrip";
import { PanelSurface } from "./PanelSurface";
import type { ShellLayout } from "./useShellLayout";
import type { ToolWindowAnchor } from "./types";

export interface ToolWindowDockProps {
  anchor: ToolWindowAnchor;
  layout: ShellLayout;
}

/**
 * One edge's worth of VS-style tool windows: a strip of tabs that's always
 * visible, plus at most one open panel at a time — either "pinned" (docked
 * inline, takes layout space) or a "flyout" (floats over the main area and
 * auto-closes on outside click, i.e. real auto-hide). Any tab or panel
 * header can be dragged to another dock's strip to redock there, or out
 * onto the main area to pop it into a floating panel.
 */
export function ToolWindowDock({ anchor, layout }: ToolWindowDockProps) {
  const panelIds = layout.idsByAnchor(anchor);
  const activeId = layout.activeInAnchor(anchor);
  const active = activeId ? layout.panelsById[activeId] : null;
  const placement = activeId ? layout.placements[activeId] : null;
  const pinned = placement && placement.anchor !== "float" && placement.mode === "pinned";
  const flyout = placement && placement.anchor !== "float" && placement.mode === "flyout";

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

  const dockedPanel = active && ActiveComponent && (pinned || flyout) && (
    <PanelSurface
      panelId={activeId!}
      title={active.title}
      pinned={!!pinned}
      onTogglePin={() => (pinned ? layout.unpin(activeId!) : layout.pin(activeId!))}
      onClose={() => layout.close(activeId!)}
    >
      <ActiveComponent />
    </PanelSurface>
  );

  return (
    <div className={`sp-dock sp-dock--${anchor}`}>
      {anchor !== "right" && pinned && dockedPanel}

      <DockStrip
        anchor={anchor}
        panelIds={panelIds}
        panelsById={layout.panelsById}
        activeId={flyout || pinned ? activeId : null}
        onSelect={layout.toggle}
        onDropPanel={layout.dockTo}
      />

      {anchor === "right" && pinned && dockedPanel}

      {flyout && (
        <div ref={flyoutRef} className={`sp-dock-flyout sp-dock-flyout--${anchor}`}>
          {dockedPanel}
        </div>
      )}
    </div>
  );
}
