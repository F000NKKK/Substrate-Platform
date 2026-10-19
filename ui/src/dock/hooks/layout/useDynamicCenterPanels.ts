// Placements kept in step with the product's dynamic center panels.

import { useEffect, useRef } from "react";
import { NO_PANELS } from "../shellLayoutDefaults";
import type { ShellLayoutOptions } from "./shellLayoutTypes";
import type { AnchorQueries } from "./useAnchorQueries";
import type { LayoutState } from "./useLayoutState";

/** Placements kept in step with the product's dynamic center panels. */
export function useDynamicCenterPanels(options: ShellLayoutOptions, layoutState: LayoutState, anchorQueries: AnchorQueries) {
  const { main, extraCenterPanels = NO_PANELS } = options;
  const { setPlacements, setCenterActive } = layoutState;
  const { successor } = anchorQueries;
  // Keeps placements/the active center tab in sync with `extraCenterPanels`
  // as the product adds/removes entries — newly-added panels get pinned to
  // center and become active; removed ones lose their placement entirely
  // (not just hidden, since they no longer exist to reopen).
  const prevExtraIdsRef = useRef<Set<string>>(new Set());
  useEffect(() => {
    const currentIds = new Set(extraCenterPanels.map((p) => p.id));
    const prevIds = prevExtraIdsRef.current;
    const added = extraCenterPanels.filter((p) => !prevIds.has(p.id));
    const removed = [...prevIds].filter((id) => !currentIds.has(id));

    if (added.length > 0 || removed.length > 0) {
      setPlacements((prev) => {
        const next = { ...prev };
        for (const panel of added) next[panel.id] = { anchor: "center", mode: "pinned" };
        for (const id of removed) delete next[id];
        return next;
      });
    }
    if (added.length > 0) setCenterActive(added[added.length - 1].id);
    if (removed.length > 0) setCenterActive((cur) => (removed.includes(cur) ? successor(removed) : cur));

    prevExtraIdsRef.current = currentIds;
  }, [extraCenterPanels, main.id]);
}

