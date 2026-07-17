import type { ReactNode } from "react";
import { IconButton } from "../../widgets/IconButton";
import { IconPin, IconPinOff, IconClose, IconFloat } from "../../infra/icons";
import { startPanelDrag, endPanelDrag } from "../dnd";

export interface PanelSurfaceProps {
  panelId: string;
  title: string;
  pinned: boolean;
  onTogglePin: () => void;
  onClose: () => void;
  /** Omit to hide the pop-out action (e.g. a panel that's already floating). */
  onFloat?: () => void;
  children: ReactNode;
}

/**
 * Chrome shared by every tool window regardless of where it currently
 * lives (docked, flyout, or floating): a draggable title bar — drag it onto
 * any dock strip to redock, or onto the center to join its tabs — plus
 * pin/float/close actions.
 */
export function PanelSurface({ panelId, title, pinned, onTogglePin, onClose, onFloat, children }: PanelSurfaceProps) {
  return (
    <div className="sp-panel-surface">
      <div
        className="sp-panel-surface-header"
        draggable
        onDragStart={(e) => startPanelDrag(e, panelId)}
        onDragEnd={endPanelDrag}
      >
        <span className="sp-panel-surface-title">{title}</span>
        <div className="sp-panel-surface-actions">
          <IconButton size={24} title={pinned ? "Auto-hide" : "Pin"} onClick={onTogglePin}>
            {pinned ? <IconPinOff size={20} /> : <IconPin size={20} />}
          </IconButton>
          {onFloat && (
            <IconButton size={24} title="Float" onClick={onFloat}>
              <IconFloat size={20} />
            </IconButton>
          )}
          <IconButton size={24} title="Close" onClick={onClose}>
            <IconClose size={20} />
          </IconButton>
        </div>
      </div>
      <div className="sp-panel-surface-body">{children}</div>
    </div>
  );
}
