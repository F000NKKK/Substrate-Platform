import { ToolWindowDock } from "./ToolWindowDock";
import type { PlatformShellProps } from "./types";
import "./shell.css";

/**
 * The whole product-agnostic IDE chrome: an optional menu bar, one fixed
 * main content area, and up to three tool-window docks (left/right/bottom)
 * that are auto-hidden by default and can be pinned open — mirrors Visual
 * Studio's distinction between the editor surface and its tool windows,
 * rather than a flat grid of permanently docked panels.
 */
export function PlatformShell({ main, toolWindows, defaultPinned, menu }: PlatformShellProps) {
  const MainComponent = main.component;

  return (
    <div className="sp-shell">
      {menu && <div className="sp-shell-menu">{menu}</div>}
      <div className="sp-shell-body">
        {toolWindows.left && (
          <ToolWindowDock anchor="left" panels={toolWindows.left} defaultPinnedId={defaultPinned?.left} />
        )}

        <div className="sp-shell-center">
          <div className="sp-shell-main">
            <MainComponent />
          </div>
          {toolWindows.bottom && (
            <ToolWindowDock anchor="bottom" panels={toolWindows.bottom} defaultPinnedId={defaultPinned?.bottom} />
          )}
        </div>

        {toolWindows.right && (
          <ToolWindowDock anchor="right" panels={toolWindows.right} defaultPinnedId={defaultPinned?.right} />
        )}
      </div>
    </div>
  );
}
