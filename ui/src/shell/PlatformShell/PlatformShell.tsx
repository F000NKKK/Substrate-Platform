import type { CSSProperties, DragEvent } from "react";
import { useLayoutEffect, useRef, useState } from "react";
import { ToolWindowDock } from "../ToolWindowDock/ToolWindowDock";
import { CenterDock } from "../CenterDock/CenterDock";
import { FloatingPanel } from "../FloatingPanel/FloatingPanel";
import { useShellLayout } from "../useShellLayout";
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
 */
export function PlatformShell({ main, toolWindows, defaultPinned, menu }: PlatformShellProps) {
  const layout = useShellLayout(main, toolWindows, defaultPinned);
  const [dropZone, setDropZone] = useState<DropZone | null>(null);

  // The bottom dock drives two independent measurements of the space it takes,
  // both measured live so every pinned/flyout combination is exact:
  //   • bottomOccupied — the WHOLE dock height (panels + strip). A left/right
  //     *flyout* overlays the main area, so it must stop above all of this.
  //   • bottomBelow — the space BELOW the last pinned bottom panel (the strip
  //     + the gap under it). Pinned side panels are capped by this so their
  //     bottom edge lines up flush with the bottom panel's bottom edge, rather
  //     than running past it down to the window's very bottom.
  const bottomSlotRef = useRef<HTMLDivElement>(null);
  const [bottomOccupied, setBottomOccupied] = useState(0);
  const [bottomBelow, setBottomBelow] = useState(0);
  useLayoutEffect(() => {
    const el = bottomSlotRef.current;
    if (!el) {
      setBottomOccupied(0);
      setBottomBelow(0);
      return;
    }
    const measure = () => {
      const slot = el.getBoundingClientRect();
      setBottomOccupied(slot.height);
      const panels = el.querySelectorAll<HTMLElement>(".sp-dock--bottom > .sp-panel-surface");
      const last = panels[panels.length - 1];
      setBottomBelow(last ? Math.max(0, slot.bottom - last.getBoundingClientRect().bottom) : slot.height);
    };
    measure();
    const ro = new ResizeObserver(measure);
    ro.observe(el);
    return () => ro.disconnect();
  }, [layout.placements, toolWindows.bottom]);
  const shellBodyStyle = {
    "--sp-bottom-occupied": `${bottomOccupied}px`,
    "--sp-bottom-below": `${bottomBelow}px`,
  } as CSSProperties;

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
