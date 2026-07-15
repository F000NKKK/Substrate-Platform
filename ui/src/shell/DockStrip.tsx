import type { DragEvent } from "react";
import { startPanelDrag, readPanelDrag } from "./dnd";
import type { PanelDef, ToolWindowAnchor } from "./types";

export interface DockStripProps {
  anchor: ToolWindowAnchor;
  panelIds: string[];
  panelsById: Record<string, PanelDef>;
  activeId: string | null;
  onSelect: (panelId: string) => void;
  onDropPanel: (panelId: string, anchor: ToolWindowAnchor) => void;
}

/**
 * The always-visible strip of collapsed tool-window tabs along one edge of
 * the shell — plain rotated text, no icons, same as Visual Studio's own
 * tool-window tabs. Also a drop target: dragging any panel's tab/header
 * here redocks it to this anchor.
 */
export function DockStrip({ anchor, panelIds, panelsById, activeId, onSelect, onDropPanel }: DockStripProps) {
  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    const id = readPanelDrag(e);
    if (id) onDropPanel(id, anchor);
  }

  return (
    <div className={`sp-dock-strip sp-dock-strip--${anchor}`} onDragOver={handleDragOver} onDrop={handleDrop}>
      {panelIds.map((id) => {
        const panel = panelsById[id];
        if (!panel) return null;
        return (
          <button
            key={id}
            className="sp-dock-tab"
            data-active={id === activeId}
            draggable
            onDragStart={(e) => startPanelDrag(e, id)}
            onClick={() => onSelect(id)}
            title={panel.title}
          >
            <span className="sp-dock-tab-title">{panel.title}</span>
          </button>
        );
      })}
    </div>
  );
}
