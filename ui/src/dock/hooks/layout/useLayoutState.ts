// The placements, sizes and active center tab — remembered in the state store, re-applied when its scope changes.

import { useEffect, useMemo, useRef, useState } from "react";
import { useStateStore } from "../../../infra/state";
import { type PanelPlacement, TOOL_WINDOW_ANCHORS } from "../../types";
import type { PersistedShellLayoutState } from "../shellLayoutPersistence";
import { initialFloat, isSanePlacement } from "./placements";
import type { ShellLayoutOptions } from "./shellLayoutTypes";
import type { PanelIndex } from "./usePanelIndex";

/** The placements, sizes and active center tab — remembered in the state store, re-applied when its scope changes. */
export function useLayoutState(options: ShellLayoutOptions, panelIndex: PanelIndex) {
  const { main, toolWindows, defaultPinned, persistKey } = options;
  const { panelsById } = panelIndex;
  const store = useStateStore();
  const storeKey = persistKey ? `shell.layout:${persistKey}` : null;
  const readPersisted = (): PersistedShellLayoutState | undefined =>
    storeKey ? (store.get(storeKey) as PersistedShellLayoutState | undefined) : undefined;
  const persisted = useMemo(readPersisted, [store, storeKey]);

  /** Every declared tool window at its default placement, overridden by whatever `from` restores. */
  function restoredPlacements(from: PersistedShellLayoutState | undefined): Record<string, PanelPlacement> {
    const initial: Record<string, PanelPlacement> = { [main.id]: { anchor: "center", mode: "pinned" } };
    for (const anchor of TOOL_WINDOW_ANCHORS) {
      for (const panel of toolWindows[anchor] ?? []) {
        initial[panel.id] = panel.floatSize
          ? { anchor, mode: "closed", reopen: initialFloat(panel.floatSize) }
          : { anchor, mode: defaultPinned?.[anchor] === panel.id ? "pinned" : "hidden" };
      }
    }
    // The center's original tab is always center/pinned (see `close` above) —
    // every other panel's placement can be restored from last session.
    const saved = from?.placements;
    if (saved && typeof saved === "object") {
      for (const id of Object.keys(initial)) {
        if (id !== main.id && isSanePlacement(saved[id])) initial[id] = saved[id];
      }
    }
    return initial;
  }

  function restoredSizes(from: PersistedShellLayoutState | undefined): Record<string, number> {
    const saved = from?.sizes && typeof from.sizes === "object" ? from.sizes : {};
    return Object.fromEntries(Object.entries(saved).filter(([, n]) => typeof n === "number" && Number.isFinite(n)));
  }

  const [placements, setPlacements] = useState<Record<string, PanelPlacement>>(() => restoredPlacements(persisted));

  // Read by `show`/`hide`, which branch on the current placement before
  // deciding what to set — kept out of a state updater so `close` (which
  // has side effects of its own) is never called from inside one.
  const placementsRef = useRef(placements);
  placementsRef.current = placements;

  const [centerActiveId, setCenterActive] = useState(() => {
    const restored = persisted?.centerActiveId;
    return restored && (restored === main.id || panelsById[restored]) ? restored : main.id;
  });

  const [sizes, setSizes] = useState<Record<string, number>>(() => restoredSizes(persisted));

  // The store's scope changed (another project opened): apply its saved
  // layout in place — the shell stays mounted, so panels (and their state,
  // e.g. live terminals) survive the switch. Dynamic center panels keep
  // their placements; the product re-adds/removes those itself.
  const appliedRef = useRef(persisted);
  const skipSaveRef = useRef(false);
  useEffect(() => {
    if (appliedRef.current === persisted) return;
    appliedRef.current = persisted;
    skipSaveRef.current = true;
    const restored = restoredPlacements(persisted);
    setPlacements((prev) => {
      const next = { ...prev };
      for (const [id, placement] of Object.entries(restored)) next[id] = placement;
      return next;
    });
    setSizes(restoredSizes(persisted));
    const active = persisted?.centerActiveId;
    if (active && panelsById[active]) setCenterActive(active);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [persisted]);

  return { store, storeKey, readPersisted, persisted, restoredPlacements, restoredSizes, placements, setPlacements, placementsRef, centerActiveId, setCenterActive, sizes, setSizes, appliedRef, skipSaveRef };
}

/** What `useLayoutState` gives. */
export type LayoutState = ReturnType<typeof useLayoutState>;
