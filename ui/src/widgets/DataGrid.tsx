import {
  useCallback,
  useMemo,
  useRef,
  useState,
  type CSSProperties,
  type KeyboardEvent,
  type PointerEvent as ReactPointerEvent,
  type ReactNode,
} from "react";
import "./DataGrid.css";

export type SortDirection = "asc" | "desc";

export interface DataGridSortState {
  key: string;
  direction: SortDirection;
}

export interface DataGridColumn<T> {
  key: string;
  header: ReactNode;
  /** Initial pixel width; the user can resize from here. */
  width?: number;
  minWidth?: number;
  align?: "left" | "right" | "center";
  sortable?: boolean;
  /** Value the default comparator sorts on; omit for columns rendered from more than one field. */
  sortValue?: (row: T) => string | number;
  render?: (row: T) => ReactNode;
}

export interface DataGridProps<T> {
  columns: DataGridColumn<T>[];
  rows: T[];
  getRowId: (row: T) => string;
  /** Row height in px — also the virtualization unit, so keep it fixed. Default 28. */
  rowHeight?: number;
  selectable?: boolean;
  selectedIds?: Set<string>;
  defaultSelectedIds?: Set<string>;
  onSelectionChange?: (ids: Set<string>) => void;
  sort?: DataGridSortState | null;
  defaultSort?: DataGridSortState | null;
  onSortChange?: (sort: DataGridSortState | null) => void;
  onRowActivate?: (row: T) => void;
  emptyState?: ReactNode;
  className?: string;
  style?: CSSProperties;
}

const DEFAULT_ROW_HEIGHT = 28;
const DEFAULT_COLUMN_WIDTH = 140;
const DEFAULT_MIN_WIDTH = 48;
const SELECT_COLUMN_WIDTH = 32;
const OVERSCAN = 6;

function defaultCompare<T>(column: DataGridColumn<T>, a: T, b: T): number {
  const av = column.sortValue ? column.sortValue(a) : "";
  const bv = column.sortValue ? column.sortValue(b) : "";
  if (typeof av === "number" && typeof bv === "number") return av - bv;
  return String(av).localeCompare(String(bv), undefined, { numeric: true, sensitivity: "base" });
}

function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

/**
 * A dense, virtualized data table — sortable/resizable columns, roving-tabindex
 * keyboard navigation, and single/range/multi row selection. Rows above ~50
 * items should always go through this rather than a plain mapped `<div>` list:
 * the windowing is what keeps a few-thousand-row grid smooth.
 */
export function DataGrid<T>({
  columns,
  rows,
  getRowId,
  rowHeight = DEFAULT_ROW_HEIGHT,
  selectable = false,
  selectedIds,
  defaultSelectedIds,
  onSelectionChange,
  sort,
  defaultSort = null,
  onSortChange,
  onRowActivate,
  emptyState,
  className,
  style,
}: DataGridProps<T>) {
  const [internalSelection, setInternalSelection] = useState<Set<string>>(defaultSelectedIds ?? new Set());
  const [internalSort, setInternalSort] = useState<DataGridSortState | null>(defaultSort);
  const [widths, setWidths] = useState<Record<string, number>>(() =>
    Object.fromEntries(columns.map((c) => [c.key, c.width ?? DEFAULT_COLUMN_WIDTH]))
  );
  const [scrollTop, setScrollTop] = useState(0);
  const [viewportHeight, setViewportHeight] = useState(480);
  const [activeIndex, setActiveIndex] = useState(0);
  const [flash, setFlash] = useState(false);

  const scrollRef = useRef<HTMLDivElement>(null);
  const lastAnchorRef = useRef<number | null>(null);
  const resizeRef = useRef<{ key: string; startX: number; startWidth: number } | null>(null);

  const activeSort = sort !== undefined ? sort : internalSort;
  const activeSelection = selectedIds ?? internalSelection;

  const sortedRows = useMemo(() => {
    if (!activeSort) return rows;
    const column = columns.find((c) => c.key === activeSort.key);
    if (!column) return rows;
    const dir = activeSort.direction === "asc" ? 1 : -1;
    return [...rows].sort((a, b) => dir * defaultCompare(column, a, b));
  }, [rows, activeSort, columns]);

  const gridTemplate = useMemo(() => {
    const cols = columns.map((c) => `${widths[c.key] ?? DEFAULT_COLUMN_WIDTH}px`);
    return selectable ? [`${SELECT_COLUMN_WIDTH}px`, ...cols].join(" ") : cols.join(" ");
  }, [columns, widths, selectable]);

  function commitSelection(next: Set<string>) {
    if (selectedIds === undefined) setInternalSelection(next);
    onSelectionChange?.(next);
  }

  function commitSort(next: DataGridSortState | null) {
    if (sort === undefined) setInternalSort(next);
    onSortChange?.(next);
    setFlash(true);
    window.setTimeout(() => setFlash(false), 160);
  }

  function handleHeaderClick(column: DataGridColumn<T>) {
    if (!column.sortable || resizeRef.current) return;
    if (activeSort?.key !== column.key) {
      commitSort({ key: column.key, direction: "asc" });
    } else if (activeSort.direction === "asc") {
      commitSort({ key: column.key, direction: "desc" });
    } else {
      commitSort(null);
    }
  }

  function handleResizeStart(e: ReactPointerEvent, column: DataGridColumn<T>) {
    e.stopPropagation();
    e.preventDefault();
    const startWidth = widths[column.key] ?? DEFAULT_COLUMN_WIDTH;
    resizeRef.current = { key: column.key, startX: e.clientX, startWidth };
    const minWidth = column.minWidth ?? DEFAULT_MIN_WIDTH;

    function onMove(ev: PointerEvent) {
      if (!resizeRef.current) return;
      const delta = ev.clientX - resizeRef.current.startX;
      const next = clamp(resizeRef.current.startWidth + delta, minWidth, 2000);
      setWidths((w) => ({ ...w, [resizeRef.current!.key]: next }));
    }
    function onUp() {
      resizeRef.current = null;
      window.removeEventListener("pointermove", onMove);
      window.removeEventListener("pointerup", onUp);
    }
    window.addEventListener("pointermove", onMove);
    window.addEventListener("pointerup", onUp);
  }

  function selectRow(index: number, opts: { toggle?: boolean; range?: boolean }) {
    const id = getRowId(sortedRows[index]);
    let next: Set<string>;
    if (opts.range && lastAnchorRef.current !== null) {
      const [lo, hi] = [lastAnchorRef.current, index].sort((a, b) => a - b);
      next = new Set(activeSelection);
      for (let i = lo; i <= hi; i++) next.add(getRowId(sortedRows[i]));
    } else if (opts.toggle) {
      next = new Set(activeSelection);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      lastAnchorRef.current = index;
    } else {
      next = new Set([id]);
      lastAnchorRef.current = index;
    }
    commitSelection(next);
  }

  function scrollIndexIntoView(index: number) {
    const el = scrollRef.current;
    if (!el) return;
    const top = index * rowHeight;
    const bottom = top + rowHeight;
    if (top < el.scrollTop) el.scrollTop = top;
    else if (bottom > el.scrollTop + viewportHeight) el.scrollTop = bottom - viewportHeight;
  }

  function handleRowKeyDown(e: KeyboardEvent, index: number) {
    switch (e.key) {
      case "ArrowDown": {
        e.preventDefault();
        const next = clamp(index + 1, 0, sortedRows.length - 1);
        setActiveIndex(next);
        scrollIndexIntoView(next);
        if (e.shiftKey && selectable) selectRow(next, { range: true });
        break;
      }
      case "ArrowUp": {
        e.preventDefault();
        const next = clamp(index - 1, 0, sortedRows.length - 1);
        setActiveIndex(next);
        scrollIndexIntoView(next);
        if (e.shiftKey && selectable) selectRow(next, { range: true });
        break;
      }
      case "Home":
        e.preventDefault();
        setActiveIndex(0);
        scrollIndexIntoView(0);
        break;
      case "End": {
        e.preventDefault();
        const last = sortedRows.length - 1;
        setActiveIndex(last);
        scrollIndexIntoView(last);
        break;
      }
      case " ":
        if (selectable) {
          e.preventDefault();
          selectRow(index, { toggle: true });
        }
        break;
      case "Enter":
        onRowActivate?.(sortedRows[index]);
        break;
    }
  }

  const attachScrollRef = useCallback((el: HTMLDivElement | null) => {
    scrollRef.current = el;
    if (el) setViewportHeight(el.clientHeight);
  }, []);

  const totalHeight = sortedRows.length * rowHeight;
  const startIndex = clamp(Math.floor(scrollTop / rowHeight) - OVERSCAN, 0, Math.max(0, sortedRows.length - 1));
  const visibleCount = Math.ceil(viewportHeight / rowHeight) + OVERSCAN * 2;
  const endIndex = clamp(startIndex + visibleCount, 0, sortedRows.length);
  const visibleRows = sortedRows.slice(startIndex, endIndex);

  const allSelected = selectable && sortedRows.length > 0 && sortedRows.every((r) => activeSelection.has(getRowId(r)));

  return (
    <div className={["sp-datagrid", className].filter(Boolean).join(" ")} style={style}>
      <div className="sp-datagrid-header" role="row" style={{ gridTemplateColumns: gridTemplate }}>
        {selectable && (
          <div className="sp-datagrid-headercell sp-datagrid-headercell--select">
            <input
              type="checkbox"
              checked={allSelected}
              onChange={() => commitSelection(allSelected ? new Set() : new Set(sortedRows.map(getRowId)))}
              aria-label="Select all rows"
            />
          </div>
        )}
        {columns.map((column) => {
          const isSorted = activeSort?.key === column.key;
          return (
            <div
              key={column.key}
              className="sp-datagrid-headercell"
              data-align={column.align ?? "left"}
              data-sortable={column.sortable || undefined}
              role="columnheader"
              aria-sort={isSorted ? (activeSort!.direction === "asc" ? "ascending" : "descending") : "none"}
              onClick={() => handleHeaderClick(column)}
            >
              <span className="sp-datagrid-headercell-label">{column.header}</span>
              {column.sortable && (
                <span className="sp-datagrid-sorticon" data-active={isSorted || undefined} data-dir={isSorted ? activeSort!.direction : "asc"}>
                  ▾
                </span>
              )}
              <span
                className="sp-datagrid-resize-handle"
                onPointerDown={(e) => handleResizeStart(e, column)}
                onClick={(e) => e.stopPropagation()}
              />
            </div>
          );
        })}
      </div>

      <div
        className="sp-datagrid-body"
        ref={attachScrollRef}
        role="rowgroup"
        data-flash={flash || undefined}
        onScroll={(e) => setScrollTop(e.currentTarget.scrollTop)}
      >
        {sortedRows.length === 0 ? (
          <div className="sp-datagrid-empty">{emptyState ?? "No rows"}</div>
        ) : (
          <div className="sp-datagrid-spacer" style={{ height: totalHeight }}>
            {visibleRows.map((row, i) => {
              const index = startIndex + i;
              const id = getRowId(row);
              const selected = activeSelection.has(id);
              return (
                <div
                  key={id}
                  className="sp-datagrid-row"
                  role="row"
                  data-selected={selected || undefined}
                  tabIndex={index === activeIndex ? 0 : -1}
                  style={{ top: index * rowHeight, height: rowHeight, gridTemplateColumns: gridTemplate }}
                  onFocus={() => setActiveIndex(index)}
                  onKeyDown={(e) => handleRowKeyDown(e, index)}
                  onDoubleClick={() => onRowActivate?.(row)}
                  onClick={(e) => {
                    setActiveIndex(index);
                    if (!selectable) return;
                    selectRow(index, { toggle: e.metaKey || e.ctrlKey, range: e.shiftKey });
                  }}
                >
                  {selectable && (
                    <div className="sp-datagrid-cell sp-datagrid-cell--select" onClick={(e) => e.stopPropagation()}>
                      <input
                        type="checkbox"
                        checked={selected}
                        onChange={() => selectRow(index, { toggle: true })}
                        aria-label={`Select row ${id}`}
                      />
                    </div>
                  )}
                  {columns.map((column) => (
                    <div key={column.key} className="sp-datagrid-cell" data-align={column.align ?? "left"}>
                      {column.render ? column.render(row) : (row as Record<string, ReactNode>)[column.key]}
                    </div>
                  ))}
                </div>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
}
