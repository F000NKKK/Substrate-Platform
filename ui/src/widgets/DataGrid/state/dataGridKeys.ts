import type { KeyboardEvent } from "react";
import { eventKey } from "../../../infra/keys";
import { dataGridCellKeys } from "./dataGridCellKeys";
import { clamp, type DisplayItem } from "./dataGridRows";
import type { GridParts } from "../gridParts";

/** The grid's keyboard: rows and groups (arrows, Home/End, Space, Enter, Ctrl+A) — and cells under `cellNavigation` (`dataGridCellKeys`). */
export function dataGridKeys<T>(parts: GridParts<T>) {
  const { props, state, view, commits, nav, editing } = parts;
  const { getRowId, selectable, cellNavigation, onGridKeyDown, onRowActivate } = props;
  const { activeColumnKey, collapsedGroups } = state;
  const { displayItems, sortedRows, sortedIndexOf, columnByKey } = view;
  const { handleCellKey } = dataGridCellKeys(parts);

  /** Up/Down: the next row — under `cellNavigation` focused and selected, as a spreadsheet's cell is. */
  function step(e: KeyboardEvent, index: number) {
    e.preventDefault();
    if (!cellNavigation) return nav.activate(index);
    nav.moveTo(index);
    nav.selectByKeyboard(index, e);
  }

  /** A group's Left/Right: fold it when it's open (`open`) — or unfold it when it isn't. */
  function fold(e: KeyboardEvent, item: DisplayItem<T>, open: boolean) {
    if (item.kind !== "group" || collapsedGroups.has(item.key) === open) return;
    e.preventDefault();
    commits.toggleGroupCollapsed(item.key);
  }

  function activateItem(item: DisplayItem<T>) {
    if (item.kind === "group") return commits.toggleGroupCollapsed(item.key);
    const column = activeColumnKey ? columnByKey.get(activeColumnKey) : undefined;
    if (column?.editable) editing.startEdit(getRowId(item.row), column.key);
    else onRowActivate?.(item.row);
  }

  function handleItemKeyDown(e: KeyboardEvent, index: number) {
    const item = displayItems[index];
    if (onGridKeyDown) {
      onGridKeyDown(e, item?.kind === "row" ? item.row : null, activeColumnKey);
      if (e.defaultPrevented) return;
    }
    if (cellNavigation && item?.kind === "row" && handleCellKey(e, index, item.row)) return;
    // Ctrl+A: every row, not the page's text.
    if ((e.ctrlKey || e.metaKey) && !e.altKey && eventKey(e) === "a" && selectable) {
      e.preventDefault();
      commits.commitSelection(new Set(sortedRows.map(getRowId)));
      return;
    }
    const last = displayItems.length - 1;
    switch (e.key) {
      case "ArrowDown":
        return step(e, clamp(index + 1, 0, last));
      case "ArrowUp":
        return step(e, clamp(index - 1, 0, last));
      case "ArrowRight":
        return fold(e, item, false);
      case "ArrowLeft":
        return fold(e, item, true);
      case "Home":
      case "End":
        e.preventDefault();
        return nav.activate(e.key === "Home" ? 0 : last);
      case " ":
        e.preventDefault();
        if (item.kind === "group") commits.toggleGroupCollapsed(item.key);
        else if (selectable) nav.selectRow(sortedIndexOf(item.row), { toggle: true });
        return;
      case "Enter":
        return activateItem(item);
    }
  }

  return { handleItemKeyDown };
}
