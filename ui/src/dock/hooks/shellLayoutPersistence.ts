import type { PanelPlacement } from "../types";

export interface PersistedShellLayoutState {
  placements?: Record<string, PanelPlacement>;
  sizes?: Record<string, number>;
  centerActiveId?: string;
}

/** Where layouts lived before they moved into the `StateStore` — still read as a fallback (see `useShellLayout`), never written. */
const STORAGE_PREFIX = "sp-shell-layout:";

/** Best-effort read — a corrupt/missing entry (or `localStorage` being unavailable) just means "nothing to restore," never a thrown error. */
export function loadShellLayoutState(key: string): PersistedShellLayoutState | undefined {
  try {
    const raw = localStorage.getItem(STORAGE_PREFIX + key);
    return raw ? (JSON.parse(raw) as PersistedShellLayoutState) : undefined;
  } catch {
    return undefined;
  }
}
