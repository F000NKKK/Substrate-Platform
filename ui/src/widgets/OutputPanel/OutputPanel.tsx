import { memo, useEffect, useLayoutEffect, useReducer, useRef } from "react";
import { getOutputBuffer, getOutputBufferStart, subscribeOutput, MAX_LINES_DEFAULT } from "./outputBuffer";
import { useT } from "../../infra/i18n";
import "./OutputPanel.css";

export type OutputLineLevel = "info" | "warn" | "error";

/** Recognizes a leading `[warn]`/`[warning]`/`[error]` tag (case-insensitive) — the plain-string convention a host uses when flattening a leveled log line into one event payload. Anything else is `info`. */
export function defaultLineLevel(line: string): OutputLineLevel {
  const tag = line.match(/^\s*\[(warn|warning|error)\]/i)?.[1].toLowerCase();
  if (tag === "error") return "error";
  if (tag) return "warn";
  return "info";
}

export interface OutputPanelProps {
  /** Tauri event name streaming new log lines; omit for a static/empty panel driven only by `lines`. */
  eventName?: string;
  lines?: string[];
  emptyLabel?: string;
  /** Oldest lines beyond this are dropped — the buffer lives outside this component (see `outputBuffer`), so it keeps accumulating across the panel being hidden/reopened, not just while mounted. Default 10,000. */
  maxLines?: number;
  /** Severity per line, for coloring — default `defaultLineLevel`. */
  lineLevel?: (line: string) => OutputLineLevel;
}

/** How close (px) to the bottom still counts as "following" the output. */
const STICK_THRESHOLD = 24;

/**
 * Generic append-only log viewer — a product feeds it lines directly via
 * `lines`, or via a Tauri event through `eventName`. For the event-driven
 * case, the accumulated buffer is owned by `outputBuffer`, not this
 * component's own state: a tool-window panel only stays *mounted* while
 * it's pinned or the active flyout, so anything kept in local state here
 * would be wiped every time the panel is hidden and reopened (clear it with
 * `clearOutputBuffer`). Follows new output while scrolled to the bottom;
 * scrolling up stops following until the user scrolls back down.
 */
export function OutputPanel({
  eventName,
  lines: staticLines,
  emptyLabel,
  maxLines = MAX_LINES_DEFAULT,
  lineLevel = defaultLineLevel,
}: OutputPanelProps) {
  const [, forceUpdate] = useReducer((n: number) => n + 1, 0);
  const t = useT();
  const containerRef = useRef<HTMLDivElement>(null);
  const stickToBottom = useRef(true);

  useEffect(() => {
    if (!eventName) return;
    // A build streams hundreds of lines per frame, each its own event —
    // render once per frame, not once per line.
    let frame = 0;
    const unsubscribe = subscribeOutput(eventName, maxLines, () => {
      frame ||= requestAnimationFrame(() => {
        frame = 0;
        forceUpdate();
      });
    });
    return () => {
      cancelAnimationFrame(frame);
      unsubscribe();
    };
  }, [eventName, maxLines]);

  const lines = eventName ? getOutputBuffer(eventName) : (staticLines ?? []);
  // Keys follow the line, not its position, so trimming the oldest line at
  // the cap doesn't re-render every row below it.
  const start = eventName ? getOutputBufferStart(eventName) : 0;

  useLayoutEffect(() => {
    const el = containerRef.current;
    if (el && stickToBottom.current) el.scrollTop = el.scrollHeight;
  });

  function handleScroll() {
    const el = containerRef.current;
    if (el) stickToBottom.current = el.scrollHeight - el.scrollTop - el.clientHeight <= STICK_THRESHOLD;
  }

  return (
    <div className="sp-output-panel" ref={containerRef} onScroll={handleScroll}>
      {lines.length === 0 ? (
        <p className="sp-output-panel-empty">{emptyLabel ?? t("No output yet.")}</p>
      ) : (
        lines.map((line, i) => <OutputLine key={start + i} line={line} lineLevel={lineLevel} />)
      )}
    </div>
  );
}

const OutputLine = memo(function OutputLine({ line, lineLevel }: { line: string; lineLevel: (line: string) => OutputLineLevel }) {
  return <div className={`sp-output-panel-line sp-output-panel-line--${lineLevel(line)}`}>{line}</div>;
});
