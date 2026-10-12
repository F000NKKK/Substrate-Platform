import { useEffect, useLayoutEffect, useMemo, useRef, useState, type KeyboardEvent, type ReactNode } from "react";
import { HighlightedText } from "../HighlightedText";
import { IconChevronRight } from "../../infra/icons";
import "./SearchResults.css";

export interface SearchResultHit {
  key: string;
  /** Before the text, dimmed: `lib.rs(12,5):`. */
  location: string;
  text: string;
  /** Match spans within `text`, `[start, end)`. */
  ranges?: readonly (readonly [number, number])[];
}

export interface SearchResultGroup<H extends SearchResultHit = SearchResultHit> {
  key: string;
  /** The file's path. */
  label: string;
  /** Dimmed after the label — e.g. the hit count. */
  detail?: string;
  icon?: ReactNode;
  hits: readonly H[];
}

export interface SearchResultsProps<H extends SearchResultHit> {
  groups: readonly SearchResultGroup<H>[];
  /** Double-click or Enter on a hit (or on a group header, with `hit` null). */
  onActivate: (group: SearchResultGroup<H>, hit: H | null) => void;
  /** Shown when there are no groups. */
  emptyText?: ReactNode;
  /** A line after the last group — Visual Studio's "Matching lines: ..." summary. */
  summary?: ReactNode;
}

const ROW_HEIGHT = 22;
const OVERSCAN = 12;

type Row = { kind: "group"; group: number } | { kind: "hit"; group: number; hit: number } | { kind: "summary" };

/**
 * Find Results: hits grouped under their file, collapsible, each one
 * `file(line,col): text` with the match highlighted. Virtualized, so tens of
 * thousands of hits stay smooth. Up/Down select, Left/Right collapse and
 * expand, Enter or a double-click opens.
 */
export function SearchResults<H extends SearchResultHit>({ groups, onActivate, emptyText, summary }: SearchResultsProps<H>) {
  const [collapsed, setCollapsed] = useState<ReadonlySet<string>>(new Set());
  const [selected, setSelected] = useState<{ group: string; hit: number } | null>(null);
  const [scrollTop, setScrollTop] = useState(0);
  const [height, setHeight] = useState(400);
  const ref = useRef<HTMLDivElement>(null);

  const rows = useMemo(() => {
    const out: Row[] = [];
    groups.forEach((g, gi) => {
      out.push({ kind: "group", group: gi });
      if (!collapsed.has(g.key)) g.hits.forEach((_, hi) => out.push({ kind: "hit", group: gi, hit: hi }));
    });
    if (summary) out.push({ kind: "summary" });
    return out;
  }, [groups, collapsed, summary]);

  const selectedRow = selected
    ? rows.findIndex((r) => r.kind !== "summary" && groups[r.group].key === selected.group && (r.kind === "hit" ? r.hit : -1) === selected.hit)
    : -1;

  useEffect(() => {
    const el = ref.current;
    if (!el) return;
    const observer = new ResizeObserver(() => setHeight(el.clientHeight));
    observer.observe(el);
    setHeight(el.clientHeight);
    return () => observer.disconnect();
  }, []);

  // A new search starts expanded, at the top.
  const firstKey = groups[0]?.key;
  useEffect(() => {
    if (groups.length === 0) {
      setCollapsed(new Set());
      setSelected(null);
      ref.current?.scrollTo({ top: 0 });
    }
  }, [groups.length, firstKey]);

  useLayoutEffect(() => {
    const el = ref.current;
    if (!el || selectedRow < 0) return;
    const top = selectedRow * ROW_HEIGHT;
    if (top < el.scrollTop) el.scrollTop = top;
    else if (top + ROW_HEIGHT > el.scrollTop + el.clientHeight) el.scrollTop = top + ROW_HEIGHT - el.clientHeight;
  }, [selectedRow]);

  function select(row: Row | undefined) {
    if (!row || row.kind === "summary") return;
    setSelected({ group: groups[row.group].key, hit: row.kind === "hit" ? row.hit : -1 });
  }

  function toggle(key: string, open?: boolean) {
    setCollapsed((prev) => {
      const next = new Set(prev);
      const isOpen = !prev.has(key);
      if (open ?? !isOpen) next.delete(key);
      else next.add(key);
      return next;
    });
  }

  function activate(row: Row | undefined) {
    if (!row || row.kind === "summary") return;
    const group = groups[row.group];
    onActivate(group, row.kind === "hit" ? group.hits[row.hit] : null);
  }

  function onKeyDown(e: KeyboardEvent) {
    const row = rows[selectedRow];
    const page = Math.max(1, Math.floor(height / ROW_HEIGHT) - 1);
    const go = (i: number) => select(rows[Math.max(0, Math.min(rows.length - 1, i))]);
    let handled = true;
    if (e.key === "ArrowDown") go(selectedRow + 1 < rows.length && rows[selectedRow + 1].kind === "summary" ? selectedRow : selectedRow + 1);
    else if (e.key === "ArrowUp") go(Math.max(0, selectedRow - 1));
    else if (e.key === "PageDown") go(Math.min(selectedRow + page, rows.length - (summary ? 2 : 1)));
    else if (e.key === "PageUp") go(selectedRow - page);
    else if (e.key === "Home") go(0);
    else if (e.key === "End") go(rows.length - (summary ? 2 : 1));
    else if (e.key === "Enter") activate(row);
    else if (e.key === "ArrowLeft" && row && row.kind !== "summary") {
      const key = groups[row.group].key;
      if (row.kind === "hit") setSelected({ group: key, hit: -1 });
      else toggle(key, false);
    } else if (e.key === "ArrowRight" && row?.kind === "group") toggle(groups[row.group].key, true);
    else handled = false;
    if (handled) e.preventDefault();
  }

  if (groups.length === 0 && !summary) {
    return <div className="sp-search-results sp-search-results--empty">{emptyText}</div>;
  }

  const first = Math.max(0, Math.floor(scrollTop / ROW_HEIGHT) - OVERSCAN);
  const last = Math.min(rows.length, Math.ceil((scrollTop + height) / ROW_HEIGHT) + OVERSCAN);

  return (
    <div
      ref={ref}
      className="sp-search-results"
      tabIndex={0}
      role="tree"
      onKeyDown={onKeyDown}
      onScroll={(e) => setScrollTop(e.currentTarget.scrollTop)}
    >
      <div className="sp-search-results-canvas" style={{ height: rows.length * ROW_HEIGHT }}>
        {rows.slice(first, last).map((row, i) => {
          const at = first + i;
          const style = { top: at * ROW_HEIGHT, height: ROW_HEIGHT };
          if (row.kind === "summary") {
            return (
              <div key="summary" className="sp-search-results-summary" style={style}>
                {summary}
              </div>
            );
          }
          const group = groups[row.group];
          const isSelected = at === selectedRow;
          if (row.kind === "group") {
            const open = !collapsed.has(group.key);
            return (
              <div
                key={`g:${group.key}`}
                role="treeitem"
                aria-expanded={open}
                aria-selected={isSelected}
                className="sp-search-results-row sp-search-results-group"
                data-selected={isSelected || undefined}
                style={style}
                onClick={() => select(row)}
                onDoubleClick={() => activate(row)}
              >
                <span
                  className="sp-search-results-twisty"
                  data-open={open || undefined}
                  onClick={(e) => {
                    e.stopPropagation();
                    toggle(group.key);
                  }}
                >
                  <IconChevronRight size={12} />
                </span>
                {group.icon}
                <span className="sp-search-results-label">{group.label}</span>
                {group.detail && <span className="sp-search-results-detail">{group.detail}</span>}
              </div>
            );
          }
          const hit = group.hits[row.hit];
          return (
            <div
              key={`h:${group.key}:${hit.key}`}
              role="treeitem"
              aria-selected={isSelected}
              className="sp-search-results-row sp-search-results-hit"
              data-selected={isSelected || undefined}
              style={style}
              onClick={() => select(row)}
              onDoubleClick={() => activate(row)}
            >
              <span className="sp-search-results-location">{hit.location}</span>
              <HighlightedText className="sp-search-results-text" text={hit.text} ranges={hit.ranges} />
            </div>
          );
        })}
      </div>
    </div>
  );
}
