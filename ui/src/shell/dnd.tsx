import type { DragEvent } from "react";
import type { ToolWindowAnchor } from "./types";

/** MIME type used for the single piece of data every panel drag carries: its id. */
export const PANEL_DRAG_MIME = "application/x-substrate-panel-id";

const DRAGGING_OPACITY = "0.35";

export function startPanelDrag(e: DragEvent, panelId: string): void {
  e.dataTransfer.setData(PANEL_DRAG_MIME, panelId);
  e.dataTransfer.effectAllowed = "move";
  // The browser's own drag-ghost image isn't reliable on every WebView, so
  // dim the source itself as a fallback "something is being dragged" cue.
  (e.currentTarget as HTMLElement).style.opacity = DRAGGING_OPACITY;
}

export function endPanelDrag(e: DragEvent): void {
  (e.currentTarget as HTMLElement).style.opacity = "";
}

export function readPanelDrag(e: DragEvent): string | null {
  const id = e.dataTransfer.getData(PANEL_DRAG_MIME);
  return id || null;
}

export type DropZone = ToolWindowAnchor | "center";

const EDGE_FRACTION = 0.22;

/** Which VS-style drop zone a point falls into within a rect: an edge (redock) or the center (float). */
export function zoneFromPoint(clientX: number, clientY: number, rect: DOMRect): DropZone {
  const x = clientX - rect.left;
  const y = clientY - rect.top;
  if (x < rect.width * EDGE_FRACTION) return "left";
  if (x > rect.width * (1 - EDGE_FRACTION)) return "right";
  if (y > rect.height * (1 - EDGE_FRACTION)) return "bottom";
  return "center";
}
