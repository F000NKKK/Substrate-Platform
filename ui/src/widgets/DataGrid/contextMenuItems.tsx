import type { ContextMenuItem } from "../ContextMenu";
import { t } from "../../infra/i18n";
import type { DataGridColumn, DataGridSortState } from "./DataGrid";
import type { DataGridMenuTarget } from "./state/useDataGrid";

interface MenuGrid<T> {
  orderedColumns: DataGridColumn<T>[];
  activeHiddenColumns: Set<string>;
  activeGroupBy: string[];
  activeSort: DataGridSortState[];
  groupPanelVisible: boolean;
  toggleColumnHidden: (key: string) => void;
  toggleGroupPanelVisible: () => void;
  commitGroupBy: (next: string[]) => void;
  commitSort: (next: DataGridSortState[]) => void;
  resetToDefaults: () => void;
}

function hiddenColumnsSubmenu<T>(grid: MenuGrid<T>, separatorBefore = false): ContextMenuItem {
  const hidden = grid.orderedColumns.filter((c) => grid.activeHiddenColumns.has(c.key));
  return {
    label: t("Show hidden columns"),
    separatorBefore,
    submenu:
      hidden.length === 0
        ? [{ label: t("No hidden columns"), disabled: true }]
        : hidden.map((column) => ({
            label: column.header,
            onSelect: () => grid.toggleColumnHidden(column.key),
          })),
  };
}

export function buildDataGridMenuItems<T>(
  target: DataGridMenuTarget<T> | null,
  grid: MenuGrid<T>
): ContextMenuItem[] {
  if (!target) return [];

  if (target.kind === "grid") {
    return [
      { label: t("Reset to default"), onSelect: grid.resetToDefaults },
      {
        label: t("Clear grouping"),
        disabled: grid.activeGroupBy.length === 0,
        onSelect: () => grid.commitGroupBy([]),
      },
      {
        label: grid.groupPanelVisible ? t("Hide group panel") : t("Show group panel"),
        onSelect: grid.toggleGroupPanelVisible,
      },
      hiddenColumnsSubmenu(grid, true),
    ];
  }

  const column = target.column;
  const grouped = grid.activeGroupBy.includes(column.key);
  const sortIndex = grid.activeSort.findIndex((s) => s.key === column.key);
  const sortedAsc = sortIndex !== -1 && grid.activeSort[sortIndex].direction === "asc";
  const sortedDesc = sortIndex !== -1 && grid.activeSort[sortIndex].direction === "desc";

  return [
    {
      label: t("Hide column"),
      disabled: column.hideable === false,
      onSelect: () => grid.toggleColumnHidden(column.key),
    },
    {
      label: grouped ? t("Stop grouping") : t("Group by this column"),
      disabled: column.lockGroup,
      onSelect: () => grid.commitGroupBy(grouped ? grid.activeGroupBy.filter((k) => k !== column.key) : [...grid.activeGroupBy, column.key]),
    },
    {
      label: t("Sort ascending"),
      checked: sortedAsc,
      disabled: !column.sortable || column.lockSort,
      onSelect: () => grid.commitSort([{ key: column.key, direction: "asc" }]),
    },
    {
      label: t("Sort descending"),
      checked: sortedDesc,
      disabled: !column.sortable || column.lockSort,
      onSelect: () => grid.commitSort([{ key: column.key, direction: "desc" }]),
    },
    {
      label: t("Clear sort"),
      disabled: (!column.sortable || column.lockSort) && sortIndex === -1,
      onSelect: () => grid.commitSort(grid.activeSort.filter((s) => s.key !== column.key)),
    },
    {
      label: grid.groupPanelVisible ? t("Hide group panel") : t("Show group panel"),
      separatorBefore: true,
      onSelect: grid.toggleGroupPanelVisible,
    },
    hiddenColumnsSubmenu(grid, false),
  ];
}
