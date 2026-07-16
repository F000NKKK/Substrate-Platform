import type { ContextMenuItem } from "../ContextMenu";
import type { DataGridColumn, DataGridSortState } from "./DataGrid";
import type { DataGridMenuTarget } from "./useDataGrid";

interface MenuGrid<T> {
  orderedColumns: DataGridColumn<T>[];
  activeHiddenColumns: Set<string>;
  activeGroupBy: string[];
  activeSort: DataGridSortState[];
  toggleColumnHidden: (key: string) => void;
  commitGroupBy: (next: string[]) => void;
  commitSort: (next: DataGridSortState[]) => void;
  resetToDefaults: () => void;
}

export function buildDataGridMenuItems<T>(
  target: DataGridMenuTarget<T> | null,
  grid: MenuGrid<T>
): ContextMenuItem[] {
  if (!target) return [];

  if (target.kind === "grid") {
    return [
      { label: "Reset to default", onSelect: grid.resetToDefaults },
      {
        label: "Clear grouping",
        disabled: grid.activeGroupBy.length === 0,
        onSelect: () => grid.commitGroupBy([]),
      },
      {
        label: "Manage columns",
        submenu: grid.orderedColumns.map((column) => ({
          label: column.header,
          checked: !grid.activeHiddenColumns.has(column.key),
          disabled: column.hideable === false,
          onSelect: () => grid.toggleColumnHidden(column.key),
        })),
      },
    ];
  }

  const column = target.column;
  const grouped = grid.activeGroupBy.includes(column.key);
  const sortIndex = grid.activeSort.findIndex((s) => s.key === column.key);
  const sortedAsc = sortIndex !== -1 && grid.activeSort[sortIndex].direction === "asc";
  const sortedDesc = sortIndex !== -1 && grid.activeSort[sortIndex].direction === "desc";

  return [
    {
      label: "Hide column",
      disabled: column.hideable === false,
      onSelect: () => grid.toggleColumnHidden(column.key),
    },
    {
      label: grouped ? "Stop grouping" : "Group by this column",
      disabled: column.lockGroup,
      onSelect: () => grid.commitGroupBy(grouped ? grid.activeGroupBy.filter((k) => k !== column.key) : [...grid.activeGroupBy, column.key]),
    },
    {
      label: "Sort ascending",
      checked: sortedAsc,
      disabled: !column.sortable || column.lockSort,
      onSelect: () => grid.commitSort([{ key: column.key, direction: "asc" }]),
    },
    {
      label: "Sort descending",
      checked: sortedDesc,
      disabled: !column.sortable || column.lockSort,
      onSelect: () => grid.commitSort([{ key: column.key, direction: "desc" }]),
    },
    {
      label: "Clear sort",
      disabled: (!column.sortable || column.lockSort) && sortIndex === -1,
      onSelect: () => grid.commitSort(grid.activeSort.filter((s) => s.key !== column.key)),
    },
  ];
}
