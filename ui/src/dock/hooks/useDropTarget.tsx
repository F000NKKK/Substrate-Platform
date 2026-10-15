import { useState, type DragEventHandler } from "react";
import { isPanelDrag, readPanelDrag } from "../dnd";

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
 * readPanelDrag boilerplate. Drags that aren't panels are left alone, so
 * they reach whatever inside actually accepts them.
 */
export function useDropTarget(onDropPanel: (panelId: string) => void): DropTarget {
  const [dragOver, setDragOver] = useState(false);

  return {
    dragOver,
    handlers: {
      onDragEnter: (e) => {
        if (!isPanelDrag(e)) return;
        e.preventDefault();
        setDragOver(true);
      },
      onDragOver: (e) => {
        if (!isPanelDrag(e)) return;
        e.preventDefault();
        // Ours, not the shell's guides under it.
        e.stopPropagation();
        e.dataTransfer.dropEffect = "move";
      },
      onDragLeave: () => setDragOver(false),
      onDrop: (e) => {
        setDragOver(false);
        const id = readPanelDrag(e);
        if (!id) return;
        e.preventDefault();
        e.stopPropagation();
        onDropPanel(id);
      },
    },
  };
}
