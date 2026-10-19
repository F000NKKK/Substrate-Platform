import type { ShellLayout, ShellLayoutOptions } from "./layout/shellLayoutTypes";
import { usePanelIndex } from "./layout/usePanelIndex";
import { useLayoutState } from "./layout/useLayoutState";
import { useAnchorQueries } from "./layout/useAnchorQueries";
import { usePinning } from "./layout/usePinning";
import { usePanelVisibility } from "./layout/usePanelVisibility";
import { useDynamicCenterPanels } from "./layout/useDynamicCenterPanels";
import { usePanelMoves } from "./layout/usePanelMoves";

/**
 * Owns where every panel currently lives — tool windows move between edges,
 * the center is a normal tabbed dock (more than one panel can sit there at
 * once), and anything can be pulled out into a floating panel. Centralized
 * here rather than split per-anchor since panels move between all of these.
 *
 * Several panels can be pinned to the same anchor at once (they sit side by
 * side, like Visual Studio's own tool-window groups) — only the single
 * auto-hide flyout per anchor is exclusive, since only one can sensibly
 * peek open over the main area at a time.
 */
export function useShellLayout(options: ShellLayoutOptions): ShellLayout {
  const panelIndex = usePanelIndex(options);
  const layoutState = useLayoutState(options, panelIndex);
  const anchorQueries = useAnchorQueries(options, panelIndex, layoutState);
  const pinning = usePinning(layoutState);
  const panelVisibility = usePanelVisibility(options, panelIndex, layoutState, anchorQueries);
  useDynamicCenterPanels(options, layoutState, anchorQueries);
  const panelMoves = usePanelMoves(options, panelIndex, layoutState, anchorQueries);
  const { main } = options;
  const { panelsById, homeAnchor } = panelIndex;
  const { placements, centerActiveId, setCenterActive } = layoutState;
  const { anchorSize, setAnchorSize, idsByAnchor, centerIds, pinnedInAnchor, flyoutInAnchor, floatingIds } = anchorQueries;
  const { toggle, pin, unpin } = pinning;
  const { close, show, hide, isVisible } = panelVisibility;
  const { dockTo, floatAt, moveFloat, resizeFloat } = panelMoves;
  return {
    panelsById,
    mainId: main.id,
    homeAnchor,
    idsByAnchor,
    pinnedInAnchor,
    flyoutInAnchor,
    centerIds,
    centerActiveId,
    setCenterActive,
    floatingIds,
    placements,
    toggle,
    pin,
    unpin,
    close,
    show,
    hide,
    isVisible,
    dockTo,
    floatAt,
    moveFloat,
    resizeFloat,
    anchorSize,
    setAnchorSize,
  };
}

export type { ShellLayout, ShellLayoutOptions } from "./layout/shellLayoutTypes";
