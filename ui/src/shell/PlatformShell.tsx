import type { CSSProperties, DragEvent } from "react";
import { useState } from "react";
import { ToolWindowDock } from "../dock/ToolWindowDock/ToolWindowDock";
import { CenterDock } from "../dock/CenterDock/CenterDock";
import { FloatingPanel } from "../dock/FloatingPanel/FloatingPanel";
import { useShellLayout } from "../dock/hooks/useShellLayout";
import { readPanelDrag, zoneFromPoint, type DropZone } from "../dock/dnd";
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
 */
export function PlatformShell({
  main,
  toolWindows,
  defaultPinned,
  persistKey,
  extraCenterPanels,
  onCloseDynamicPanel,
  menu,
}: PlatformShellProps) {
  const layout = useShellLayout(main, toolWindows, defaultPinned, persistKey, extraCenterPanels, onCloseDynamicPanel);
  const [dropZone, setDropZone] = useState<DropZone | null>(null);

  const bottomInsetStyle: CSSProperties = {
    marginLeft: toolWindows.left ? "calc(var(--sp-toolwindow-strip) + var(--sp-space-xs))" : undefined,
    marginRight: toolWindows.right ? "calc(var(--sp-toolwindow-strip) + var(--sp-space-xs))" : undefined,
  };

  function handleMainDragOver(e: DragEvent) {
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
    setDropZone(zoneFromPoint(e.clientX, e.clientY, e.currentTarget.getBoundingClientRect()));
  }

  function handleMainDragLeave() {
    setDropZone(null);
  }

  function handleMainDrop(e: DragEvent) {
    e.preventDefault();
    const id = readPanelDrag(e);
    setDropZone(null);
    if (!id) return;
    const zone = zoneFromPoint(e.clientX, e.clientY, e.currentTarget.getBoundingClientRect());
    layout.dockTo(id, zone === "center" ? "center" : zone);
  }

  return (
    <div className="sp-shell">
      {menu && <div className="sp-shell-menu">{menu}</div>}
      <div className="sp-shell-body">
        <div className="sp-shell-upper">
          {toolWindows.left && <ToolWindowDock anchor="left" layout={layout} />}

          <div
            className="sp-shell-main"
            onDragOver={handleMainDragOver}
            onDragLeave={handleMainDragLeave}
            onDrop={handleMainDrop}
          >
            <CenterDock layout={layout} />
            {dropZone && <div className={`sp-dock-guide sp-dock-guide--${dropZone}`} />}
          </div>

          {toolWindows.right && <ToolWindowDock anchor="right" layout={layout} />}
        </div>

        {toolWindows.bottom && (
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
            />
          );
        })}
      </div>
    </div>
  );
}
