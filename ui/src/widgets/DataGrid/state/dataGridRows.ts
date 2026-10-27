// DataGrid's rows as data: filtering, sorting, grouping with aggregates —
// pure functions over the columns — and the grid's sizes.

import type { DataGridColumn, DataGridSortState } from "../DataGrid";

export type DataGridMenuTarget<T> = { kind: "grid" } | { kind: "header"; column: DataGridColumn<T> };

export type DisplayItem<T> =
  | { kind: "group"; key: string; level: number; columnKey: string; label: string; count: number; aggregates: Record<string, number>; rows: T[] }
  | { kind: "row"; row: T };

/** = `--sp-control-height`: a row fits an inline editor or a 20px thumbnail. */
export const DEFAULT_ROW_HEIGHT = 24;
export const DEFAULT_COLUMN_WIDTH = 140;
export const DEFAULT_MIN_WIDTH = 48;
export const SELECT_COLUMN_WIDTH = 32;
export const OVERSCAN = 6;
export const COLUMN_DRAG_MIME = "application/x-sp-datagrid-column";
/** The widest a column can be dragged to. */
export const MAX_COLUMN_WIDTH = 2000;
/** The viewport's height before it's measured. */
export const DEFAULT_VIEWPORT_HEIGHT = 480;
/** How long the header flashes after the sort changes, in ms. */
export const SORT_FLASH_MS = 160;
/** What a group of blank values is called. */
export const BLANK_GROUP = "(blank)";

/** Text order for sorting and grouping: `File2` before `File10`, case-insensitive. One collator — `localeCompare` with options builds a new one per call. */
const naturalOrder = new Intl.Collator(undefined, { numeric: true, sensitivity: "base" });

function compareValues(a: string | number, b: string | number): number {
  if (typeof a === "number" && typeof b === "number") return a - b;
  return naturalOrder.compare(String(a), String(b));
}

export function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

export function nextSortState(current: DataGridSortState[], key: string, additive: boolean): DataGridSortState[] {
  const index = current.findIndex((s) => s.key === key);
  if (!additive) {
    if (index === -1) return [{ key, direction: "asc" }];
    return current[index].direction === "asc" ? [{ key, direction: "desc" }] : [];
  }
  if (index === -1) return [...current, { key, direction: "asc" }];
  if (current[index].direction === "asc") {
    const next = [...current];
    next[index] = { key, direction: "desc" };
    return next;
  }
  return current.filter((s) => s.key !== key);
}

export function formatAggregate<T>(column: DataGridColumn<T>, value: number): string {
  if (column.formatSummary) return column.formatSummary(value);
  if (column.summary === "avg") return value.toFixed(1);
  return value.toLocaleString();
}

export function computeAggregates<T>(columns: DataGridColumn<T>[], rows: T[]): Record<string, number> {
  const aggregates: Record<string, number> = {};
  for (const column of columns) {
    if (!column.summary || !column.summaryValue) continue;
    if (column.summary === "count") {
      aggregates[column.key] = rows.length;
      continue;
    }
    const sum = rows.reduce((acc, row) => acc + column.summaryValue!(row), 0);
    aggregates[column.key] = column.summary === "avg" ? sum / Math.max(1, rows.length) : sum;
  }
  return aggregates;
}

function groupValueOf<T>(column: DataGridColumn<T>, row: T): string {
  if (column.groupValue) return column.groupValue(row);
  if (column.filterValue) return column.filterValue(row);
  if (column.sortValue) return String(column.sortValue(row));
  return "";
}

/** `rows` whose every non-blank filter (column key → needle) is a case-insensitive substring of that column's filter text. */
export function filterRows<T>(rows: T[], filters: Record<string, string>, columnByKey: Map<string, DataGridColumn<T>>): T[] {
  const active = Object.entries(filters).flatMap(([key, needle]) => {
    const column = columnByKey.get(key);
    return column && needle.trim() !== "" ? [{ column, needle: needle.toLowerCase() }] : [];
  });
  if (active.length === 0) return rows;
  return rows.filter((row) =>
    active.every(({ column, needle }) => {
      const text = column.filterValue ? column.filterValue(row) : column.sortValue ? String(column.sortValue(row)) : "";
      return text.toLowerCase().includes(needle);
    })
  );
}

/** `rows` ordered by each sort key in turn (a stable sort; `rows` itself is left alone). */
export function sortRows<T>(rows: T[], sort: DataGridSortState[], columnByKey: Map<string, DataGridColumn<T>>): T[] {
  if (sort.length === 0) return rows;
  const keys = sort.flatMap((s) => {
    const column = columnByKey.get(s.key);
    return column ? [{ value: column.sortValue, sign: s.direction === "asc" ? 1 : -1 }] : [];
  });
  // Each row's sort values computed once, not once per comparison.
  const decorated = rows.map((row, index) => ({ row, index, values: keys.map((k) => (k.value ? k.value(row) : "")) }));
  decorated.sort((a, b) => {
    for (let i = 0; i < keys.length; i++) {
      const cmp = compareValues(a.values[i], b.values[i]) * keys[i].sign;
      if (cmp !== 0) return cmp;
    }
    return a.index - b.index;
  });
  return decorated.map((d) => d.row);
}

/** What grouping needs at every level: the grouped-by keys, the columns, and which groups are folded. */
interface Grouping<T> {
  groupBy: string[];
  columnByKey: Map<string, DataGridColumn<T>>;
  collapsedGroups: Set<string>;
}

/** `rows` as the grid shows them: grouped by each `groupBy` column in turn (each group's rows only while it's unfolded), or as they are. */
export function buildDisplayItems<T>(rows: T[], groupBy: string[], columnByKey: Map<string, DataGridColumn<T>>, collapsedGroups: Set<string>): DisplayItem<T>[] {
  return groupLevel({ groupBy, columnByKey, collapsedGroups }, rows, 0, "");
}

function bucketsOf<T>(column: DataGridColumn<T>, rows: T[]): Map<string, T[]> {
  const buckets = new Map<string, T[]>();
  for (const row of rows) {
    const value = groupValueOf(column, row);
    const bucket = buckets.get(value);
    if (bucket) bucket.push(row);
    else buckets.set(value, [row]);
  }
  return buckets;
}

function groupLevel<T>(grouping: Grouping<T>, rows: T[], level: number, path: string): DisplayItem<T>[] {
  const { groupBy, columnByKey, collapsedGroups } = grouping;
  if (level >= groupBy.length) return rows.map((row) => ({ kind: "row", row }));
  const column = columnByKey.get(groupBy[level]);
  if (!column) return groupLevel(grouping, rows, level + 1, path);
  const buckets = bucketsOf(column, rows);
  const columns = [...columnByKey.values()];
  return [...buckets.keys()].sort(naturalOrder.compare).flatMap((value): DisplayItem<T>[] => {
    const bucketRows = buckets.get(value) ?? [];
    const key = `${path}/${column.key}:${value}`;
    const group: DisplayItem<T> = { kind: "group", key, level, columnKey: column.key, label: value || BLANK_GROUP, count: bucketRows.length, aggregates: computeAggregates(columns, bucketRows), rows: bucketRows };
    return collapsedGroups.has(key) ? [group] : [group, ...groupLevel(grouping, bucketRows, level + 1, key)];
  });
}
