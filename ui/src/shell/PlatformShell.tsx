import type { DragEvent } from "react";
import { useRef, useState } from "react";
import { ToolWindowDock } from "./ToolWindowDock";
import { FloatingPanel } from "./FloatingPanel";
import { useShellLayout } from "./useShellLayout";
import { readPanelDrag, zoneFromPoint, type DropZone } from "./dnd";
import type { PlatformShellProps } from "./types";
import "./shell.css";

/**
 * The whole product-agnostic IDE chrome: an optional menu bar, one fixed
 * main content area, and up to three tool-window docks (left/right/bottom)
 * that are auto-hidden by default and can be pinned open, dragged to
 * another edge, or pulled free into a floating panel — mirrors Visual
 * Studio's tool windows rather than a flat grid of permanently docked
 * panels.
 */
export function PlatformShell({ main, toolWindows, defaultPinned, menu }: PlatformShellProps) {
  const MainComponent = main.component;
  const layout = useShellLayout(toolWindows, defaultPinned);
  const bodyRef = useRef<HTMLDivElement>(null);
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
    const body = bodyRef.current;
    setDropZone(null);
    if (!id || !body) return;
    const rect = e.currentTarget.getBoundingClientRect();
    const zone = zoneFromPoint(e.clientX, e.clientY, rect);
    if (zone === "center") {
      const bodyRect = body.getBoundingClientRect();
      layout.floatAt(id, e.clientX - bodyRect.left, e.clientY - bodyRect.top);
    } else {
      layout.dockTo(id, zone);
    }
  }

  return (
    <div className="sp-shell">
      {menu && <div className="sp-shell-menu">{menu}</div>}
      <div className="sp-shell-body" ref={bodyRef}>
        {toolWindows.left && <ToolWindowDock anchor="left" layout={layout} />}

        <div className="sp-shell-center">
          <div
            className="sp-shell-main"
            onDragOver={handleMainDragOver}
            onDragLeave={handleMainDragLeave}
            onDrop={handleMainDrop}
          >
            <MainComponent />
            {dropZone && <div className={`sp-dock-guide sp-dock-guide--${dropZone}`} />}
          </div>
          {toolWindows.bottom && <ToolWindowDock anchor="bottom" layout={layout} />}
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
