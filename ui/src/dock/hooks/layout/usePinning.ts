// Pinning and unpinning a tool window, and toggling it.

import { useCallback } from "react";
import { closeOtherFlyouts } from "./placements";
import type { LayoutState } from "./useLayoutState";

/** Pinning and unpinning a tool window, and toggling it. */
export function usePinning(layoutState: LayoutState) {
  const { setPlacements } = layoutState;
  const toggle = useCallback((id: string) => {
    setPlacements((prev) => {
      const cur = prev[id];
      if (!cur || cur.anchor === "float" || cur.anchor === "center") return prev;
      if (cur.mode !== "hidden") return { ...prev, [id]: { anchor: cur.anchor, mode: "hidden" } };
      const next = closeOtherFlyouts(prev, cur.anchor, id);
      next[id] = { anchor: cur.anchor, mode: "flyout" };
      return next;
    });
  }, []);

  const pin = useCallback((id: string) => {
    setPlacements((prev) => {
      const cur = prev[id];
      if (!cur || cur.anchor === "float" || cur.anchor === "center") return prev;
      return { ...prev, [id]: { anchor: cur.anchor, mode: "pinned" } };
    });
  }, []);

  const unpin = useCallback((id: string) => {
    setPlacements((prev) => {
      const cur = prev[id];
      if (!cur || cur.anchor === "float" || cur.anchor === "center") return prev;
      return { ...prev, [id]: { anchor: cur.anchor, mode: "flyout" } };
    });
  }, []);

  return { toggle, pin, unpin };
}

/** What `usePinning` gives. */
export type Pinning = ReturnType<typeof usePinning>;
