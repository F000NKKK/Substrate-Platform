import { useCallback, useEffect } from "react";
import type { DataGridProps } from "../DataGrid";
import { dataGridCommits } from "./dataGridCommits";
import { dataGridEditing } from "./dataGridEditing";
import { dataGridHeader } from "./dataGridHeader";
import { dataGridKeys } from "./dataGridKeys";
import { dataGridNavigation } from "./dataGridNavigation";
import { useDataGridState, withDefaults } from "./useDataGridState";
import { useDataGridView } from "./useDataGridView";

export { buildDisplayItems, DEFAULT_ROW_HEIGHT, filterRows, formatAggregate, sortRows } from "./dataGridRows";
export type { DataGridMenuTarget } from "./dataGridRows";

/**
 * All of DataGrid's state and behavior — multi-column sort, drag-to-group
 * with aggregates, per-column filtering, column resize/reorder, selection,
 * and virtualization — with no rendering. `DataGrid.tsx` is the view over
 * this; `formatAggregate` is exported for the view's group/footer cells.
 */
export function useDataGrid<T>(gridProps: DataGridProps<T>) {
  const props = withDefaults(gridProps);
  const state = useDataGridState(props);
  const view = useDataGridView(props, state);
  const commits = dataGridCommits(props, state, view);
  const nav = dataGridNavigation(props, state, view, commits);
  const editing = dataGridEditing(props, state, view, nav);
  const header = dataGridHeader(state, view, commits);
  const { handleItemKeyDown } = dataGridKeys({ props, state, view, commits, nav, editing });
  const { scrollRef, setViewportHeight, pendingFocusRef, editingCell, activeIndex } = state;

  // Keyboard navigation moved the active row: focus it once it's rendered
  // (it may only appear after the scroll it caused).
  useEffect(() => {
    if (!pendingFocusRef.current || editingCell) return;
    const el = scrollRef.current?.querySelector<HTMLElement>(`[data-index="${activeIndex}"]`);
    if (!el) return;
    pendingFocusRef.current = false;
    el.focus({ preventScroll: true });
  });

  const attachScrollRef = useCallback(
    (el: HTMLDivElement | null) => {
      scrollRef.current = el;
      if (el) setViewportHeight(el.clientHeight);
    },
    [scrollRef, setViewportHeight],
  );

  const { commitSelection, commitSort, commitGroupBy, toggleGroupPanelVisible, toggleGroupCollapsed, toggleColumnHidden, resetToDefaults } = commits;
  const { selectRow, focusCell } = nav;
  const { columnByKey, orderedColumns, visibleColumns, summaryColumns, gridTemplate, sortedRows, sortedIndexOf, displayItems, visibleItems, startIndex, totalHeight, footerAggregates, allSelected, activeCellKey } = view;
  const { activeSort, activeGroupBy, activeSelection, dragColumnKey, dropIndicator, groupPanelHover, setGroupPanelHover, groupPanelVisible, filters, setFilters, collapsedGroups, setActiveIndex, flash, setScrollTop, activeHiddenColumns, activeColumnKey, setActiveColumnKey, gridMenu } = state;
  return {
    columnByKey, orderedColumns, visibleColumns, summaryColumns, gridTemplate, activeSort, activeGroupBy, activeSelection, dragColumnKey, dropIndicator, groupPanelHover, setGroupPanelHover, groupPanelVisible, toggleGroupPanelVisible,
    filters, setFilters, sortedRows, sortedIndexOf, displayItems, visibleItems, startIndex, totalHeight, collapsedGroups, activeIndex, setActiveIndex, flash, footerAggregates, allSelected,
    commitSelection, commitSort, commitGroupBy, ...header, toggleGroupCollapsed, selectRow, handleItemKeyDown, attachScrollRef, setScrollTop, activeHiddenColumns, toggleColumnHidden, resetToDefaults,
    editingCell, ...editing, focusCell, activeCellKey, activeColumnKey, setActiveColumnKey, gridMenu,
  };
}
