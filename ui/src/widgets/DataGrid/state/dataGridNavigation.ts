import type { KeyboardEvent } from "react";
import { clamp } from "./dataGridRows";
import type { GridCommits } from "./dataGridCommits";
import type { GridProps, GridState } from "./useDataGridState";
import type { GridView } from "./useDataGridView";

/** Moving the active row and cell, scrolling to it, and selecting rows by click or keyboard. */
export function dataGridNavigation<T>({ getRowId, rowHeight, selectable }: GridProps<T>, state: GridState<T>, view: GridView<T>, commits: GridCommits) {
  const { activeColumnKey, setActiveIndex, setActiveColumnKey, pendingFocusRef, setFocusTick, scrollRef, viewportHeight, lastAnchorRef, activeSelection } = state;
  const { displayItems, visibleColumns, sortedRows, sortedIndexOf, indexOfRow } = view;

  function scrollIndexIntoView(index: number) {
    const el = scrollRef.current;
    if (!el) return;
    const top = index * rowHeight;
    const bottom = top + rowHeight;
    if (top < el.scrollTop) el.scrollTop = top;
    else if (bottom > el.scrollTop + viewportHeight) el.scrollTop = bottom - viewportHeight;
  }

  /** Makes item `index` (clamped) the active one, `columnKey` the active cell, scrolled to and focused. */
  function moveTo(index: number, columnKey: string | null = activeColumnKey) {
    if (displayItems.length === 0) return;
    const next = clamp(index, 0, displayItems.length - 1);
    setActiveIndex(next);
    if (columnKey) setActiveColumnKey(columnKey);
    scrollIndexIntoView(next);
    pendingFocusRef.current = true;
    setFocusTick((n) => n + 1);
  }

  /** Makes item `index` (clamped) the active one and scrolls to it, without focusing it. */
  function activate(index: number) {
    const next = clamp(index, 0, displayItems.length - 1);
    setActiveIndex(next);
    scrollIndexIntoView(next);
  }

  /** Tab: the next (`dir` 1) or previous editable-or-not visible cell, wrapping onto the next/previous row. */
  function moveAcross(index: number, columnKey: string | null, dir: 1 | -1) {
    const keys = visibleColumns.map((c) => c.key);
    if (keys.length === 0) return;
    const col = (columnKey ? keys.indexOf(columnKey) : -1) + dir;
    const wraps = col >= keys.length ? 1 : col < 0 ? -1 : 0;
    const row = index + wraps;
    if (row < 0 || row >= displayItems.length) return moveTo(index, columnKey);
    moveTo(row, wraps > 0 ? keys[0] : wraps < 0 ? keys[keys.length - 1] : keys[col]);
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
    commits.commitSelection(next);
  }

  /** Keyboard selection follows the active row: plain arrows select it alone, Shift extends from the anchor, Ctrl just moves. */
  function selectByKeyboard(index: number, e: KeyboardEvent) {
    if (!selectable || e.ctrlKey || e.metaKey) return;
    const item = displayItems[clamp(index, 0, displayItems.length - 1)];
    if (item?.kind !== "row") return;
    const rowIndex = sortedIndexOf(item.row);
    if (rowIndex < 0) return;
    if (e.shiftKey && lastAnchorRef.current === null) lastAnchorRef.current = rowIndex;
    selectRow(rowIndex, { range: e.shiftKey });
  }

  /** Focuses (and scrolls to) the cell of row `rowId` in `columnKey` — for a host's "Add" button, after a paste, ... */
  function focusCell(rowId: string, columnKey?: string) {
    const index = indexOfRow(rowId);
    if (index >= 0) moveTo(index, columnKey ?? activeColumnKey);
  }

  return { moveTo, activate, moveAcross, selectRow, selectByKeyboard, focusCell };
}

export type GridNavigation = ReturnType<typeof dataGridNavigation>;
