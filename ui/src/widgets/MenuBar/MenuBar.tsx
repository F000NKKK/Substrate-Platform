import type { ReactNode } from "react";
import "./MenuBar.css";

export interface MenuBarProps {
  /** Product name/branding shown at the far left. */
  title?: string;
  /** Menu entries — typically a row of <MenuBarItem>. */
  children?: ReactNode;
  /** Right-aligned slot — e.g. a <PalettePicker>. */
  actions?: ReactNode;
  /** Furthest-right slot — typically a <WindowControls> for a `decorations: false` window. */
  windowControls?: ReactNode;
}

/**
 * The platform's top menu bar chrome — a product only supplies its own items
 * and actions. Doubles as the window's drag region (`data-tauri-drag-region`)
 * so a `decorations: false` Tauri window stays draggable from its empty
 * space; `<MenuBarItem>`/`actions`/`windowControls` are ordinary elements and
 * remain clickable since the drag only engages on the bar itself.
 */
export function MenuBar({ title, children, actions, windowControls }: MenuBarProps) {
  return (
    <div className="sp-menubar" data-tauri-drag-region>
      {title && (
        <span className="sp-menubar-title" data-tauri-drag-region>
          {title}
        </span>
      )}
      <div className="sp-menubar-items">{children}</div>
      <div className="sp-menubar-spacer" data-tauri-drag-region />
      {actions && <div className="sp-menubar-actions">{actions}</div>}
      {windowControls && <div className="sp-menubar-windowcontrols">{windowControls}</div>}
    </div>
  );
}
