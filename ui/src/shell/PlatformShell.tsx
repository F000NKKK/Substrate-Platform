import type { CSSProperties, DragEvent } from "react";
import { useMemo, useState } from "react";
import { ToolWindowDock } from "../dock/ToolWindowDock/ToolWindowDock";
import { CenterDock } from "../dock/CenterDock/CenterDock";
import { FloatingPanel } from "../dock/FloatingPanel/FloatingPanel";
import { useShellLayout } from "../dock/hooks/useShellLayout";
import { isPanelDrag, readPanelDrag, zoneFromPoint, type DropZone } from "../dock/dnd";
import { DialogHost } from "../widgets/Dialog";
import { NotificationHost } from "../widgets/Notifications";
import { ShellContext, type ShellController } from "./ShellContext";
import type { ToolWindowAnchor } from "../dock/types";
import type { PlatformShellProps } from "./types";
import "../dock/dock.css";
import "./shell.css";

/**
 * The whole product-agnostic IDE chrome: an optional menu bar, a tabbed
 * center dock, and up to three tool-window docks (left/right/bottom) that
 * are auto-hidden by default and can be pinned open, dragged to another
 * edge or into the center's tabs, or floated free — mirrors Visual
 * Studio's editor + tool windows rather than a flat grid of permanently
 * docked panels.
 *
 * The bottom dock is a full-width row below the left/main/right row (not
 * nested inside the center column), inset on each side by
 * `--sp-toolwindow-strip` wherever that side has a dock — the same physical
 * offset a pinned OR flyout panel's own outer edge sits at (see dock.css),
 * so the bottom dock's edges line up with the side panel's edges in either
 * state without a live measurement.
 *
 * Everything rendered inside it (menu, panels, status bar) can drive it via
 * `useShell()`, and it mounts the platform's one `DialogHost` and
 * `NotificationHost`, so `showDialog`/`notify` work anywhere in the app.
 */
export function PlatformShell({
  main,
  toolWindows,
  defaultPinned,
  persistKey,
  extraCenterPanels,
  onCloseDynamicPanel,
  menu,
  statusBar,
}: PlatformShellProps) {
  const layout = useShellLayout(main, toolWindows, defaultPinned, persistKey, extraCenterPanels, onCloseDynamicPanel);
  const [dropZone, setDropZone] = useState<DropZone | null>(null);

  const controller = useMemo<ShellController>(
    () => ({
      toolWindows: (["left", "right", "bottom"] as const).flatMap((anchor) =>
        (toolWindows[anchor] ?? []).map(({ id, title }) => ({ id, title }))
      ),
      isToolWindowVisible: layout.isVisible,
      showToolWindow: layout.show,
      hideToolWindow: layout.hide,
      isToolWindowFloating: (id) => layout.placements[id]?.anchor === "float",
      activeCenterId: layout.centerActiveId,
      activateCenterPanel: layout.setCenterActive,
    }),
    [toolWindows, layout.isVisible, layout.show, layout.hide, layout.placements, layout.centerActiveId, layout.setCenterActive]
  );

  // An edge gets a dock if the product declared tool windows for it, OR if
  // any panel has since been dragged (or restored from a persisted layout)
  // onto it — otherwise a panel dropped on an undeclared edge would have
  // nowhere to render and simply vanish.
  const hasDock = (anchor: ToolWindowAnchor) => !!toolWindows[anchor] || layout.idsByAnchor(anchor).length > 0;
  const hasLeft = hasDock("left");
  const hasRight = hasDock("right");
  const hasBottom = hasDock("bottom");

  const bottomInsetStyle: CSSProperties = {
    marginLeft: hasLeft ? "calc(var(--sp-toolwindow-strip) + var(--sp-space-xs))" : undefined,
    marginRight: hasRight ? "calc(var(--sp-toolwindow-strip) + var(--sp-space-xs))" : undefined,
  };

  function handleMainDragOver(e: DragEvent) {
    // Only panel drags redock; anything else (a toolbox item over a designer) is the content's business.
    if (!isPanelDrag(e)) return;
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
    setDropZone(zoneFromPoint(e.clientX, e.clientY, e.currentTarget.getBoundingClientRect()));
  }

  function handleMainDragLeave() {
    setDropZone(null);
  }

  function handleMainDrop(e: DragEvent) {
    const id = readPanelDrag(e);
    setDropZone(null);
    if (!id) return;
    e.preventDefault();
    const zone = zoneFromPoint(e.clientX, e.clientY, e.currentTarget.getBoundingClientRect());
    layout.dockTo(id, zone === "center" ? "center" : zone);
  }

  return (
    <ShellContext.Provider value={controller}>
      <div className="sp-shell">
        {menu && <div className="sp-shell-menu">{menu}</div>}
        <div className="sp-shell-body">
          <div className="sp-shell-upper">
            {hasLeft && <ToolWindowDock anchor="left" layout={layout} />}

            <div
              className="sp-shell-main"
              onDragOver={handleMainDragOver}
              onDragLeave={handleMainDragLeave}
              onDrop={handleMainDrop}
            >
              <CenterDock layout={layout} />
              {dropZone && <div className={`sp-dock-guide sp-dock-guide--${dropZone}`} />}
            </div>

            {hasRight && <ToolWindowDock anchor="right" layout={layout} />}
          </div>

          {hasBottom && (
            <div className="sp-shell-bottom-inset" style={bottomInsetStyle}>
              <ToolWindowDock anchor="bottom" layout={layout} />
            </div>
          )}

          {layout.floatingIds.map((id) => {
            const panel = layout.panelsById[id];
            const placement = layout.placements[id];
            if (!panel || placement.anchor !== "float") return null;
            return (
              <FloatingPanel
                key={id}
                panel={panel}
                x={placement.x}
                y={placement.y}
                w={placement.w}
                h={placement.h}
                onDock={() => layout.dockTo(id, layout.homeAnchor[id] ?? "bottom")}
                onClose={() => layout.close(id)}
                onResize={(w, h) => layout.resizeFloat(id, w, h)}
                onMove={(x, y) => layout.moveFloat(id, x, y)}
                onDockTo={(anchor) => layout.dockTo(id, anchor)}
              />
            );
          })}
        </div>
        {statusBar && <div className="sp-shell-status-bar">{statusBar}</div>}
        <DialogHost />
        <NotificationHost />
      </div>
    </ShellContext.Provider>
  );
}
