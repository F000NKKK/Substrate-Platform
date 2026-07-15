import { useState, type DragEvent } from "react";
import { Tab, type TabOrientation } from "../widgets";
import { startPanelDrag, endPanelDrag, readPanelDrag } from "./dnd";
import type { PanelDef, ToolWindowAnchor } from "./types";

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
  bottom: "horizontal",
};

/**
 * The always-visible strip of collapsed tool-window tabs along one edge of
 * the shell. Also a drop target: dragging any panel's tab/header here
 * redocks it to this anchor.
 */
export function DockStrip({ anchor, panelIds, panelsById, activeId, onSelect, onDropPanel }: DockStripProps) {
  const [dragOver, setDragOver] = useState(false);

  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    setDragOver(true);
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    setDragOver(false);
    const id = readPanelDrag(e);
    if (id) onDropPanel(id, anchor);
  }

  return (
    <div
      className={`sp-dock-strip sp-dock-strip--${anchor}`}
      data-drag-over={dragOver}
      onDragEnter={handleDragEnter}
      onDragOver={handleDragOver}
      onDragLeave={() => setDragOver(false)}
      onDrop={handleDrop}
    >
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
