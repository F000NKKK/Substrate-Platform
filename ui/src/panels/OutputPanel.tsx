import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import "./OutputPanel.css";

export interface OutputPanelProps {
  /** Tauri event name streaming new log lines; omit for a static/empty panel driven only by `lines`. */
  eventName?: string;
  lines?: string[];
  emptyLabel?: string;
}

/** Generic append-only log viewer — a product feeds it lines directly or via a Tauri event. */
export function OutputPanel({ eventName, lines: staticLines, emptyLabel = "No output yet." }: OutputPanelProps) {
  const [lines, setLines] = useState<string[]>(staticLines ?? []);

  useEffect(() => {
    if (!eventName) return;
    const unlistenPromise = listen<string>(eventName, (event) => {
      setLines((prev) => [...prev, event.payload]);
    });
    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, [eventName]);

  useEffect(() => {
    if (staticLines) setLines(staticLines);
  }, [staticLines]);

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
