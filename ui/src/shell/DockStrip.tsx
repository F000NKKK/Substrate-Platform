import type { PanelDef, ToolWindowAnchor } from "./types";

export interface DockStripProps {
  anchor: ToolWindowAnchor;
  panels: PanelDef[];
  activeId: string | null;
  onSelect: (panelId: string) => void;
}

/** The always-visible strip of collapsed tool-window tabs along one edge of the shell. */
export function DockStrip({ anchor, panels, activeId, onSelect }: DockStripProps) {
  return (
    <div className={`sp-dock-strip sp-dock-strip--${anchor}`}>
      {panels.map((panel) => (
        <button
          key={panel.id}
          className="sp-dock-tab"
          data-active={panel.id === activeId}
          onClick={() => onSelect(panel.id)}
          title={panel.title}
        >
          <span className="sp-dock-tab-title">{panel.title}</span>
        </button>
      ))}
    </div>
  );
}
