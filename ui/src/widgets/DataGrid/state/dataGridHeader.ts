import type { DragEvent as ReactDragEvent, MouseEvent as ReactMouseEvent, PointerEvent as ReactPointerEvent } from "react";
import type { DataGridColumn } from "../DataGrid";
import type { GridCommits } from "./dataGridCommits";
import { clamp, COLUMN_DRAG_MIME, DEFAULT_COLUMN_WIDTH, DEFAULT_MIN_WIDTH, MAX_COLUMN_WIDTH, nextSortState } from "./dataGridRows";
import type { GridState } from "./useDataGridState";
import type { GridView } from "./useDataGridView";

/** The header's mouse: sorting by click, resizing by the edge, reordering by drag, grouping by a drop on the group panel. */
export function dataGridHeader<T>(state: GridState<T>, view: GridView<T>, commits: GridCommits) {
  const { dragColumnKey, setDragColumnKey, activeSort, widths, setWidths, dropIndicator, setDropIndicator, setColumnOrder, setGroupPanelHover, activeGroupBy } = state;

  function handleHeaderClick(e: ReactMouseEvent, column: DataGridColumn<T>) {
    if (!column.sortable || column.lockSort || dragColumnKey) return;
    commits.commitSort(nextSortState(activeSort, column.key, e.metaKey || e.ctrlKey));
  }

  function handleResizeStart(e: ReactPointerEvent, column: DataGridColumn<T>) {
    e.stopPropagation();
    e.preventDefault();
    const startX = e.clientX;
    const startWidth = widths[column.key] ?? DEFAULT_COLUMN_WIDTH;
    const minWidth = column.minWidth ?? DEFAULT_MIN_WIDTH;
    const onMove = (ev: PointerEvent) => setWidths((w) => ({ ...w, [column.key]: clamp(startWidth + ev.clientX - startX, minWidth, MAX_COLUMN_WIDTH) }));
    const onUp = () => {
      window.removeEventListener("pointermove", onMove);
      window.removeEventListener("pointerup", onUp);
      window.removeEventListener("pointercancel", onUp);
    };
    window.addEventListener("pointermove", onMove);
    window.addEventListener("pointerup", onUp);
    window.addEventListener("pointercancel", onUp);
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
    setDropIndicator({ key: column.key, side: e.clientX - rect.left < rect.width / 2 ? "before" : "after" });
  }

  function handleHeaderDrop(e: ReactDragEvent, column: DataGridColumn<T>) {
    e.preventDefault();
    const draggedKey = e.dataTransfer.getData(COLUMN_DRAG_MIME);
    if (draggedKey && draggedKey !== column.key) {
      setColumnOrder((order) => {
        const without = order.filter((k) => k !== draggedKey);
        const index = without.indexOf(column.key) + (dropIndicator?.side === "after" ? 1 : 0);
        without.splice(index, 0, draggedKey);
        return without;
      });
    }
    handleHeaderDragEnd();
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
    const column = key ? view.columnByKey.get(key) : undefined;
    if (key && column && !column.lockGroup && !activeGroupBy.includes(key)) commits.commitGroupBy([...activeGroupBy, key]);
    setGroupPanelHover(false);
    setDragColumnKey(null);
  }

  return { handleHeaderClick, handleResizeStart, handleHeaderDragStart, handleHeaderDragOver, handleHeaderDrop, handleHeaderDragEnd, handleGroupPanelDragOver, handleGroupPanelDrop };
}
