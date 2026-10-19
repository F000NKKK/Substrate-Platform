import { useCallback, useMemo } from "react";
import type { DataGridColumn } from "./DataGrid";
import { buildDisplayItems, clamp, computeAggregates, DEFAULT_COLUMN_WIDTH, type DisplayItem, filterRows, OVERSCAN, SELECT_COLUMN_WIDTH, sortRows } from "./dataGridRows";
import type { GridProps, GridState } from "./useDataGridState";

/** What the state shows: the columns in order and visible, the rows filtered, sorted and grouped, and the window of them rendered. */
export function useDataGridView<T>({ columns, rows, getRowId, pinnedRows, rowHeight, selectable, showSelectColumn }: GridProps<T>, state: GridState<T>) {
  const { columnOrder, widths, filters, collapsedGroups, activeSort, activeGroupBy, activeHiddenColumns, activeSelection, activeColumnKey, scrollTop, viewportHeight } = state;
  const columnByKey = useMemo(() => new Map(columns.map((c) => [c.key, c])), [columns]);
  const orderedColumns = useMemo(() => columnOrder.map((key) => columnByKey.get(key)).filter((c): c is DataGridColumn<T> => !!c), [columnOrder, columnByKey]);
  const visibleColumns = useMemo(() => orderedColumns.filter((c) => !activeGroupBy.includes(c.key) && !activeHiddenColumns.has(c.key)), [orderedColumns, activeGroupBy, activeHiddenColumns]);
  const summaryColumns = useMemo(() => orderedColumns.filter((c) => c.summary && c.summaryValue), [orderedColumns]);
  const filteredRows = useMemo(() => filterRows(rows, filters, columnByKey), [rows, filters, columnByKey]);
  const sortedRows = useMemo(() => sortRows(filteredRows, activeSort, columnByKey), [filteredRows, activeSort, columnByKey]);
  const sortedIndex = useMemo(() => new Map(sortedRows.map((row, i) => [row, i])), [sortedRows]);
  /** `row`'s index in `sortedRows` (-1 for a pinned row) — a lookup, not a scan per rendered row. */
  const sortedIndexOf = useCallback((row: T) => sortedIndex.get(row) ?? -1, [sortedIndex]);

  const displayItems = useMemo(() => {
    const items = buildDisplayItems(sortedRows, activeGroupBy, columnByKey, collapsedGroups);
    // Pinned rows (a "new row" placeholder, say) stay last — not sorted, filtered or grouped.
    if (pinnedRows?.length) items.push(...pinnedRows.map((row): DisplayItem<T> => ({ kind: "row", row })));
    return items;
  }, [sortedRows, activeGroupBy, columnByKey, collapsedGroups, pinnedRows]);

  const footerAggregates = useMemo(() => computeAggregates(orderedColumns, sortedRows), [orderedColumns, sortedRows]);
  const gridTemplate = useMemo(() => {
    const cols = visibleColumns.map((c) => `${widths[c.key] ?? DEFAULT_COLUMN_WIDTH}px`);
    return selectable && showSelectColumn ? [`${SELECT_COLUMN_WIDTH}px`, ...cols].join(" ") : cols.join(" ");
  }, [visibleColumns, widths, selectable, showSelectColumn]);

  // The active cell's column under `cellNavigation` — the first visible one until a cell is clicked or moved to.
  const activeCellKey = activeColumnKey && visibleColumns.some((c) => c.key === activeColumnKey) ? activeColumnKey : (visibleColumns[0]?.key ?? null);

  const totalHeight = displayItems.length * rowHeight;
  const startIndex = clamp(Math.floor(scrollTop / rowHeight) - OVERSCAN, 0, Math.max(0, displayItems.length - 1));
  const visibleCount = Math.ceil(viewportHeight / rowHeight) + OVERSCAN * 2;
  const endIndex = clamp(startIndex + visibleCount, 0, displayItems.length);
  const visibleItems = displayItems.slice(startIndex, endIndex);
  const allSelected = selectable && sortedRows.length > 0 && sortedRows.every((r) => activeSelection.has(getRowId(r)));

  /** Item `rowId`'s index among the shown items (-1 if it isn't shown). */
  const indexOfRow = (rowId: string) => displayItems.findIndex((item) => item.kind === "row" && getRowId(item.row) === rowId);

  return { columnByKey, orderedColumns, visibleColumns, summaryColumns, sortedRows, sortedIndexOf, displayItems, footerAggregates, gridTemplate, activeCellKey, totalHeight, startIndex, visibleItems, allSelected, indexOfRow };
}

export type GridView<T> = ReturnType<typeof useDataGridView<T>>;
