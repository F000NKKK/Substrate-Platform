import type { PanelPlacement } from "../types";

export interface PersistedShellLayoutState {
  placements?: Record<string, PanelPlacement>;
  sizes?: Record<string, number>;
  centerActiveId?: string;
}
