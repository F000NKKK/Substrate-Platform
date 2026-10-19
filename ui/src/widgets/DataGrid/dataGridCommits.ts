import type { DataGridSortState } from "./DataGrid";
import { DEFAULT_COLUMN_WIDTH, SORT_FLASH_MS } from "./dataGridRows";
import type { GridProps, GridState } from "./useDataGridState";
import type { GridView } from "./useDataGridView";

/** Changes to what the grid shows: selection, sort, grouping, hidden columns, groups folded, back to the defaults. Each goes to the owner when it's controlled, or the grid's own state. */
export function dataGridCommits<T>(props: GridProps<T>, state: GridState<T>, view: GridView<T>) {
  const { columns, selectedIds, onSelectionChange, sort, onSortChange, groupBy, onGroupByChange, hiddenColumns, onHiddenColumnsChange, defaultSort, defaultGroupBy } = props;
  const { setInternalSelection, setInternalSort, setInternalGroupBy, setInternalHiddenColumns, setFlash, activeHiddenColumns, activeGroupBy, setColumnOrder, setWidths, setCollapsedGroups, setGroupPanelVisible } = state;

  function commitSelection(next: Set<string>) {
    if (selectedIds === undefined) setInternalSelection(next);
    onSelectionChange?.(next);
  }

  function commitSort(next: DataGridSortState[]) {
    if (sort === undefined) setInternalSort(next);
    onSortChange?.(next);
    setFlash(true);
    window.setTimeout(() => setFlash(false), SORT_FLASH_MS);
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
    if (view.columnByKey.get(key)?.hideable === false) return;
    const next = new Set(activeHiddenColumns);
    if (next.has(key)) next.delete(key);
    else next.add(key);
    commitHiddenColumns(next);
    // Hiding the last visible column leaves nothing to group by — clear it
    // rather than keep a grouping the grid can no longer show any column for.
    const anyStillVisible = columns.some((c) => !next.has(c.key) && !activeGroupBy.includes(c.key));
    if (!anyStillVisible && activeGroupBy.length > 0) commitGroupBy([]);
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

  function toggleGroupCollapsed(key: string) {
    setCollapsedGroups((prev) => {
      const next = new Set(prev);
      if (next.has(key)) next.delete(key);
      else next.add(key);
      return next;
    });
  }

  return { commitSelection, commitSort, commitGroupBy, toggleColumnHidden, resetToDefaults, toggleGroupPanelVisible, toggleGroupCollapsed };
}

export type GridCommits = ReturnType<typeof dataGridCommits>;
