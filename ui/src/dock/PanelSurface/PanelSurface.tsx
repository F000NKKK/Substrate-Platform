import type { CSSProperties, ReactNode, Ref } from "react";
import { IconButton } from "../../widgets/IconButton";
import { IconPin, IconPinOff, IconClose, IconFloat } from "../../infra/icons";
import { Outline } from "../../infra/outline";
import { startPanelDrag, endPanelDrag } from "../dnd";

export interface PanelSurfaceProps {
  panelId: string;
  title: string;
  pinned: boolean;
  onTogglePin: () => void;
  onClose: () => void;
  /** Omit to hide the pop-out action (e.g. a panel that's already floating). */
  onFloat?: () => void;
  /** Sizing override for the root — e.g. a dock's own remembered width/height for this specific panel. */
  style?: CSSProperties;
  /** The root element. */
  ref?: Ref<HTMLDivElement>;
  children: ReactNode;
}

/**
 * Chrome shared by every tool window regardless of where it currently lives
 * (docked, flyout, or floating): a draggable title bar — drag it onto any
 * dock strip to redock, or onto the center to join its tabs — plus
 * pin/float/close actions, and its own accent `Outline`.
 *
 * The outline lives INSIDE the panel (see `Outline`), so a pinned panel, its
 * flyout state, and a floating panel all render one identical accent border
 * that can never drift from the panel it belongs to — the whole class of
 * "the outline lagged behind the panel" bugs is structurally impossible.
 */
export function PanelSurface({ panelId, title, pinned, onTogglePin, onClose, onFloat, style, ref, children }: PanelSurfaceProps) {
  return (
    <div ref={ref} className="sp-panel-surface" style={style}>
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
      <Outline />
    </div>
  );
}
