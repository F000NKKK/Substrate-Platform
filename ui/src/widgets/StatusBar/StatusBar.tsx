import { useState } from "react";
import "./StatusBar.css";

export interface StatusBarProps {
  /** The current stage, e.g. "Ready" / "Building…" / "Debugging: stopped at breakpoint" — entirely the host's own vocabulary, this component has no idea what stages exist. */
  text: string;
  /** Shows a small spinner next to `text` — for an in-progress stage. */
  busy?: boolean;
  /** Recent detail lines, shown in a popover when the bar is clicked (most recent last) — e.g. build output, status events. Omit for a bar with nothing to expand into. */
  detail?: string[];
}

/**
 * A slim, click-to-expand status bar pinned to the shell's bottom edge —
 * Visual Studio's bottom-right build/run indicator. Purely presentational:
 * the host supplies the current stage text and whatever detail lines are
 * worth showing when expanded; this component has no idea what a "stage"
 * or a "detail line" means for any particular product.
 */
export function StatusBar({ text, busy, detail }: StatusBarProps) {
  const [open, setOpen] = useState(false);
  const hasDetail = !!detail && detail.length > 0;

  return (
    <div className="sp-status-bar">
      <button type="button" className="sp-status-bar-item" onClick={() => hasDetail && setOpen((o) => !o)} disabled={!hasDetail}>
        {busy && <span className="sp-status-bar-spinner" aria-hidden />}
        <span>{text}</span>
      </button>
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
