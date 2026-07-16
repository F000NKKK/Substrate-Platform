import { Tab, type TabOrientation } from "../../widgets/Tab";
import { startPanelDrag, endPanelDrag } from "../dnd";
import { useDropTarget } from "../useDropTarget";
import type { PanelDef, ToolWindowAnchor } from "../types";

export interface DockStripProps {
  anchor: ToolWindowAnchor;
  panelIds: string[];
  panelsById: Record<string, PanelDef>;
  activeId: string | null;
  onSelect: (panelId: string) => void;
  onDropPanel: (panelId: string, anchor: ToolWindowAnchor) => void;
}

const ORIENTATION: Record<ToolWindowAnchor, TabOrientation> = {
  left: "vertical-left",
  right: "vertical-right",
  bottom: "horizontal-dock",
};

/**
 * The always-visible strip of collapsed tool-window tabs along one edge of
 * the shell. Also a drop target: dragging any panel's tab/header here
 * redocks it to this anchor.
 */
export function DockStrip({ anchor, panelIds, panelsById, activeId, onSelect, onDropPanel }: DockStripProps) {
  const { dragOver, handlers } = useDropTarget((id) => onDropPanel(id, anchor));

  return (
    <div className={`sp-dock-strip sp-dock-strip--${anchor}`} data-drag-over={dragOver} {...handlers}>
      {panelIds.map((id) => {
        const panel = panelsById[id];
        if (!panel) return null;
        return (
          <Tab
            key={id}
            orientation={ORIENTATION[anchor]}
            active={id === activeId}
            draggable
            onDragStart={(e) => startPanelDrag(e, id)}
            onDragEnd={endPanelDrag}
            onClick={() => onSelect(id)}
            title={panel.title}
          >
            {panel.title}
          </Tab>
        );
      })}
    </div>
  );
}
