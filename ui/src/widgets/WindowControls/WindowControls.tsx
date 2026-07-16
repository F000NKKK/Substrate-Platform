import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Icon } from "../../infra/icons";
import "./WindowControls.css";

const appWindow = getCurrentWindow();

/**
 * Custom minimize/maximize/close buttons for a `decorations: false` Tauri
 * window — the platform never relies on the OS's own title-bar chrome
 * (GNOME/Windows/etc. all draw it differently), so every product using this
 * shell gets identical window controls. Pair with `data-tauri-drag-region`
 * on the bar this renders inside (e.g. `MenuBar`) so the window is still
 * draggable and double-click-to-maximize still works.
 */
export function WindowControls() {
  const [maximized, setMaximized] = useState(false);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    appWindow.isMaximized().then(setMaximized);
    appWindow.onResized(() => {
      appWindow.isMaximized().then(setMaximized);
    }).then((fn) => {
      unlisten = fn;
    });
    return () => unlisten?.();
  }, []);

  return (
    <div className="sp-windowcontrols">
      <button
        type="button"
        className="sp-windowcontrols-btn"
        aria-label="Minimize"
        onClick={() => appWindow.minimize()}
      >
        <Icon name="minimize" size={14} />
      </button>
      <button
        type="button"
        className="sp-windowcontrols-btn"
        aria-label={maximized ? "Restore" : "Maximize"}
        onClick={() => appWindow.toggleMaximize()}
      >
        <Icon name={maximized ? "restore" : "maximize"} size={14} />
      </button>
      <button
        type="button"
        className="sp-windowcontrols-btn sp-windowcontrols-btn--close"
        aria-label="Close"
        onClick={() => appWindow.close()}
      >
        <Icon name="close" size={14} />
      </button>
    </div>
  );
}
