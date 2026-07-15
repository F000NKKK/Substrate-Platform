import { PanelSurface } from "./PanelSurface";
import type { PanelDef } from "./types";

export interface FloatingPanelProps {
  panel: PanelDef;
  x: number;
  y: number;
  w: number;
  h: number;
  onDock: () => void;
  onClose: () => void;
}

/**
 * A panel pulled out of every dock — a free-floating window inside the
 * shell. Drag its header onto a dock strip to redock it, drop it anywhere
 * else on the main area to move it, or drag its bottom-right corner to
 * resize (plain CSS `resize`, no extra JS needed for that part).
 */
export function FloatingPanel({ panel, x, y, w, h, onDock, onClose }: FloatingPanelProps) {
  const Component = panel.component;
  return (
    <div className="sp-floating-panel" style={{ left: x, top: y, width: w, height: h }}>
      <PanelSurface panelId={panel.id} title={panel.title} pinned={false} onTogglePin={onDock} onClose={onClose}>
        <Component />
      </PanelSurface>
    </div>
  );
}
