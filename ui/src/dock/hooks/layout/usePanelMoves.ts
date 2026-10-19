// Docking a panel somewhere, floating it, moving and resizing a float — and saving the layout.

import { useCallback, useEffect } from "react";
import type { DockAnchor } from "../../types";
import type { PersistedShellLayoutState } from "../shellLayoutPersistence";
import { DEFAULT_FLOAT_SIZE } from "./placements";
import type { ShellLayoutOptions } from "./shellLayoutTypes";
import type { AnchorQueries } from "./useAnchorQueries";
import type { LayoutState } from "./useLayoutState";
import type { PanelIndex } from "./usePanelIndex";

/** Docking a panel somewhere, floating it, moving and resizing a float — and saving the layout. */
export function usePanelMoves(options: ShellLayoutOptions, panelIndex: PanelIndex, layoutState: LayoutState, anchorQueries: AnchorQueries) {
  const { main } = options;
  const { panelsById } = panelIndex;
  const { store, storeKey, placements, setPlacements, centerActiveId, setCenterActive, sizes, skipSaveRef } = layoutState;
  const { successor } = anchorQueries;
  const dockTo = useCallback(
    (id: string, anchor: DockAnchor) => {
      if (anchor === "center") {
        setPlacements((prev) => ({ ...prev, [id]: { anchor: "center", mode: "pinned" } }));
        setCenterActive(id);
        return;
      }
      // Redocking joins whatever's already pinned there side by side — it
      // doesn't evict them, same as pin() above.
      setPlacements((prev) => ({ ...prev, [id]: { anchor, mode: "pinned" } }));
      // This panel just left the center dock — if it was the active center
      // tab, CenterDock would otherwise keep rendering it there (it looks up
      // `panelsById[centerActiveId]` directly, not filtered by `centerIds`),
      // showing the same panel both at its new anchor and, stale, in the
      // center at once. The next tab takes over (`successor`), not the
      // empty main view.
      setCenterActive((cur) => (cur === id ? successor([id]) : cur));
    },
    [main.id]
  );

  const floatAt = useCallback(
    (id: string, x: number, y: number) => {
      setPlacements((prev) => {
        const cur = prev[id];
        const size = cur?.anchor === "float" ? { w: cur.w, h: cur.h } : (panelsById[id]?.floatSize ?? DEFAULT_FLOAT_SIZE);
        return { ...prev, [id]: { anchor: "float", x, y, ...size } };
      });
      setCenterActive((activeCur) => (activeCur === id ? successor([id]) : activeCur));
    },
    [main.id, panelsById]
  );

  const moveFloat = useCallback((id: string, x: number, y: number) => {
    setPlacements((prev) => {
      const cur = prev[id];
      if (cur?.anchor !== "float" || (cur.x === x && cur.y === y)) return prev;
      return { ...prev, [id]: { ...cur, x, y } };
    });
  }, []);

  const resizeFloat = useCallback((id: string, w: number, h: number) => {
    setPlacements((prev) => {
      const cur = prev[id];
      if (cur?.anchor !== "float" || (cur.w === w && cur.h === h)) return prev;
      return { ...prev, [id]: { ...cur, w, h } };
    });
  }, []);

  useEffect(() => {
    if (!storeKey) return;
    // The render that switched stores still holds the previous scope's
    // layout — saving it would overwrite the new scope's before it's applied.
    if (skipSaveRef.current) {
      skipSaveRef.current = false;
      return;
    }
    store.set(storeKey, { placements, sizes, centerActiveId } satisfies PersistedShellLayoutState);
  }, [store, storeKey, placements, sizes, centerActiveId]);

  return { dockTo, floatAt, moveFloat, resizeFloat };
}

/** What `usePanelMoves` gives. */
export type PanelMoves = ReturnType<typeof usePanelMoves>;
