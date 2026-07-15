import { useCallback, useMemo, useState } from "react";
import type { DockAnchor, PanelDef, PanelPlacement, ToolWindowAnchor } from "./types";

const ANCHORS: readonly ToolWindowAnchor[] = ["left", "right", "bottom"];
const DEFAULT_FLOAT_SIZE = { w: 340, h: 280 };

function closeOthersOnAnchor(
  placements: Record<string, PanelPlacement>,
  anchor: ToolWindowAnchor,
  exceptId: string
): Record<string, PanelPlacement> {
  const next = { ...placements };
  for (const [id, p] of Object.entries(placements)) {
    if (id !== exceptId && p.anchor === anchor && p.mode !== "hidden") {
      next[id] = { anchor, mode: "hidden" };
    }
  }
  return next;
}

export interface ShellLayout {
  panelsById: Record<string, PanelDef>;
  mainId: string;
  homeAnchor: Record<string, DockAnchor>;
  idsByAnchor: (anchor: ToolWindowAnchor) => string[];
  activeInAnchor: (anchor: ToolWindowAnchor) => string | null;
  centerIds: string[];
  centerActiveId: string;
  setCenterActive: (id: string) => void;
  floatingIds: string[];
  placements: Record<string, PanelPlacement>;
  toggle: (id: string) => void;
  pin: (id: string) => void;
  unpin: (id: string) => void;
  close: (id: string) => void;
  dockTo: (id: string, anchor: DockAnchor) => void;
  floatAt: (id: string, x: number, y: number) => void;
}

/**
 * Owns where every panel currently lives — tool windows move between edges,
 * the center is a normal tabbed dock (more than one panel can sit there at
 * once), and anything can be pulled out into a floating panel. Centralized
 * here rather than split per-anchor since panels move between all of these.
 */
export function useShellLayout(
  main: PanelDef,
  toolWindows: Partial<Record<ToolWindowAnchor, PanelDef[]>>,
  defaultPinned?: Partial<Record<ToolWindowAnchor, string>>
): ShellLayout {
  const panelsById = useMemo(() => {
    const map: Record<string, PanelDef> = { [main.id]: main };
    for (const anchor of ANCHORS) {
      for (const panel of toolWindows[anchor] ?? []) map[panel.id] = panel;
    }
    return map;
  }, [main, toolWindows]);

  const homeAnchor = useMemo(() => {
    const map: Record<string, DockAnchor> = { [main.id]: "center" };
    for (const anchor of ANCHORS) {
      for (const panel of toolWindows[anchor] ?? []) map[panel.id] = anchor;
    }
    return map;
  }, [main, toolWindows]);

  const [placements, setPlacements] = useState<Record<string, PanelPlacement>>(() => {
    const initial: Record<string, PanelPlacement> = { [main.id]: { anchor: "center", mode: "pinned" } };
    for (const anchor of ANCHORS) {
      for (const panel of toolWindows[anchor] ?? []) {
        initial[panel.id] = { anchor, mode: defaultPinned?.[anchor] === panel.id ? "pinned" : "hidden" };
      }
    }
    return initial;
  });

  const [centerActiveId, setCenterActive] = useState(main.id);

  const idsByAnchor = useCallback(
    (anchor: ToolWindowAnchor) => Object.entries(placements).filter(([, p]) => p.anchor === anchor).map(([id]) => id),
    [placements]
  );

  const centerIds = useMemo(
    () => Object.entries(placements).filter(([, p]) => p.anchor === "center").map(([id]) => id),
    [placements]
  );

  const activeInAnchor = useCallback(
    (anchor: ToolWindowAnchor) => {
      const entry = Object.entries(placements).find(([, p]) => p.anchor === anchor && p.mode !== "hidden");
      return entry?.[0] ?? null;
    },
    [placements]
  );

  const floatingIds = useMemo(() => Object.entries(placements).filter(([, p]) => p.anchor === "float").map(([id]) => id), [placements]);

  const toggle = useCallback((id: string) => {
    setPlacements((prev) => {
      const cur = prev[id];
      if (!cur || cur.anchor === "float" || cur.anchor === "center") return prev;
      if (cur.mode !== "hidden") return { ...prev, [id]: { anchor: cur.anchor, mode: "hidden" } };
      const next = closeOthersOnAnchor(prev, cur.anchor, id);
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

  const close = useCallback(
    (id: string) => {
      if (id === main.id) return; // the center's original tab is never closable
      setPlacements((prev) => ({ ...prev, [id]: { anchor: homeAnchor[id] ?? "bottom", mode: "hidden" } }));
      setCenterActive((cur) => (cur === id ? main.id : cur));
    },
    [homeAnchor, main.id]
  );

  const dockTo = useCallback(
    (id: string, anchor: DockAnchor) => {
      if (anchor === "center") {
        setPlacements((prev) => ({ ...prev, [id]: { anchor: "center", mode: "pinned" } }));
        setCenterActive(id);
        return;
      }
      setPlacements((prev) => {
        const next = closeOthersOnAnchor(prev, anchor, id);
        next[id] = { anchor, mode: "pinned" };
        return next;
      });
    },
    []
  );

  const floatAt = useCallback(
    (id: string, x: number, y: number) => {
      setPlacements((prev) => {
        const cur = prev[id];
        const size = cur?.anchor === "float" ? { w: cur.w, h: cur.h } : DEFAULT_FLOAT_SIZE;
        return { ...prev, [id]: { anchor: "float", x, y, ...size } };
      });
      setCenterActive((activeCur) => (activeCur === id ? main.id : activeCur));
    },
    [main.id]
  );

  return {
    panelsById,
    mainId: main.id,
    homeAnchor,
    idsByAnchor,
    activeInAnchor,
    centerIds,
    centerActiveId,
    setCenterActive,
    floatingIds,
    placements,
    toggle,
    pin,
    unpin,
    close,
    dockTo,
    floatAt,
  };
}
