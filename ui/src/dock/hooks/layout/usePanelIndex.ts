// Every panel by id, and the edge each tool window belongs to.

import { useMemo } from "react";
import { type DockAnchor, type PanelDef, TOOL_WINDOW_ANCHORS } from "../../types";
import { NO_PANELS } from "../shellLayoutDefaults";
import type { ShellLayoutOptions } from "./shellLayoutTypes";

/** Every panel by id, and the edge each tool window belongs to. */
export function usePanelIndex(options: ShellLayoutOptions) {
  const { main, toolWindows, extraCenterPanels = NO_PANELS } = options;
  const panelsById = useMemo(() => {
    const map: Record<string, PanelDef> = { [main.id]: main };
    for (const anchor of TOOL_WINDOW_ANCHORS) {
      for (const panel of toolWindows[anchor] ?? []) map[panel.id] = panel;
    }
    for (const panel of extraCenterPanels) map[panel.id] = panel;
    return map;
  }, [main, toolWindows, extraCenterPanels]);

  const homeAnchor = useMemo(() => {
    const map: Record<string, DockAnchor> = { [main.id]: "center" };
    for (const anchor of TOOL_WINDOW_ANCHORS) {
      for (const panel of toolWindows[anchor] ?? []) map[panel.id] = anchor;
    }
    return map;
  }, [main, toolWindows]);

  return { panelsById, homeAnchor };
}

/** What `usePanelIndex` gives. */
export type PanelIndex = ReturnType<typeof usePanelIndex>;
