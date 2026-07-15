import { useCallback, useState } from "react";
import type { DockMode, DockState } from "./types";

export interface DockControls extends DockState {
  /** Tab click: opens as a flyout (or re-docks if already pinned) — closes if that tab is already active. */
  toggle: (panelId: string) => void;
  pin: () => void;
  unpin: () => void;
  close: () => void;
}

export function useDock(initialActiveId: string | null, initialMode: DockMode): DockControls {
  const [state, setState] = useState<DockState>({ activeId: initialActiveId, mode: initialMode });

  const toggle = useCallback((panelId: string) => {
    setState((prev) => {
      if (prev.activeId === panelId && prev.mode !== "hidden") {
        return { activeId: null, mode: "hidden" };
      }
      return { activeId: panelId, mode: "flyout" };
    });
  }, []);

  const pin = useCallback(() => {
    setState((prev) => ({ ...prev, mode: "pinned" }));
  }, []);

  const unpin = useCallback(() => {
    setState((prev) => ({ ...prev, mode: "flyout" }));
  }, []);

  const close = useCallback(() => {
    setState({ activeId: null, mode: "hidden" });
  }, []);

  return { ...state, toggle, pin, unpin, close };
}
