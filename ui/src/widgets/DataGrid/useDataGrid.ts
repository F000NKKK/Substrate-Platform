import {
  useCallback,
  useMemo,
  useRef,
  useState,
  type DragEvent as ReactDragEvent,
  type KeyboardEvent,
  type MouseEvent as ReactMouseEvent,
  type PointerEvent as ReactPointerEvent,
} from "react";
import type { DataGridColumn, DataGridProps, DataGridSortState } from "./DataGrid";
import { useContextMenu } from "../ContextMenu";

export type DataGridMenuTarget<T> = { kind: "grid" } | { kind: "header"; column: DataGridColumn<T> };

type DisplayItem<T> =
  | { kind: "group"; key: string; level: number; columnKey: string; label: string; count: number; aggregates: Record<string, number>; rows: T[] }
  | { kind: "row"; row: T };

const DEFAULT_ROW_HEIGHT = 28;
const DEFAULT_COLUMN_WIDTH = 140;
const DEFAULT_MIN_WIDTH = 48;
const SELECT_COLUMN_WIDTH = 32;
const OVERSCAN = 6;
const COLUMN_DRAG_MIME = "application/x-sp-datagrid-column";

function defaultCompare<T>(column: DataGridColumn<T>, a: T, b: T): number {
  const av = column.sortValue ? column.sortValue(a) : "";
  const bv = column.sortValue ? column.sortValue(b) : "";
  if (typeof av === "number" && typeof bv === "number") return av - bv;
  return String(av).localeCompare(String(bv), undefined, { numeric: true, sensitivity: "base" });
}

function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

function nextSortState(current: DataGridSortState[], key: string, additive: boolean): DataGridSortState[] {
  const index = current.findIndex((s) => s.key === key);
  if (!additive) {
    if (index === -1) return [{ key, direction: "asc" }];
    return current[index].direction === "asc" ? [{ key, direction: "desc" }] : [];
  }
  if (index === -1) return [...current, { key, direction: "asc" }];
  if (current[index].direction === "asc") {
    const next = [...current];
    next[index] = { key, direction: "desc" };
    return next;
  }
  return current.filter((s) => s.key !== key);
}

function formatAggregate<T>(column: DataGridColumn<T>, value: number): string {
  if (column.formatSummary) return column.formatSummary(value);
  if (column.summary === "avg") return value.toFixed(1);
  return value.toLocaleString();
}

function computeAggregates<T>(columns: DataGridColumn<T>[], rows: T[]): Record<string, number> {
  const aggregates: Record<string, number> = {};
  for (const column of columns) {
    if (!column.summary || !column.summaryValue) continue;
    if (column.summary === "count") {
      aggregates[column.key] = rows.length;
      continue;
    }
    const sum = rows.reduce((acc, row) => acc + column.summaryValue!(row), 0);
    aggregates[column.key] = column.summary === "avg" ? sum / Math.max(1, rows.length) : sum;
  }
  return aggregates;
}

function groupValueOf<T>(column: DataGridColumn<T>, row: T): string {
  if (column.groupValue) return column.groupValue(row);
  if (column.filterValue) return column.filterValue(row);
  if (column.sortValue) return String(column.sortValue(row));
  return "";
}

function buildDisplayItems<T>(
  rows: T[],
  groupBy: string[],
  columnByKey: Map<string, DataGridColumn<T>>,
  collapsedGroups: Set<string>,
  level: number,
  path: string
): DisplayItem<T>[] {
  if (level >= groupBy.length) return rows.map((row) => ({ kind: "row", row }));

  const column = columnByKey.get(groupBy[level]);
  if (!column) return buildDisplayItems(rows, groupBy, columnByKey, collapsedGroups, level + 1, path);

  const buckets = new Map<string, T[]>();
  for (const row of rows) {
    const value = groupValueOf(column, row);
    const bucket = buckets.get(value);
    if (bucket) bucket.push(row);
    else buckets.set(value, [row]);
  }

  const orderedKeys = [...buckets.keys()].sort((a, b) => a.localeCompare(b, undefined, { numeric: true, sensitivity: "base" }));
  const items: DisplayItem<T>[] = [];
  for (const value of orderedKeys) {
    const bucketRows = buckets.get(value)!;
    const groupKey = `${path}/${column.key}:${value}`;
    items.push({
      kind: "group",
      key: groupKey,
      level,
      columnKey: column.key,
      label: value || "(blank)",
      count: bucketRows.length,
      aggregates: computeAggregates([...columnByKey.values()], bucketRows),
      rows: bucketRows,
    });
    if (!collapsedGroups.has(groupKey)) {
      items.push(...buildDisplayItems(bucketRows, groupBy, columnByKey, collapsedGroups, level + 1, groupKey));
    }
  }
  return items;
}

/**
 * All of DataGrid's state and behavior — multi-column sort, drag-to-group
 * with aggregates, per-column filtering, column resize/reorder, selection,
 * and virtualization — with no rendering. `DataGrid.tsx` is the view over
 * this; `formatAggregate` is exported for the view's group/footer cells.
 */
export function useDataGrid<T>({
  columns,
  rows,
  getRowId,
  rowHeight = DEFAULT_ROW_HEIGHT,
  selectable = false,
  selectedIds,
  defaultSelectedIds,
  onSelectionChange,
  sort,
  defaultSort = [],
  onSortChange,
  groupBy,
  defaultGroupBy = [],
  onGroupByChange,
  hiddenColumns,
  defaultHiddenColumns = new Set(),
  onHiddenColumnsChange,
  onRowActivate,
  onCellEditCommit,
}: DataGridProps<T>) {
  const [internalSelection, setInternalSelection] = useState<Set<string>>(defaultSelectedIds ?? new Set());
  const [internalSort, setInternalSort] = useState<DataGridSortState[]>(defaultSort);
  const [internalGroupBy, setInternalGroupBy] = useState<string[]>(defaultGroupBy);
  const [internalHiddenColumns, setInternalHiddenColumns] = useState<Set<string>>(defaultHiddenColumns);
  const [columnOrder, setColumnOrder] = useState<string[]>(() => columns.map((c) => c.key));
  const [widths, setWidths] = useState<Record<string, number>>(() =>
    Object.fromEntries(columns.map((c) => [c.key, c.width ?? DEFAULT_COLUMN_WIDTH]))
  );
  const [filters, setFilters] = useState<Record<string, string>>({});
  const [collapsedGroups, setCollapsedGroups] = useState<Set<string>>(new Set());
  const [scrollTop, setScrollTop] = useState(0);
  const [viewportHeight, setViewportHeight] = useState(480);
  const [activeIndex, setActiveIndex] = useState(0);
  const [activeColumnKey, setActiveColumnKey] = useState<string | null>(null);
  const [editingCell, setEditingCell] = useState<{ rowId: string; columnKey: string } | null>(null);
  const [flash, setFlash] = useState(false);
  const [dragColumnKey, setDragColumnKey] = useState<string | null>(null);
  const [dropIndicator, setDropIndicator] = useState<{ key: string; side: "before" | "after" } | null>(null);
  const [groupPanelHover, setGroupPanelHover] = useState(false);
  const [groupPanelVisible, setGroupPanelVisible] = useState(true);
  const gridMenu = useContextMenu<DataGridMenuTarget<T>>();

  const scrollRef = useRef<HTMLDivElement>(null);
  const lastAnchorRef = useRef<number | null>(null);
  const resizeRef = useRef<{ key: string; startX: number; startWidth: number } | null>(null);

  const activeSort = sort ?? internalSort;
  const activeGroupBy = groupBy ?? internalGroupBy;
  const activeSelection = selectedIds ?? internalSelection;
  const activeHiddenColumns = hiddenColumns ?? internalHiddenColumns;

  const columnByKey = useMemo(() => new Map(columns.map((c) => [c.key, c])), [columns]);

  const orderedColumns = useMemo(
    () => columnOrder.map((key) => columnByKey.get(key)).filter((c): c is DataGridColumn<T> => !!c),
    [columnOrder, columnByKey]
  );
  const visibleColumns = useMemo(
    () => orderedColumns.filter((c) => !activeGroupBy.includes(c.key) && !activeHiddenColumns.has(c.key)),
    [orderedColumns, activeGroupBy, activeHiddenColumns]
  );
  const summaryColumns = useMemo(() => orderedColumns.filter((c) => c.summary && c.summaryValue), [orderedColumns]);

  const filteredRows = useMemo(() => {
    const activeFilters = Object.entries(filters).filter(([, v]) => v.trim() !== "");
    if (activeFilters.length === 0) return rows;
    return rows.filter((row) =>
      activeFilters.every(([key, needle]) => {
        const column = columnByKey.get(key);
        if (!column) return true;
        const text = column.filterValue ? column.filterValue(row) : column.sortValue ? String(column.sortValue(row)) : "";
        return text.toLowerCase().includes(needle.toLowerCase());
      })
    );
  }, [rows, filters, columnByKey]);

  const sortedRows = useMemo(() => {
    if (activeSort.length === 0) return filteredRows;
    return [...filteredRows].sort((a, b) => {
      for (const s of activeSort) {
        const column = columnByKey.get(s.key);
        if (!column) continue;
        const cmp = defaultCompare(column, a, b) * (s.direction === "asc" ? 1 : -1);
        if (cmp !== 0) return cmp;
      }
      return 0;
    });
  }, [filteredRows, activeSort, columnByKey]);

  const displayItems = useMemo(
    () => buildDisplayItems(sortedRows, activeGroupBy, columnByKey, collapsedGroups, 0, ""),
    [sortedRows, activeGroupBy, columnByKey, collapsedGroups]
  );

  const footerAggregates = useMemo(() => computeAggregates(orderedColumns, sortedRows), [orderedColumns, sortedRows]);

  const gridTemplate = useMemo(() => {
    const cols = visibleColumns.map((c) => `${widths[c.key] ?? DEFAULT_COLUMN_WIDTH}px`);
    return selectable ? [`${SELECT_COLUMN_WIDTH}px`, ...cols].join(" ") : cols.join(" ");
  }, [visibleColumns, widths, selectable]);

  function commitSelection(next: Set<string>) {
    if (selectedIds === undefined) setInternalSelection(next);
    onSelectionChange?.(next);
  }

  function commitSort(next: DataGridSortState[]) {
    if (sort === undefined) setInternalSort(next);
    onSortChange?.(next);
    setFlash(true);
    window.setTimeout(() => setFlash(false), 160);
  }

  function commitGroupBy(next: string[]) {
    if (groupBy === undefined) setInternalGroupBy(next);
    onGroupByChange?.(next);
  }

  function commitHiddenColumns(next: Set<string>) {
    if (hiddenColumns === undefined) setInternalHiddenColumns(next);
    onHiddenColumnsChange?.(next);
  }

  function toggleColumnHidden(key: string) {
    const column = columnByKey.get(key);
    if (column?.hideable === false) return;
    const next = new Set(activeHiddenColumns);
    if (next.has(key)) next.delete(key);
    else next.add(key);
    commitHiddenColumns(next);
  }

  function resetToDefaults() {
    setColumnOrder(columns.map((c) => c.key));
    setWidths(Object.fromEntries(columns.map((c) => [c.key, c.width ?? DEFAULT_COLUMN_WIDTH])));
    commitHiddenColumns(new Set());
    commitSort(defaultSort);
    commitGroupBy(defaultGroupBy);
    setCollapsedGroups(new Set());
    setGroupPanelVisible(true);
  }

  function toggleGroupPanelVisible() {
    setGroupPanelVisible((v) => !v);
  }

  function startEdit(rowId: string, columnKey: string) {
    const column = columnByKey.get(columnKey);
    if (!column?.editable) return;
    setEditingCell({ rowId, columnKey });
  }

  function commitEdit(row: T, columnKey: string, value: string) {
    const column = columnByKey.get(columnKey);
    column?.onCellEdit?.(row, columnKey, value);
    onCellEditCommit?.(row, columnKey, value);
    setEditingCell(null);
  }

  function cancelEdit() {
    setEditingCell(null);
  }

  function toggleGroupCollapsed(key: string) {
    setCollapsedGroups((prev) => {
      const next = new Set(prev);
      if (next.has(key)) next.delete(key);
      else next.add(key);
      return next;
    });
  }

  function handleHeaderClick(e: ReactMouseEvent, column: DataGridColumn<T>) {
    if (!column.sortable || column.lockSort || dragColumnKey) return;
    commitSort(nextSortState(activeSort, column.key, e.metaKey || e.ctrlKey));
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

  function handleHeaderDragStart(e: ReactDragEvent, column: DataGridColumn<T>) {
    e.dataTransfer.setData(COLUMN_DRAG_MIME, column.key);
    e.dataTransfer.effectAllowed = "move";
    setDragColumnKey(column.key);
  }

  function handleHeaderDragOver(e: ReactDragEvent, column: DataGridColumn<T>) {
    if (!e.dataTransfer.types.includes(COLUMN_DRAG_MIME)) return;
    e.preventDefault();
    const rect = e.currentTarget.getBoundingClientRect();
    const side = e.clientX - rect.left < rect.width / 2 ? "before" : "after";
    setDropIndicator({ key: column.key, side });
  }

  function handleHeaderDrop(e: ReactDragEvent, column: DataGridColumn<T>) {
    e.preventDefault();
    const draggedKey = e.dataTransfer.getData(COLUMN_DRAG_MIME);
    if (draggedKey && draggedKey !== column.key) {
      setColumnOrder((order) => {
        const without = order.filter((k) => k !== draggedKey);
        let index = without.indexOf(column.key);
        if (dropIndicator?.side === "after") index += 1;
        without.splice(index, 0, draggedKey);
        return without;
      });
    }
    setDropIndicator(null);
    setDragColumnKey(null);
  }

  function handleHeaderDragEnd() {
    setDragColumnKey(null);
    setDropIndicator(null);
  }

  function handleGroupPanelDragOver(e: ReactDragEvent) {
    if (!e.dataTransfer.types.includes(COLUMN_DRAG_MIME)) return;
    e.preventDefault();
    setGroupPanelHover(true);
  }

  function handleGroupPanelDrop(e: ReactDragEvent) {
    e.preventDefault();
    const key = e.dataTransfer.getData(COLUMN_DRAG_MIME);
    const column = key ? columnByKey.get(key) : undefined;
    if (key && column && !column.lockGroup && !activeGroupBy.includes(key)) commitGroupBy([...activeGroupBy, key]);
    setGroupPanelHover(false);
    setDragColumnKey(null);
  }

  function selectRow(rowIndexInSorted: number, opts: { toggle?: boolean; range?: boolean }) {
    const id = getRowId(sortedRows[rowIndexInSorted]);
    let next: Set<string>;
    if (opts.range && lastAnchorRef.current !== null) {
      const [lo, hi] = [lastAnchorRef.current, rowIndexInSorted].sort((a, b) => a - b);
      next = new Set(activeSelection);
      for (let i = lo; i <= hi; i++) next.add(getRowId(sortedRows[i]));
    } else if (opts.toggle) {
      next = new Set(activeSelection);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      lastAnchorRef.current = rowIndexInSorted;
    } else {
      next = new Set([id]);
      lastAnchorRef.current = rowIndexInSorted;
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

  function handleItemKeyDown(e: KeyboardEvent, index: number) {
    const item = displayItems[index];
    switch (e.key) {
      case "ArrowDown": {
        e.preventDefault();
        const next = clamp(index + 1, 0, displayItems.length - 1);
        setActiveIndex(next);
        scrollIndexIntoView(next);
        break;
      }
      case "ArrowUp": {
        e.preventDefault();
        const next = clamp(index - 1, 0, displayItems.length - 1);
        setActiveIndex(next);
        scrollIndexIntoView(next);
        break;
      }
      case "ArrowRight":
        if (item.kind === "group" && collapsedGroups.has(item.key)) {
          e.preventDefault();
          toggleGroupCollapsed(item.key);
        }
        break;
      case "ArrowLeft":
        if (item.kind === "group" && !collapsedGroups.has(item.key)) {
          e.preventDefault();
          toggleGroupCollapsed(item.key);
        }
        break;
      case "Home":
        e.preventDefault();
        setActiveIndex(0);
        scrollIndexIntoView(0);
        break;
      case "End": {
        e.preventDefault();
        const last = displayItems.length - 1;
        setActiveIndex(last);
        scrollIndexIntoView(last);
        break;
      }
      case " ":
        e.preventDefault();
        if (item.kind === "group") toggleGroupCollapsed(item.key);
        else if (selectable) selectRow(sortedRows.indexOf(item.row), { toggle: true });
        break;
      case "Enter": {
        if (item.kind === "group") {
          toggleGroupCollapsed(item.key);
          break;
        }
        const column = activeColumnKey ? columnByKey.get(activeColumnKey) : undefined;
        if (column?.editable) startEdit(getRowId(item.row), column.key);
        else onRowActivate?.(item.row);
        break;
      }
    }
  }

  const attachScrollRef = useCallback((el: HTMLDivElement | null) => {
    scrollRef.current = el;
    if (el) setViewportHeight(el.clientHeight);
  }, []);

  const totalHeight = displayItems.length * rowHeight;
  const startIndex = clamp(Math.floor(scrollTop / rowHeight) - OVERSCAN, 0, Math.max(0, displayItems.length - 1));
  const visibleCount = Math.ceil(viewportHeight / rowHeight) + OVERSCAN * 2;
  const endIndex = clamp(startIndex + visibleCount, 0, displayItems.length);
  const visibleItems = displayItems.slice(startIndex, endIndex);

  const allSelected = selectable && sortedRows.length > 0 && sortedRows.every((r) => activeSelection.has(getRowId(r)));

  return {
    columnByKey,
    orderedColumns,
    visibleColumns,
    summaryColumns,
    gridTemplate,
    activeSort,
    activeGroupBy,
    activeSelection,
    dragColumnKey,
    dropIndicator,
    groupPanelHover,
    setGroupPanelHover,
    groupPanelVisible,
    toggleGroupPanelVisible,
    filters,
    setFilters,
    sortedRows,
    displayItems,
    visibleItems,
    startIndex,
    totalHeight,
    collapsedGroups,
    activeIndex,
    setActiveIndex,
    flash,
    footerAggregates,
    allSelected,
    commitSelection,
    commitSort,
    commitGroupBy,
    handleHeaderClick,
    handleResizeStart,
    handleHeaderDragStart,
    handleHeaderDragOver,
    handleHeaderDrop,
    handleHeaderDragEnd,
    handleGroupPanelDragOver,
    handleGroupPanelDrop,
    toggleGroupCollapsed,
    selectRow,
    handleItemKeyDown,
    attachScrollRef,
    setScrollTop,
    activeHiddenColumns,
    toggleColumnHidden,
    resetToDefaults,
    editingCell,
    startEdit,
    commitEdit,
    cancelEdit,
    activeColumnKey,
    setActiveColumnKey,
    gridMenu,
  };
}

export { formatAggregate };
