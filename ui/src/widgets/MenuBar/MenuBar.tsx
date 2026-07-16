import type { ReactNode } from "react";
import "./MenuBar.css";

export interface MenuBarProps {
  /** Product name/branding shown at the far left. */
  title?: string;
  /** Menu entries — typically a row of <MenuBarItem>. */
  children?: ReactNode;
  /** Right-aligned slot — e.g. a <PalettePicker>. */
  actions?: ReactNode;
}

/** The platform's top menu bar chrome — a product only supplies its own items and actions. */
export function MenuBar({ title, children, actions }: MenuBarProps) {
  return (
    <div className="sp-menubar">
      {title && <span className="sp-menubar-title">{title}</span>}
      <div className="sp-menubar-items">{children}</div>
      <div className="sp-menubar-spacer" />
      {actions && <div className="sp-menubar-actions">{actions}</div>}
    </div>
  );
}
