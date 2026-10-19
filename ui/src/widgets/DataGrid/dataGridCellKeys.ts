import type { KeyboardEvent } from "react";
import { clamp } from "./dataGridRows";
import type { GridParts } from "./gridParts";

/** The keys of a data row's cells under `cellNavigation`: Left/Right, Home/End, Tab, F2/Enter, and typing to edit. */
export function dataGridCellKeys<T>({ props, view, nav, editing }: GridParts<T>) {
  const { getRowId } = props;
  const { columnByKey, visibleColumns, activeCellKey } = view;

  /** `cellNavigation` keys on a data row — `true` if handled. */
  function handleCellKey(e: KeyboardEvent, index: number, row: T): boolean {
    const keys = visibleColumns.map((c) => c.key);
    const current = activeCellKey;
    const column = current ? columnByKey.get(current) : undefined;
    const plain = !e.ctrlKey && !e.metaKey && !e.altKey;
    const handled = (): true => {
      e.preventDefault();
      return true;
    };
    if ((e.key === "ArrowLeft" || e.key === "ArrowRight") && plain && current) {
      nav.moveTo(index, keys[clamp(keys.indexOf(current) + (e.key === "ArrowRight" ? 1 : -1), 0, keys.length - 1)]);
      return handled();
    }
    if ((e.key === "Home" || e.key === "End") && plain && current) {
      nav.moveTo(index, e.key === "Home" ? keys[0] : keys[keys.length - 1]);
      return handled();
    }
    if (e.key === "Tab" && plain) {
      nav.moveAcross(index, current, e.shiftKey ? -1 : 1);
      return handled();
    }
    if (!column?.editable) return false;
    // F2/Enter edit the cell; typing on it starts editing with what was typed, like a spreadsheet.
    if (e.key === "F2" || e.key === "Enter") editing.startEdit(getRowId(row), column.key);
    else if (plain && e.key.length === 1) editing.startEdit(getRowId(row), column.key, e.key);
    else return false;
    return handled();
  }

  return { handleCellKey };
}
