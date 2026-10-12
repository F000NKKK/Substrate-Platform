import type { ReactNode } from "react";
import "./HighlightedText.css";

export interface HighlightedTextProps {
  text: string;
  /** Spans to highlight, `[start, end)` offsets into `text` (UTF-16, i.e. JS string indices). */
  ranges?: readonly (readonly [number, number])[];
  /** Single highlighted characters (a fuzzy match's positions) — merged into runs. */
  positions?: readonly number[];
  className?: string;
}

/** Contiguous runs from sorted character positions: `[0, 1, 2, 5]` → `[[0, 3], [5, 6]]`. */
export function positionsToRanges(positions: readonly number[]): [number, number][] {
  const sorted = [...positions].sort((a, b) => a - b);
  const runs: [number, number][] = [];
  for (const p of sorted) {
    const last = runs[runs.length - 1];
    if (last && p <= last[1]) last[1] = Math.max(last[1], p + 1);
    else runs.push([p, p + 1]);
  }
  return runs;
}

/**
 * Text with some spans marked — search hits in a result line, the letters a
 * fuzzy query matched in a name. Overlapping or out-of-range spans are
 * tolerated.
 */
export function HighlightedText({ text, ranges, positions, className }: HighlightedTextProps) {
  const spans = [...(ranges ?? []), ...(positions ? positionsToRanges(positions) : [])]
    .map(([s, e]) => [Math.max(0, Math.min(s, text.length)), Math.max(0, Math.min(e, text.length))] as const)
    .filter(([s, e]) => e > s)
    .sort((a, b) => a[0] - b[0]);
  const parts: ReactNode[] = [];
  let at = 0;
  for (const [s, e] of spans) {
    if (e <= at) continue;
    const start = Math.max(s, at);
    if (start > at) parts.push(text.slice(at, start));
    parts.push(
      <mark key={start} className="sp-hl-match">
        {text.slice(start, e)}
      </mark>,
    );
    at = e;
  }
  if (at < text.length) parts.push(text.slice(at));
  return <span className={["sp-hl", className].filter(Boolean).join(" ")}>{parts}</span>;
}
