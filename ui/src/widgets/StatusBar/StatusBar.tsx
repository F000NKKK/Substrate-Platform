import { useState, type MouseEvent, type ReactNode } from "react";
import { Icon } from "../../infra/icons";
import "./StatusBar.css";

export interface StatusBarProps {
  /** The current stage, e.g. "Ready" / "Building…" / "Debugging: stopped at breakpoint" — entirely the host's own vocabulary, this component has no idea what stages exist. */
  text: string;
  /** Shows a small spinner next to `text` — for an in-progress stage. */
  busy?: boolean;
  /** Recent detail lines, shown in a popover when the bar is clicked (most recent last) — e.g. build output, status events. Omit for a bar with nothing to expand into. */
  detail?: string[];
  /** More items after the stage text, at the bar's right end — typically `StatusBarButton`s (Visual Studio's branch and pending-changes indicators). */
  trailing?: ReactNode;
}

/**
 * A slim, click-to-expand status bar pinned to the shell's bottom edge —
 * Visual Studio's bottom-right build/run indicator. Purely presentational:
 * the host supplies the current stage text and whatever detail lines are
 * worth showing when expanded; this component has no idea what a "stage"
 * or a "detail line" means for any particular product.
 */
export function StatusBar({ text, busy, detail, trailing }: StatusBarProps) {
  const [open, setOpen] = useState(false);
  const hasDetail = !!detail && detail.length > 0;

  return (
    <div className="sp-status-bar">
      <button type="button" className="sp-status-bar-item" onClick={() => hasDetail && setOpen((o) => !o)} disabled={!hasDetail}>
        {busy && <span className="sp-status-bar-spinner" aria-hidden />}
        <span>{text}</span>
      </button>
      {trailing}
      {open && hasDetail && (
        <>
          <div className="sp-status-bar-scrim" onClick={() => setOpen(false)} />
          <div className="sp-status-bar-popover">
            {detail!.map((line, i) => (
              <div key={i} className="sp-status-bar-line">
                {line}
              </div>
            ))}
          </div>
        </>
      )}
    </div>
  );
}

export interface StatusBarButtonProps {
  /** An `Icon` name, drawn before the label. */
  icon?: string;
  label?: ReactNode;
  title?: string;
  onClick?: (e: MouseEvent<HTMLButtonElement>) => void;
}

/** One clickable indicator in `StatusBar`'s `trailing` slot — an icon and/or a short label. */
export function StatusBarButton({ icon, label, title, onClick }: StatusBarButtonProps) {
  return (
    <button type="button" className="sp-status-bar-item sp-status-bar-button" title={title} aria-label={title} onClick={onClick} disabled={!onClick}>
      {icon && <Icon name={icon} size={12} />}
      {label !== undefined && <span>{label}</span>}
    </button>
  );
}
