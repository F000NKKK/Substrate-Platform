import { useState, type DragEvent } from "react";
import { Tab } from "../widgets";
import { startPanelDrag, endPanelDrag, readPanelDrag } from "./dnd";
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
  const [dragOver, setDragOver] = useState(false);
  const active = layout.panelsById[layout.centerActiveId];
  const ActiveComponent = active?.component;

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
    if (id) layout.dockTo(id, "center");
  }

  return (
    <div className="sp-center-dock">
      <div
        className="sp-center-tabs"
        data-drag-over={dragOver}
        onDragEnter={handleDragEnter}
        onDragOver={handleDragOver}
        onDragLeave={() => setDragOver(false)}
        onDrop={handleDrop}
      >
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
      <div className="sp-center-body" onDragEnter={handleDragEnter} onDragOver={handleDragOver} onDragLeave={() => setDragOver(false)} onDrop={handleDrop}>
        {ActiveComponent && <ActiveComponent />}
      </div>
    </div>
  );
}
