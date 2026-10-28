import { useEffect, useRef } from "react";
import { ErrorBoundary } from "../../widgets/ErrorBoundary";
import { IconButton } from "../../widgets/IconButton";
import { Tab } from "../../widgets/Tab";
import { IconFloat } from "../../infra/icons";
import { useT } from "../../infra/i18n";
import { startPanelDrag, endPanelDrag } from "../dnd";
import { useDropTarget } from "../hooks/useDropTarget";
import type { ShellLayout } from "../hooks/useShellLayout";
import type { ToolWindowAnchor } from "../types";
import { ResizeHandle, sizeStyle } from "../ToolWindowDock/ToolWindowDock";

export interface TabGroupDockProps {
  anchor: ToolWindowAnchor;
  layout: ShellLayout;
}

const DEFAULT_FLOAT_POS = { x: 160, y: 120 };

/**
 * One edge whose tool windows share a single pane — Unity's Project window:
 * every panel docked here is a tab of the same group, the active one fills
 * the pane, and any panel's tab or title bar can be dragged onto the tab
 * strip to join it (or off it, to another dock). With no tab active the
 * group collapses to its strip.
 *
 * The group's active tab is the panel pinned on this anchor. Pinning another
 * (a click on its tab, View, a drop) makes it the active one and the rest
 * step back to hidden, so the layout's "several pinned side by side" never
 * shows here.
 */
export function TabGroupDock({ anchor, layout }: TabGroupDockProps) {
  const t = useT();
  const ids = layout.idsByAnchor(anchor);
  const pinned = layout.pinnedInAnchor(anchor);
  const lastActive = useRef<string | null>(null);

  // Whichever panel was pinned last wins; the others step back.
  const activeId = pinned.length > 1 ? (pinned.find((id) => id !== lastActive.current) ?? pinned[0]) : (pinned[0] ?? null);
  useEffect(() => {
    lastActive.current = activeId;
    for (const id of pinned) if (id !== activeId) layout.hide(id);
  }, [activeId, pinned, layout]);

  const { dragOver, handlers } = useDropTarget((id) => layout.dockTo(id, anchor));
  if (ids.length === 0) return null;

  const active = activeId ? layout.panelsById[activeId] : null;
  const Component = active?.component;
  const size = activeId ? layout.anchorSize(activeId, anchor, "pinned") : 0;
  const handleFirst = anchor !== "left";
  const handle = activeId && <ResizeHandle key="handle" anchor={anchor} panelId={activeId} mode="pinned" layout={layout} />;

  return (
    <div className={`sp-dock sp-dock--${anchor} sp-dock--tabbed`}>
      <div className={`sp-dock-pinned-slot sp-dock-pinned-slot--${anchor}`} style={active ? sizeStyle(anchor, size) : undefined}>
        {handleFirst && handle}
        <div className="sp-tabgroup">
          <div className="sp-tabgroup-tabs" role="tablist" data-drag-over={dragOver} {...handlers}>
            {ids.map((id) => {
              const panel = layout.panelsById[id];
              if (!panel) return null;
              return (
                <Tab
                  key={id}
                  role="tab"
                  active={id === activeId}
                  draggable
                  onDragStart={(e) => startPanelDrag(e, id)}
                  onDragEnd={endPanelDrag}
                  onClick={() => (id === activeId ? layout.hide(id) : layout.pin(id))}
                  onRequestClose={() => layout.close(id)}
                  title={panel.title}
                >
                  {panel.title}
                </Tab>
              );
            })}
            <span className="sp-tabgroup-spacer" />
            {activeId && (
              <IconButton title={t("Float")} onClick={() => layout.floatAt(activeId, DEFAULT_FLOAT_POS.x, DEFAULT_FLOAT_POS.y)}>
                <IconFloat />
              </IconButton>
            )}
          </div>
          {active && Component && (
            <div className="sp-tabgroup-body">
              <ErrorBoundary what={active.title}>
                <Component />
              </ErrorBoundary>
            </div>
          )}
        </div>
        {!handleFirst && handle}
      </div>
    </div>
  );
}
