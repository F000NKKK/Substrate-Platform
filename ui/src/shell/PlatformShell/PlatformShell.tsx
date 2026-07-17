import type { DragEvent } from "react";
import { useState } from "react";
import { ToolWindowDock } from "../ToolWindowDock/ToolWindowDock";
import { CenterDock } from "../CenterDock/CenterDock";
import { FloatingPanel } from "../FloatingPanel/FloatingPanel";
import { useShellLayout } from "../hooks/useShellLayout";
import { readPanelDrag, zoneFromPoint, type DropZone } from "../dnd";
import type { PlatformShellProps } from "../types";
import "../shell.css";

/**
 * The whole product-agnostic IDE chrome: an optional menu bar, a tabbed
 * center dock, and up to three tool-window docks (left/right/bottom) that
 * are auto-hidden by default and can be pinned open, dragged to another
 * edge or into the center's tabs, or floated free — mirrors Visual
 * Studio's editor + tool windows rather than a flat grid of permanently
 * docked panels.
 *
 * The bottom dock is a full-width row below the left/main/right row (not
 * nested inside the center column) so its left edge always lines up with
 * the left dock's outer edge, exactly like the left/right docks' own edges
 * line up with the window — no separate "how tall is the bottom dock"
 * measurement is needed since the two rows no longer overlap.
 */
export function PlatformShell({ main, toolWindows, defaultPinned, menu }: PlatformShellProps) {
  const layout = useShellLayout(main, toolWindows, defaultPinned);
  const [dropZone, setDropZone] = useState<DropZone | null>(null);

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
      <div className="sp-shell-body" style={shellBodyStyle}>
        {toolWindows.left && <ToolWindowDock anchor="left" layout={layout} />}

        <div className="sp-shell-center">
          <div
            className="sp-shell-main"
            onDragOver={handleMainDragOver}
            onDragLeave={handleMainDragLeave}
            onDrop={handleMainDrop}
          >
            <CenterDock layout={layout} />
            {dropZone && <div className={`sp-dock-guide sp-dock-guide--${dropZone}`} />}
          </div>
          {toolWindows.bottom && (
            <div className="sp-bottom-slot" ref={bottomSlotRef}>
              <ToolWindowDock anchor="bottom" layout={layout} />
            </div>
          )}
        </div>

        {toolWindows.right && <ToolWindowDock anchor="right" layout={layout} />}

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
