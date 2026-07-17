import type { DataGridSortState } from "./DataGrid";

export interface PersistedDataGridState {
  columnOrder?: string[];
  widths?: Record<string, number>;
  sort?: DataGridSortState[];
  groupBy?: string[];
  hiddenColumns?: string[];
  groupPanelVisible?: boolean;
}

const STORAGE_PREFIX = "sp-datagrid:";

/** Best-effort read — a corrupt/missing entry (or `localStorage` being unavailable) just means "nothing to restore," never a thrown error. */
export function loadDataGridState(key: string): PersistedDataGridState | undefined {
  try {
    const raw = localStorage.getItem(STORAGE_PREFIX + key);
    return raw ? (JSON.parse(raw) as PersistedDataGridState) : undefined;
  } catch {
    return undefined;
  }
}

export function saveDataGridState(key: string, state: PersistedDataGridState): void {
  try {
    localStorage.setItem(STORAGE_PREFIX + key, JSON.stringify(state));
  } catch {
    // Quota exceeded or unavailable — persistence is a nicety, not load-bearing.
  }
}
