// Closing, showing and hiding panels.

import { useCallback } from "react";
import type { PanelPlacement, ToolWindowAnchor } from "../../types";
import { closeOtherFlyouts } from "./placements";
import type { ShellLayoutOptions } from "./shellLayoutTypes";
import type { AnchorQueries } from "./useAnchorQueries";
import type { LayoutState } from "./useLayoutState";
import type { PanelIndex } from "./usePanelIndex";

/** Closing, showing and hiding panels. */
export function usePanelVisibility(options: ShellLayoutOptions, panelIndex: PanelIndex, layoutState: LayoutState, anchorQueries: AnchorQueries) {
  const { main, onCloseDynamicPanel } = options;
  const { panelsById, homeAnchor } = panelIndex;
  const { placements, setPlacements, placementsRef, setCenterActive } = layoutState;
  const { successor } = anchorQueries;
  const close = useCallback(
    (id: string) => {
      if (id === main.id) return; // the center's original tab is never closable
      if (!(id in homeAnchor)) {
        // A dynamically-registered center panel (e.g. an open file tab) has
        // no toolWindow "home" to hide it to — the product owns removing it
        // from `extraCenterPanels`, which the sync effect below then reflects.
        onCloseDynamicPanel?.(id);
        return;
      }
      const home = homeAnchor[id] ?? "bottom";
      // Like Visual Studio, × takes a tool window off screen entirely —
      // strip tab included — remembering where it was for View to bring it back.
      setPlacements((prev) => {
        const cur = prev[id];
        let closed: PanelPlacement = { anchor: home, mode: "closed" };
        if (cur?.anchor === "float") closed = { anchor: home, mode: "closed", reopen: { ...cur } };
        else if (cur?.anchor === "center") closed = { anchor: home, mode: "closed", reopen: { anchor: "center" } };
        else if (cur?.mode === "closed") closed = cur;
        else if (cur?.mode === "flyout" || cur?.mode === "hidden") closed = { anchor: cur.anchor, mode: "closed", reopen: { anchor: "flyout" } };
        else if (cur) closed = { anchor: cur.anchor, mode: "closed" };
        return { ...prev, [id]: closed };
      });
      setCenterActive((cur) => (cur === id ? successor([id]) : cur));
    },
    [homeAnchor, main.id, onCloseDynamicPanel]
  );

  const show = useCallback((id: string) => {
    const cur = placementsRef.current[id];
    if (!cur || cur.anchor === "float") return;
    if (cur.anchor === "center") {
      setCenterActive(id);
      return;
    }
    if (cur.mode === "closed" && cur.reopen?.anchor === "center") {
      setPlacements((prev) => ({ ...prev, [id]: { anchor: "center", mode: "pinned" } }));
      setCenterActive(id);
    } else if (cur.mode === "closed" && cur.reopen?.anchor === "float") {
      const { x, y, w, h } = cur.reopen;
      setPlacements((prev) => ({ ...prev, [id]: { anchor: "float", x, y, w, h } }));
    } else if (cur.mode === "closed" && cur.reopen?.anchor === "flyout") {
      setPlacements((prev) => {
        const next = closeOtherFlyouts(prev, cur.anchor as ToolWindowAnchor, id);
        next[id] = { anchor: cur.anchor, mode: "flyout" };
        return next;
      });
    } else if (cur.mode === "hidden" || cur.mode === "closed") {
      setPlacements((prev) => ({ ...prev, [id]: { anchor: cur.anchor, mode: "pinned" } }));
    }
  }, []);

  const hide = useCallback(
    (id: string) => {
      const cur = placementsRef.current[id];
      if (!cur) return;
      if (cur.anchor === "float" || cur.anchor === "center" || panelsById[id]?.floatSize) {
        close(id);
        return;
      }
      setPlacements((prev) => ({ ...prev, [id]: { anchor: cur.anchor, mode: "hidden" } }));
    },
    [close, panelsById]
  );

  const isVisible = useCallback(
    (id: string) => {
      const p = placements[id];
      return !!p && (p.anchor === "float" || p.anchor === "center" || (p.mode !== "hidden" && p.mode !== "closed"));
    },
    [placements]
  );

  return { close, show, hide, isVisible };
}

/** What `usePanelVisibility` gives. */
export type PanelVisibility = ReturnType<typeof usePanelVisibility>;
