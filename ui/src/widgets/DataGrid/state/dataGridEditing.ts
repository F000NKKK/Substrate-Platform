import type { GridNavigation } from "./dataGridNavigation";
import type { GridProps, GridState } from "./useDataGridState";
import type { GridView } from "./useDataGridView";

/** Where a committed edit moves on: down after Enter, across after Tab/Shift+Tab. */
export type EditMove = "down" | "next" | "prev";

/** Editing a cell in place: starting, committing (and moving on, as a spreadsheet does), cancelling. */
export function dataGridEditing<T>({ getRowId, onCellEditCommit, cellNavigation }: GridProps<T>, state: GridState<T>, view: GridView<T>, nav: GridNavigation) {
  const { editingRef, setEditingCell } = state;
  const { columnByKey, indexOfRow } = view;

  function startEdit(rowId: string, columnKey: string, initial?: string) {
    if (!columnByKey.get(columnKey)?.editable) return;
    editingRef.current = { rowId, columnKey };
    setEditingCell({ rowId, columnKey, initial });
  }

  /** Ends the edit of `columnKey` in `row` — `false` if it was already over (its editor's late blur). */
  function endEdit(row: T, columnKey: string): boolean {
    const current = editingRef.current;
    if (!current || current.rowId !== getRowId(row) || current.columnKey !== columnKey) return false;
    editingRef.current = null;
    setEditingCell(null);
    return true;
  }

  /**
   * Commits an edit. With `cellNavigation`, `move` then goes to the next cell
   * the way a spreadsheet does — `"down"` after Enter, `"next"`/`"prev"` after
   * Tab/Shift+Tab — and gives the grid focus back.
   */
  function commitEdit(row: T, columnKey: string, value: string, move?: EditMove) {
    if (!endEdit(row, columnKey)) return;
    columnByKey.get(columnKey)?.onCellEdit?.(row, columnKey, value);
    onCellEditCommit?.(row, columnKey, value);
    if (!cellNavigation) return;
    const index = indexOfRow(getRowId(row));
    if (move === "down") nav.moveTo(index + 1, columnKey);
    else if (move === "next" || move === "prev") nav.moveAcross(index, columnKey, move === "next" ? 1 : -1);
    else nav.moveTo(index, columnKey);
  }

  function cancelEdit() {
    const current = editingRef.current;
    editingRef.current = null;
    setEditingCell(null);
    if (cellNavigation && current) nav.moveTo(indexOfRow(current.rowId), current.columnKey);
  }

  return { startEdit, commitEdit, cancelEdit };
}

export type GridEditing<T> = ReturnType<typeof dataGridEditing<T>>;
