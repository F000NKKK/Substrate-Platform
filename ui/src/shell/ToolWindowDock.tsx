import { useEffect, useRef } from "react";
import { DockStrip } from "./DockStrip";
import { useDock } from "./useDock";
import type { PanelDef, ToolWindowAnchor } from "./types";

export interface ToolWindowDockProps {
  anchor: ToolWindowAnchor;
  panels: PanelDef[];
  defaultPinnedId?: string;
}

/**
 * One edge's worth of VS-style tool windows: a strip of tabs that's always
 * visible, plus at most one open panel at a time — either "pinned" (docked
 * inline, takes layout space) or a "flyout" (floats over the main area and
 * auto-closes on outside click, i.e. real auto-hide).
 */
export function ToolWindowDock({ anchor, panels, defaultPinnedId }: ToolWindowDockProps) {
  const dock = useDock(defaultPinnedId ?? null, defaultPinnedId ? "pinned" : "hidden");
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (dock.mode !== "flyout") return;
    const onPointerDown = (e: PointerEvent) => {
      if (!containerRef.current?.contains(e.target as Node)) dock.close();
    };
    document.addEventListener("pointerdown", onPointerDown);
    return () => document.removeEventListener("pointerdown", onPointerDown);
  }, [dock.mode, dock]);

  if (panels.length === 0) return null;

  const active = panels.find((p) => p.id === dock.activeId) ?? null;
  const ActiveComponent = active?.component;

  return (
    <div ref={containerRef} className={`sp-dock sp-dock--${anchor}`}>
      {anchor !== "right" && dock.mode === "pinned" && active && ActiveComponent && (
        <PanelSurface anchor={anchor} title={active.title} pinned onTogglePin={dock.unpin} onClose={dock.close}>
          <ActiveComponent />
        </PanelSurface>
      )}

      <DockStrip anchor={anchor} panels={panels} activeId={dock.mode === "hidden" ? null : dock.activeId} onSelect={dock.toggle} />

      {anchor === "right" && dock.mode === "pinned" && active && ActiveComponent && (
        <PanelSurface anchor={anchor} title={active.title} pinned onTogglePin={dock.unpin} onClose={dock.close}>
          <ActiveComponent />
        </PanelSurface>
      )}

      {dock.mode === "flyout" && active && ActiveComponent && (
        <div className={`sp-dock-flyout sp-dock-flyout--${anchor}`}>
          <PanelSurface anchor={anchor} title={active.title} pinned={false} onTogglePin={dock.pin} onClose={dock.close}>
            <ActiveComponent />
          </PanelSurface>
        </div>
      )}
    </div>
  );
}

interface PanelSurfaceProps {
  anchor: ToolWindowAnchor;
  title: string;
  pinned: boolean;
  onTogglePin: () => void;
  onClose: () => void;
  children: React.ReactNode;
}

function PanelSurface({ title, pinned, onTogglePin, onClose, children }: PanelSurfaceProps) {
  return (
    <div className="sp-panel-surface">
      <div className="sp-panel-surface-header">
        <span className="sp-panel-surface-title">{title}</span>
        <div className="sp-panel-surface-actions">
          <button className="sp-panel-surface-btn" title={pinned ? "Auto-hide" : "Pin"} onClick={onTogglePin}>
            {pinned ? "\u{1F4CC}" : "\u{1F4CD}"}
          </button>
          <button className="sp-panel-surface-btn" title="Close" onClick={onClose}>
            {"✕"}
          </button>
        </div>
      </div>
      <div className="sp-panel-surface-body">{children}</div>
    </div>
  );
}
