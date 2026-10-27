import { useEffect, useMemo, useRef, useState } from "react";
import { useContextMenu } from "../../ContextMenu";
import type { DataGridProps, DataGridSortState } from "../DataGrid";
import { loadDataGridState, saveDataGridState } from "./dataGridPersistence";
import { DEFAULT_COLUMN_WIDTH, DEFAULT_ROW_HEIGHT, DEFAULT_VIEWPORT_HEIGHT, type DataGridMenuTarget } from "./dataGridRows";

const NO_SORT: DataGridSortState[] = [];
const NO_GROUPS: string[] = [];

/** The props with their defaults filled in. */
export function withDefaults<T>(props: DataGridProps<T>) {
  return {
    ...props,
    rowHeight: props.rowHeight ?? DEFAULT_ROW_HEIGHT,
    selectable: props.selectable ?? false,
    defaultSort: props.defaultSort ?? NO_SORT,
    defaultGroupBy: props.defaultGroupBy ?? NO_GROUPS,
    defaultHiddenColumns: props.defaultHiddenColumns ?? new Set<string>(),
    cellNavigation: props.cellNavigation ?? false,
    showSelectColumn: props.showSelectColumn ?? true,
  };
}

export type GridProps<T> = ReturnType<typeof withDefaults<T>>;

/** The grid's state: the controlled-or-own sort, grouping, selection and hidden columns, the layout (order, widths) — remembered under `persistKey` — and what's active, edited, dragged. */
export function useDataGridState<T>({ columns, persistKey, defaultSelectedIds, defaultSort, defaultGroupBy, defaultHiddenColumns, sort, groupBy, selectedIds, hiddenColumns }: GridProps<T>) {
  const persisted = useMemo(() => (persistKey ? loadDataGridState(persistKey) : undefined), [persistKey]);
  const [internalSelection, setInternalSelection] = useState<Set<string>>(defaultSelectedIds ?? new Set());
  const [internalSort, setInternalSort] = useState<DataGridSortState[]>(persisted?.sort ?? defaultSort);
  const [internalGroupBy, setInternalGroupBy] = useState<string[]>(persisted?.groupBy ?? defaultGroupBy);
  const [internalHiddenColumns, setInternalHiddenColumns] = useState<Set<string>>(persisted?.hiddenColumns ? new Set(persisted.hiddenColumns) : defaultHiddenColumns);
  const [columnOrder, setColumnOrder] = useState<string[]>(() => {
    const keys = columns.map((c) => c.key);
    const restored = persisted?.columnOrder?.filter((k) => keys.includes(k));
    if (!restored) return keys;
    // Any column added to `columns` since this was saved (new field on the
    // consumer's config) still needs to appear — append it at the end.
    return [...restored, ...keys.filter((k) => !restored.includes(k))];
  });
  const [widths, setWidths] = useState<Record<string, number>>(() => Object.fromEntries(columns.map((c) => [c.key, persisted?.widths?.[c.key] ?? c.width ?? DEFAULT_COLUMN_WIDTH])));
  const [filters, setFilters] = useState<Record<string, string>>({});
  const [collapsedGroups, setCollapsedGroups] = useState<Set<string>>(new Set());
  const [scrollTop, setScrollTop] = useState(0);
  const [viewportHeight, setViewportHeight] = useState(DEFAULT_VIEWPORT_HEIGHT);
  const [activeIndex, setActiveIndex] = useState(0);
  const [activeColumnKey, setActiveColumnKey] = useState<string | null>(null);
  const [editingCell, setEditingCell] = useState<{ rowId: string; columnKey: string; initial?: string } | null>(null);
  // The edit being committed/cancelled — its editor's blur on unmount mustn't commit it twice.
  const editingRef = useRef<{ rowId: string; columnKey: string } | null>(null);
  // Set by keyboard navigation: focus the active row once it has rendered.
  const pendingFocusRef = useRef(false);
  const [flash, setFlash] = useState(false);
  const [, setFocusTick] = useState(0);
  const [dragColumnKey, setDragColumnKey] = useState<string | null>(null);
  const [dropIndicator, setDropIndicator] = useState<{ key: string; side: "before" | "after" } | null>(null);
  const [groupPanelHover, setGroupPanelHover] = useState(false);
  const [groupPanelVisible, setGroupPanelVisible] = useState(persisted?.groupPanelVisible ?? true);
  const gridMenu = useContextMenu<DataGridMenuTarget<T>>();
  const scrollRef = useRef<HTMLDivElement>(null);
  const lastAnchorRef = useRef<number | null>(null);

  const activeSort = sort ?? internalSort;
  const activeGroupBy = groupBy ?? internalGroupBy;
  const activeSelection = selectedIds ?? internalSelection;
  const activeHiddenColumns = hiddenColumns ?? internalHiddenColumns;

  useEffect(() => {
    if (!persistKey) return;
    saveDataGridState(persistKey, { columnOrder, widths, sort: activeSort, groupBy: activeGroupBy, hiddenColumns: [...activeHiddenColumns], groupPanelVisible });
  }, [persistKey, columnOrder, widths, activeSort, activeGroupBy, activeHiddenColumns, groupPanelVisible]);

  return { setInternalSelection, setInternalSort, setInternalGroupBy, setInternalHiddenColumns, columnOrder, setColumnOrder, widths, setWidths, filters, setFilters, collapsedGroups, setCollapsedGroups, scrollTop, setScrollTop, viewportHeight, setViewportHeight, activeIndex, setActiveIndex, activeColumnKey, setActiveColumnKey, editingCell, setEditingCell, editingRef, pendingFocusRef, flash, setFlash, setFocusTick, dragColumnKey, setDragColumnKey, dropIndicator, setDropIndicator, groupPanelHover, setGroupPanelHover, groupPanelVisible, setGroupPanelVisible, gridMenu, scrollRef, lastAnchorRef, activeSort, activeGroupBy, activeSelection, activeHiddenColumns };
}

export type GridState<T> = ReturnType<typeof useDataGridState<T>>;
