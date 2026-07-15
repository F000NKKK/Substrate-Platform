import type { ReactNode } from "react";
import { IconPin, IconPinOff, IconClose } from "../icons";
import { startPanelDrag } from "./dnd";

export interface PanelSurfaceProps {
  panelId: string;
  title: string;
  pinned: boolean;
  onTogglePin: () => void;
  onClose: () => void;
  children: ReactNode;
}

/**
 * Chrome shared by every tool window regardless of where it currently
 * lives (docked, flyout, or floating): a draggable title bar — drag it onto
 * any dock strip to redock, or it's already floating — plus pin/close.
 */
export function PanelSurface({ panelId, title, pinned, onTogglePin, onClose, children }: PanelSurfaceProps) {
  return (
    <div className="sp-panel-surface">
      <div className="sp-panel-surface-header" draggable onDragStart={(e) => startPanelDrag(e, panelId)}>
        <span className="sp-panel-surface-title">{title}</span>
        <div className="sp-panel-surface-actions">
          <button className="sp-panel-surface-btn" title={pinned ? "Auto-hide" : "Pin"} onClick={onTogglePin}>
            {pinned ? <IconPinOff size={13} /> : <IconPin size={13} />}
          </button>
          <button className="sp-panel-surface-btn" title="Close" onClick={onClose}>
            <IconClose size={13} />
          </button>
        </div>
      </div>
      <div className="sp-panel-surface-body">{children}</div>
    </div>
  );
}
