import type { DragEvent } from "react";

/** MIME type used for the single piece of data every panel drag carries: its id. */
export const PANEL_DRAG_MIME = "application/x-substrate-panel-id";

const DRAGGING_OPACITY = "0.35";

export function startPanelDrag(e: DragEvent, panelId: string): void {
  e.dataTransfer.setData(PANEL_DRAG_MIME, panelId);
  e.dataTransfer.effectAllowed = "move";
  const source = e.currentTarget as HTMLElement;
  // Under the pointer: the panel's name as a tab, not the browser's faint
  // snapshot of the header (its buttons and all).
  const title = (source.querySelector(".sp-panel-surface-title") ?? source).textContent?.trim() || panelId;
  const ghost = document.createElement("div");
  ghost.className = "sp-panel-drag-ghost";
  ghost.textContent = title;
  document.body.appendChild(ghost);
  e.dataTransfer.setDragImage(ghost, 16, 14);
  // The image is taken now; the element can go.
  setTimeout(() => ghost.remove(), 0);
  // And the source dims, a cue where the drag image isn't shown (some WebViews).
  source.style.opacity = DRAGGING_OPACITY;
}

export function endPanelDrag(e: DragEvent): void {
  (e.currentTarget as HTMLElement).style.opacity = "";
}

/**
 * Whether a drag in progress carries a panel — the only drags the dock reacts
 * to. Only the types are readable before the drop, so this is what
 * dragenter/dragover check; anything else (a toolbox item headed for a
 * designer, files from the OS) passes through untouched.
 */
export function isPanelDrag(e: DragEvent): boolean {
  return Array.from(e.dataTransfer.types).includes(PANEL_DRAG_MIME);
}

export function readPanelDrag(e: DragEvent): string | null {
  const id = e.dataTransfer.getData(PANEL_DRAG_MIME);
  return id || null;
}
