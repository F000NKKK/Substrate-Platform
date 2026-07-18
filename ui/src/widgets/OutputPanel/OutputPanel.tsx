import { useEffect, useReducer } from "react";
import { getOutputBuffer, subscribeOutput, MAX_LINES_DEFAULT } from "./outputBuffer";
import "./OutputPanel.css";

export interface OutputPanelProps {
  /** Tauri event name streaming new log lines; omit for a static/empty panel driven only by `lines`. */
  eventName?: string;
  lines?: string[];
  emptyLabel?: string;
  /** Oldest lines beyond this are dropped — the buffer lives outside this component (see `outputBuffer`), so it keeps accumulating across the panel being hidden/reopened, not just while mounted. Default 10,000. */
  maxLines?: number;
}

/**
 * Generic append-only log viewer — a product feeds it lines directly via
 * `lines`, or via a Tauri event through `eventName`. For the event-driven
 * case, the accumulated buffer is owned by `outputBuffer`, not this
 * component's own state: a tool-window panel only stays *mounted* while
 * it's pinned or the active flyout, so anything kept in local state here
 * would be wiped every time the panel is hidden and reopened.
 */
export function OutputPanel({ eventName, lines: staticLines, emptyLabel = "No output yet.", maxLines = MAX_LINES_DEFAULT }: OutputPanelProps) {
  const [, forceUpdate] = useReducer((n: number) => n + 1, 0);

  useEffect(() => {
    if (!eventName) return;
    return subscribeOutput(eventName, maxLines, forceUpdate);
  }, [eventName, maxLines]);

  const lines = eventName ? getOutputBuffer(eventName) : (staticLines ?? []);

  return (
    <div className="sp-output-panel">
      {lines.length === 0 ? (
        <p className="sp-output-panel-empty">{emptyLabel}</p>
      ) : (
        lines.map((line, i) => (
          <div className="sp-output-panel-line" key={i}>
            {line}
          </div>
        ))
      )}
    </div>
  );
}
