import type { CSSProperties, ReactNode } from "react";
import { ContextMenu } from "../ContextMenu";
import { useDataGrid, formatAggregate } from "./useDataGrid";
import { buildDataGridMenuItems } from "./contextMenuItems";
import "./DataGrid.css";

export type SortDirection = "asc" | "desc";

export interface DataGridSortState {
  key: string;
  direction: SortDirection;
}

export interface DataGridColumn<T> {
  key: string;
  header: ReactNode;
  /** Initial pixel width; the user can resize from here. */
  width?: number;
  minWidth?: number;
  align?: "left" | "right" | "center";
  sortable?: boolean;
  /** Value the default comparator sorts on; omit for columns rendered from more than one field. */
  sortValue?: (row: T) => string | number;
  /** Text the filter row matches against (case-insensitive substring). Falls back to `sortValue`; omit both to make the column unfilterable. */
  filterValue?: (row: T) => string;
  /** Value rows are bucketed by when this column is dragged into the group panel. Falls back to `filterValue`/`sortValue`. */
  groupValue?: (row: T) => string;
  /** Aggregate shown in a group's summary line and the grid's footer row. */
  summary?: "sum" | "count" | "avg";
  /** Required for "sum"/"avg" — the numeric value each row contributes. */
  summaryValue?: (row: T) => number;
  formatSummary?: (value: number) => string;
  render?: (row: T) => ReactNode;
  /** Set false to force this column permanently visible — the context menu's hide/manage-columns actions no-op on it. Default true. */
  hideable?: boolean;
  /** Disables the sort interaction (header click and the context menu's sort actions) without touching `sortable`, which still gates whether the column can ever be sorted at all. */
  lockSort?: boolean;
  /** Column can't be dragged or menu'd into the group panel. Column reordering is unaffected. */
  lockGroup?: boolean;
  /** Enables double-click/Enter inline editing on this column's cells. */
  editable?: boolean;
  /** Read value the default editor pre-fills with; falls back to the raw cell value. Distinct from `sortValue`, which can be numeric and serves a comparator, not display/edit text. */
  editValue?: (row: T) => string;
  /** Called with the row, this column's key, and the committed string value when an edit commits. */
  onCellEdit?: (row: T, key: string, newValue: string) => void;
  /** Custom editor overriding the default text `<input>`; call `commit(value)` or `cancel()` to end editing. */
  renderEdit?: (row: T, commit: (value: string) => void, cancel: () => void) => ReactNode;
}

export interface DataGridProps<T> {
  columns: DataGridColumn<T>[];
  rows: T[];
  getRowId: (row: T) => string;
  /** Row height in px — also the virtualization unit, so keep it fixed. Default 28. */
  rowHeight?: number;
  selectable?: boolean;
  selectedIds?: Set<string>;
  defaultSelectedIds?: Set<string>;
  onSelectionChange?: (ids: Set<string>) => void;
  sort?: DataGridSortState[];
  defaultSort?: DataGridSortState[];
  onSortChange?: (sort: DataGridSortState[]) => void;
  groupBy?: string[];
  defaultGroupBy?: string[];
  onGroupByChange?: (groupBy: string[]) => void;
  hiddenColumns?: Set<string>;
  defaultHiddenColumns?: Set<string>;
  onHiddenColumnsChange?: (hidden: Set<string>) => void;
  /** Shows a per-column text filter beneath the header. Default true. */
  filterRow?: boolean;
  onRowActivate?: (row: T) => void;
  /** Grid-level catch-all fired alongside any column's own `onCellEdit` when an edit commits. */
  onCellEditCommit?: (row: T, key: string, newValue: string) => void;
  emptyState?: ReactNode;
  className?: string;
  style?: CSSProperties;
}

/**
 * A dense, virtualized data grid with the trimmings of a full desktop grid:
 * multi-column sort (ctrl/cmd-click a header to add a secondary key),
 * drag-to-group with per-group aggregates, a per-column filter row, and
 * drag-to-reorder columns. Rows above ~50 items should always go through
 * this rather than a plain mapped `<div>` list — the windowing is what keeps
 * a few-thousand-row grid smooth, and grouping/filtering both run before it.
 * All state and behavior live in `useDataGrid`; this component is the view.
 */
export function DataGrid<T>(props: DataGridProps<T>) {
  const { selectable = false, filterRow = true, onRowActivate, emptyState, className, style, rowHeight = 28 } = props;
  const grid = useDataGrid(props);

  return (
    <div className={["sp-datagrid", className].filter(Boolean).join(" ")} style={style}>
      <div
        className="sp-datagrid-grouppanel"
        data-hover={grid.groupPanelHover || undefined}
        onDragOver={grid.handleGroupPanelDragOver}
        onDragLeave={() => grid.setGroupPanelHover(false)}
        onDrop={grid.handleGroupPanelDrop}
      >
        {grid.activeGroupBy.length === 0 ? (
          <span className="sp-datagrid-grouppanel-placeholder">Drag a column header here to group by that column</span>
        ) : (
          grid.activeGroupBy.map((key) => (
            <span className="sp-datagrid-groupchip" key={key}>
              {grid.columnByKey.get(key)?.header ?? key}
              <button
                type="button"
                className="sp-datagrid-groupchip-remove"
                aria-label={`Stop grouping by ${key}`}
                onClick={() => grid.commitGroupBy(grid.activeGroupBy.filter((k) => k !== key))}
              >
                ×
              </button>
            </span>
          ))
        )}
      </div>

      <div className="sp-datagrid-header" role="row" style={{ gridTemplateColumns: grid.gridTemplate }}>
        {selectable && (
          <div className="sp-datagrid-headercell sp-datagrid-headercell--select">
            <input
              type="checkbox"
              checked={grid.allSelected}
              onChange={() =>
                grid.commitSelection(grid.allSelected ? new Set() : new Set(grid.sortedRows.map(props.getRowId)))
              }
              aria-label="Select all rows"
            />
          </div>
        )}
        {grid.visibleColumns.map((column) => {
          const sortIndex = grid.activeSort.findIndex((s) => s.key === column.key);
          const isSorted = sortIndex !== -1;
          return (
            <div
              key={column.key}
              className="sp-datagrid-headercell"
              data-align={column.align ?? "left"}
              data-sortable={column.sortable || undefined}
              data-dragging={grid.dragColumnKey === column.key || undefined}
              data-drop={grid.dropIndicator?.key === column.key ? grid.dropIndicator.side : undefined}
              role="columnheader"
              aria-sort={isSorted ? (grid.activeSort[sortIndex].direction === "asc" ? "ascending" : "descending") : "none"}
              draggable
              onClick={(e) => grid.handleHeaderClick(e, column)}
              onDragStart={(e) => grid.handleHeaderDragStart(e, column)}
              onDragOver={(e) => grid.handleHeaderDragOver(e, column)}
              onDrop={(e) => grid.handleHeaderDrop(e, column)}
              onDragEnd={grid.handleHeaderDragEnd}
            >
              <span className="sp-datagrid-headercell-label">{column.header}</span>
              {column.sortable && (
                <span
                  className="sp-datagrid-sorticon"
                  data-active={isSorted || undefined}
                  data-dir={isSorted ? grid.activeSort[sortIndex].direction : "asc"}
                >
                  ▾
                </span>
              )}
              {grid.activeSort.length > 1 && isSorted && <span className="sp-datagrid-sortbadge">{sortIndex + 1}</span>}
              <span
                className="sp-datagrid-resize-handle"
                onPointerDown={(e) => grid.handleResizeStart(e, column)}
                onClick={(e) => e.stopPropagation()}
              />
            </div>
          );
        })}
      </div>

      {filterRow && (
        <div className="sp-datagrid-filterrow" style={{ gridTemplateColumns: grid.gridTemplate }}>
          {selectable && <div className="sp-datagrid-filtercell sp-datagrid-filtercell--select" />}
          {grid.visibleColumns.map((column) => {
            const filterable = !!(column.filterValue || column.sortValue);
            return (
              <div className="sp-datagrid-filtercell" key={column.key}>
                {filterable && (
                  <input
                    className="sp-datagrid-filterinput"
                    placeholder="Filter…"
                    value={grid.filters[column.key] ?? ""}
                    onChange={(e) => grid.setFilters((f) => ({ ...f, [column.key]: e.target.value }))}
                    onKeyDown={(e) => {
                      if (e.key === "Escape") grid.setFilters((f) => ({ ...f, [column.key]: "" }));
                    }}
                    aria-label={`Filter ${typeof column.header === "string" ? column.header : column.key}`}
                  />
                )}
              </div>
            );
          })}
        </div>
      )}

      <div
        className="sp-datagrid-body"
        ref={grid.attachScrollRef}
        role="rowgroup"
        data-flash={grid.flash || undefined}
        onScroll={(e) => grid.setScrollTop(e.currentTarget.scrollTop)}
      >
        {grid.displayItems.length === 0 ? (
          <div className="sp-datagrid-empty">{emptyState ?? "No rows"}</div>
        ) : (
          <div className="sp-datagrid-spacer" style={{ height: grid.totalHeight }}>
            {grid.visibleItems.map((item, i) => {
              const index = grid.startIndex + i;

              if (item.kind === "group") {
                const collapsed = grid.collapsedGroups.has(item.key);
                const column = grid.columnByKey.get(item.columnKey);
                return (
                  <div
                    key={item.key}
                    className="sp-datagrid-grouprow"
                    role="row"
                    tabIndex={index === grid.activeIndex ? 0 : -1}
                    style={{ top: index * rowHeight, height: rowHeight, paddingLeft: item.level * 20 }}
                    onFocus={() => grid.setActiveIndex(index)}
                    onKeyDown={(e) => grid.handleItemKeyDown(e, index)}
                    onClick={() => {
                      grid.setActiveIndex(index);
                      grid.toggleGroupCollapsed(item.key);
                    }}
                  >
                    <span className="sp-datagrid-groupchevron" data-collapsed={collapsed || undefined}>
                      ▾
                    </span>
                    <span className="sp-datagrid-grouplabel">
                      {column?.header ?? item.columnKey}: <strong>{item.label}</strong>
                    </span>
                    <span className="sp-datagrid-groupcount">{item.count} items</span>
                    {Object.entries(item.aggregates).map(([key, value]) => {
                      const col = grid.columnByKey.get(key);
                      if (!col) return null;
                      return (
                        <span className="sp-datagrid-groupaggregate" key={key}>
                          {col.summary}({typeof col.header === "string" ? col.header : key}): {formatAggregate(col, value)}
                        </span>
                      );
                    })}
                  </div>
                );
              }

              const row = item.row;
              const id = props.getRowId(row);
              const selected = grid.activeSelection.has(id);
              const rowIndexInSorted = grid.sortedRows.indexOf(row);
              return (
                <div
                  key={id}
                  className="sp-datagrid-row"
                  role="row"
                  data-selected={selected || undefined}
                  tabIndex={index === grid.activeIndex ? 0 : -1}
                  style={{ top: index * rowHeight, height: rowHeight, gridTemplateColumns: grid.gridTemplate }}
                  onFocus={() => grid.setActiveIndex(index)}
                  onKeyDown={(e) => grid.handleItemKeyDown(e, index)}
                  onDoubleClick={() => onRowActivate?.(row)}
                  onContextMenu={(e) => e.stopPropagation()}
                  onClick={(e) => {
                    grid.setActiveIndex(index);
                    if (!selectable) return;
                    grid.selectRow(rowIndexInSorted, { toggle: e.metaKey || e.ctrlKey, range: e.shiftKey });
                  }}
                >
                  {selectable && (
                    <div className="sp-datagrid-cell sp-datagrid-cell--select" onClick={(e) => e.stopPropagation()}>
                      <input
                        type="checkbox"
                        checked={selected}
                        onChange={() => grid.selectRow(rowIndexInSorted, { toggle: true })}
                        aria-label={`Select row ${id}`}
                      />
                    </div>
                  )}
                  {grid.visibleColumns.map((column) => {
                    const editing = grid.editingCell?.rowId === id && grid.editingCell.columnKey === column.key;
                    return (
                      <div
                        key={column.key}
                        className="sp-datagrid-cell"
                        data-align={column.align ?? "left"}
                        data-editing={editing || undefined}
                        onClick={() => grid.setActiveColumnKey(column.key)}
                        onDoubleClick={(e) => {
                          if (!column.editable) return;
                          e.stopPropagation();
                          grid.startEdit(id, column.key);
                        }}
                      >
                        {editing ? (
                          column.renderEdit ? (
                            column.renderEdit(
                              row,
                              (value) => grid.commitEdit(row, column.key, value),
                              grid.cancelEdit
                            )
                          ) : (
                            <input
                              className="sp-datagrid-cell-editor"
                              autoFocus
                              defaultValue={column.editValue?.(row) ?? String((row as Record<string, unknown>)[column.key] ?? "")}
                              onClick={(e) => e.stopPropagation()}
                              onBlur={(e) => grid.commitEdit(row, column.key, e.target.value)}
                              onKeyDown={(e) => {
                                e.stopPropagation();
                                if (e.key === "Enter") grid.commitEdit(row, column.key, (e.target as HTMLInputElement).value);
                                if (e.key === "Escape") grid.cancelEdit();
                              }}
                            />
                          )
                        ) : column.render ? (
                          column.render(row)
                        ) : (
                          (row as Record<string, ReactNode>)[column.key]
                        )}
                      </div>
                    );
                  })}
                </div>
              );
            })}
          </div>
        )}
      </div>

      {grid.summaryColumns.length > 0 && (
        <div className="sp-datagrid-footer" style={{ gridTemplateColumns: grid.gridTemplate }}>
          {selectable && <div className="sp-datagrid-footercell sp-datagrid-footercell--select" />}
          {grid.visibleColumns.map((column) => (
            <div className="sp-datagrid-footercell" key={column.key} data-align={column.align ?? "left"}>
              {column.summary && column.summaryValue ? formatAggregate(column, grid.footerAggregates[column.key] ?? 0) : ""}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
