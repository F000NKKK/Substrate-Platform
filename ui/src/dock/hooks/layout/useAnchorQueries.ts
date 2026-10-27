// What's where: sizes per anchor, panels per anchor, the center's tabs, what's pinned, flying out, floating.

import { useCallback, useMemo, useRef } from "react";
import type { ToolWindowAnchor } from "../../types";
import { DEFAULT_ANCHOR_SIZE, MAX_ANCHOR_SIZE, MIN_ANCHOR_SIZE, sizeKey, type SizeMode } from "./placements";
import type { ShellLayoutOptions } from "./shellLayoutTypes";
import type { LayoutState } from "./useLayoutState";
import type { PanelIndex } from "./usePanelIndex";

/** What's where: sizes per anchor, panels per anchor, the center's tabs, what's pinned, flying out, floating. */
export function useAnchorQueries(options: ShellLayoutOptions, panelIndex: PanelIndex, layoutState: LayoutState) {
  const { main } = options;
  const { panelsById } = panelIndex;
  const { placements, sizes, setSizes } = layoutState;
  const anchorSize = useCallback(
    (panelId: string, anchor: ToolWindowAnchor, mode: SizeMode) => sizes[sizeKey(panelId, anchor, mode)] ?? DEFAULT_ANCHOR_SIZE[anchor],
    [sizes]
  );

  const setAnchorSize = useCallback((panelId: string, anchor: ToolWindowAnchor, mode: SizeMode, size: number) => {
    const clamped = Math.min(MAX_ANCHOR_SIZE, Math.max(MIN_ANCHOR_SIZE[anchor], size));
    const key = sizeKey(panelId, anchor, mode);
    setSizes((prev) => (prev[key] === clamped ? prev : { ...prev, [key]: clamped }));
  }, []);

  const idsByAnchor = useCallback(
    (anchor: ToolWindowAnchor) =>
      Object.entries(placements)
        .filter(([, p]) => p.anchor === anchor && p.mode !== "closed")
        .map(([id]) => id),
    [placements]
  );

  const centerIds = useMemo(
    () => Object.entries(placements).filter(([, p]) => p.anchor === "center").map(([id]) => id),
    [placements]
  );

  // Read when a tab leaves the center (from inside state updaters, so refs).
  const centerIdsRef = useRef(centerIds);
  centerIdsRef.current = centerIds;
  const panelsByIdRef = useRef(panelsById);
  panelsByIdRef.current = panelsById;

  /**
   * Which tab the center shows once `leaving` go (closed, docked
   * elsewhere, floated): the first tab still there, pinned ones first, as
   * the strip lists them — the main view only once no tab is left.
   */
  const successor = useCallback(
    (leaving: readonly string[]) => {
      const byId = panelsByIdRef.current;
      const shown = centerIdsRef.current.filter((id) => id !== main.id && !leaving.includes(id) && byId[id] && !byId[id].hidden);
      return shown.find((id) => byId[id].pinned) ?? shown[0] ?? main.id;
    },
    [main.id]
  );

  const pinnedInAnchor = useCallback(
    (anchor: ToolWindowAnchor) =>
      Object.entries(placements).filter(([, p]) => p.anchor === anchor && p.mode === "pinned").map(([id]) => id),
    [placements]
  );

  const flyoutInAnchor = useCallback(
    (anchor: ToolWindowAnchor) => {
      const entry = Object.entries(placements).find(([, p]) => p.anchor === anchor && p.mode === "flyout");
      return entry?.[0] ?? null;
    },
    [placements]
  );

  const floatingIds = useMemo(() => Object.entries(placements).filter(([, p]) => p.anchor === "float").map(([id]) => id), [placements]);

  return { anchorSize, setAnchorSize, idsByAnchor, centerIds, centerIdsRef, panelsByIdRef, successor, pinnedInAnchor, flyoutInAnchor, floatingIds };
}

/** What `useAnchorQueries` gives. */
export type AnchorQueries = ReturnType<typeof useAnchorQueries>;
