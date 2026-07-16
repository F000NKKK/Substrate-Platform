import { Tab } from "../widgets";
import { startPanelDrag, endPanelDrag } from "./dnd";
import { useDropTarget } from "./useDropTarget";
import type { ShellLayout } from "./useShellLayout";

export interface CenterDockProps {
  layout: ShellLayout;
}

/**
 * The main content zone — a normal tabbed dock, same as any IDE's document
 * area: it owns its own horizontal tab bar on top and can hold more than
 * just the product's primary panel, since anything can be dragged in to
 * join it as another tab.
 */
export function CenterDock({ layout }: CenterDockProps) {
  const active = layout.panelsById[layout.centerActiveId];
  const ActiveComponent = active?.component;
  const { dragOver, handlers } = useDropTarget((id) => layout.dockTo(id, "center"));

  return (
    <div className="sp-center-dock">
      <div className="sp-center-tabs" data-drag-over={dragOver} {...handlers}>
        {layout.centerIds.map((id) => {
          const panel = layout.panelsById[id];
          if (!panel) return null;
          return (
            <Tab
              key={id}
              orientation="horizontal"
              active={id === layout.centerActiveId}
              draggable
              onDragStart={(e) => startPanelDrag(e, id)}
              onDragEnd={endPanelDrag}
              onClick={() => layout.setCenterActive(id)}
              onRequestClose={id === layout.mainId ? undefined : () => layout.close(id)}
            >
              {panel.title}
            </Tab>
          );
        })}
      </div>
      <div className="sp-center-body" {...handlers}>
        {ActiveComponent && <ActiveComponent />}
      </div>
    </div>
  );
}
