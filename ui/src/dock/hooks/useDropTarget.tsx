import { useState, type DragEventHandler } from "react";
import { readPanelDrag } from "../dnd";

export interface DropTargetHandlers {
  onDragEnter: DragEventHandler;
  onDragOver: DragEventHandler;
  onDragLeave: DragEventHandler;
  onDrop: DragEventHandler;
}

export interface DropTarget {
  dragOver: boolean;
  handlers: DropTargetHandlers;
}

/**
 * The one implementation of "this element accepts a dropped panel" — every
 * drop zone (dock strips, the center dock's tab bar and body) wires through
 * this instead of each re-declaring the same preventDefault/dragOver-state/
 * readPanelDrag boilerplate.
 */
export function useDropTarget(onDropPanel: (panelId: string) => void): DropTarget {
  const [dragOver, setDragOver] = useState(false);

  return {
    dragOver,
    handlers: {
      onDragEnter: (e) => {
        e.preventDefault();
        setDragOver(true);
      },
      onDragOver: (e) => {
        e.preventDefault();
        e.dataTransfer.dropEffect = "move";
      },
      onDragLeave: () => setDragOver(false),
      onDrop: (e) => {
        e.preventDefault();
        setDragOver(false);
        const id = readPanelDrag(e);
        if (id) onDropPanel(id);
      },
    },
  };
}
