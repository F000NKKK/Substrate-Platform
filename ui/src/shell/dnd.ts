/** MIME type used for the single piece of data every panel drag carries: its id. */
export const PANEL_DRAG_MIME = "application/x-substrate-panel-id";

export function startPanelDrag(e: React.DragEvent, panelId: string): void {
  e.dataTransfer.setData(PANEL_DRAG_MIME, panelId);
  e.dataTransfer.effectAllowed = "move";
}

export function readPanelDrag(e: React.DragEvent): string | null {
  const id = e.dataTransfer.getData(PANEL_DRAG_MIME);
  return id || null;
}
