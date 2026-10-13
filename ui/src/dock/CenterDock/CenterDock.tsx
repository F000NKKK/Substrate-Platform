import { Tab } from "../../widgets/Tab";
import { IconButton } from "../../widgets/IconButton";
import { Icon } from "../../infra/icons";
import { ContextMenu, useContextMenu } from "../../widgets/ContextMenu";
import { startPanelDrag, endPanelDrag } from "../dnd";
import { useDropTarget } from "../hooks/useDropTarget";
import type { ShellLayout } from "../hooks/useShellLayout";

export interface CenterDockProps {
  layout: ShellLayout;
}

/**
 * The main content zone — a normal tabbed dock, same as any IDE's document
 * area: it owns its own horizontal tab bar on top and can hold more than
 * just the product's primary panel, since anything can be dragged in to
 * join it as another tab.
 */
export function CenterDock({ layout }: CenterDockProps) {
  const { dragOver, handlers } = useDropTarget((id) => layout.dockTo(id, "center"));
  const menu = useContextMenu<string>();

  // Pinned tabs lead the strip, each group keeping its own order.
  const shownIds = layout.centerIds.filter((id) => {
    const panel = layout.panelsById[id];
    return panel && !panel.hidden;
  });
  const isPinned = (id: string) => !!layout.panelsById[id]?.pinned;
  const orderedIds = [...shownIds.filter(isPinned), ...shownIds.filter((id) => !isPinned(id))];
  const menuPanel = menu.target ? layout.panelsById[menu.target.node] : undefined;
  const actions = (layout.centerActiveId && layout.panelsById[layout.centerActiveId]?.actions) || [];

  return (
    <div className="sp-center-dock">
      <div className="sp-center-tabs" data-drag-over={dragOver} {...handlers}>
        {orderedIds.map((id) => {
          const panel = layout.panelsById[id];
          const closable = id !== layout.mainId;
          return (
            <Tab
              key={id}
              orientation="horizontal"
              active={id === layout.centerActiveId}
              title={panel.tooltip}
              draggable
              onDragStart={(e) => startPanelDrag(e, id)}
              onDragEnd={endPanelDrag}
              onClick={() => layout.setCenterActive(id)}
              // Middle-click closes, as in every tabbed editor.
              onAuxClick={(e) => {
                if (e.button === 1 && closable) layout.close(id);
              }}
              onContextMenu={(e) => {
                if (panel.contextMenu) menu.openAtPoint(id, e);
              }}
              pinned={panel.pinned}
              onRequestUnpin={panel.onUnpin}
              onRequestClose={closable ? () => layout.close(id) : undefined}
            >
              {panel.title}
            </Tab>
          );
        })}
        {actions.length > 0 && (
          <div className="sp-center-actions">
            {actions.map((action) => (
              <IconButton key={action.id} title={action.title} aria-label={action.title} aria-pressed={action.active} onClick={action.onClick}>
                <Icon name={action.icon} size={16} />
              </IconButton>
            ))}
          </div>
        )}
      </div>
      {menuPanel?.contextMenu && (
        <ContextMenu
          target={menu.target && menu.target.mode === "viewport" ? { mode: "viewport", x: menu.target.x, y: menu.target.y } : null}
          items={menuPanel.contextMenu({ centerIds: orderedIds })}
          onClose={menu.close}
        />
      )}
      <div className="sp-center-body" {...handlers}>
        {layout.centerIds.map((id) => {
          const panel = layout.panelsById[id];
          if (!panel) return null;
          const Component = panel.component;
          return (
            <div key={id} className="sp-center-view" data-active={id === layout.centerActiveId || undefined}>
              <Component />
            </div>
          );
        })}
      </div>
    </div>
  );
}
