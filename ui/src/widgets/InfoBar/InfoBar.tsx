import type { ReactNode } from "react";
import "./InfoBar.css";

export interface InfoBarAction {
  label: ReactNode;
  onClick: () => void;
}

export interface InfoBarProps {
  /** What the bar says. */
  message: ReactNode;
  /** Link-style actions after the message (e.g. "Open the source file"). */
  actions?: InfoBarAction[];
  /** The content the bar sits above (e.g. a document's editor) — fills the rest of the frame. Without it, only the bar renders. */
  children?: ReactNode;
}

/**
 * A notice strip across the top of a document, like Visual Studio's info
 * bar ("This file is auto-generated…", "File was modified outside the
 * editor…"): a message plus optional link actions. With `children`, frames
 * that content below the bar so the two share the document area.
 */
export function InfoBar({ message, actions, children }: InfoBarProps) {
  const bar = (
    <div className="sp-infobar" role="note">
      <span className="sp-infobar-message">{message}</span>
      {actions?.map((action, i) => (
        <button key={i} type="button" className="sp-infobar-action" onClick={action.onClick}>
          {action.label}
        </button>
      ))}
    </div>
  );
  if (children === undefined) return bar;
  return (
    <div className="sp-infobar-frame">
      {bar}
      <div className="sp-infobar-content">{children}</div>
    </div>
  );
}
