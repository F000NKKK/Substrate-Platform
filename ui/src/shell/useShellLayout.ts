import { useCallback, useMemo, useState } from "react";
import type { PanelDef, PanelPlacement, ToolWindowAnchor } from "./types";

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
  homeAnchor: Record<string, ToolWindowAnchor>;
  idsByAnchor: (anchor: ToolWindowAnchor) => string[];
  activeInAnchor: (anchor: ToolWindowAnchor) => string | null;
  floatingIds: string[];
  placements: Record<string, PanelPlacement>;
  toggle: (id: string) => void;
  pin: (id: string) => void;
  unpin: (id: string) => void;
  close: (id: string) => void;
  dockTo: (id: string, anchor: ToolWindowAnchor) => void;
  floatAt: (id: string, x: number, y: number) => void;
  moveFloat: (id: string, x: number, y: number) => void;
}

/**
 * Owns where every tool window currently lives — this is the one thing that
 * changes as panels get dragged between docks or pulled out into a floating
 * panel, so it's centralized here rather than split per-anchor.
 */
export function useShellLayout(
  toolWindows: Partial<Record<ToolWindowAnchor, PanelDef[]>>,
  defaultPinned?: Partial<Record<ToolWindowAnchor, string>>
): ShellLayout {
  const panelsById = useMemo(() => {
    const map: Record<string, PanelDef> = {};
    for (const anchor of ANCHORS) {
      for (const panel of toolWindows[anchor] ?? []) map[panel.id] = panel;
    }
    return map;
  }, [toolWindows]);

  const homeAnchor = useMemo(() => {
    const map: Record<string, ToolWindowAnchor> = {};
    for (const anchor of ANCHORS) {
      for (const panel of toolWindows[anchor] ?? []) map[panel.id] = anchor;
    }
    return map;
  }, [toolWindows]);

  const [placements, setPlacements] = useState<Record<string, PanelPlacement>>(() => {
    const initial: Record<string, PanelPlacement> = {};
    for (const anchor of ANCHORS) {
      for (const panel of toolWindows[anchor] ?? []) {
        initial[panel.id] = { anchor, mode: defaultPinned?.[anchor] === panel.id ? "pinned" : "hidden" };
      }
    }
    return initial;
  });

  const idsByAnchor = useCallback(
    (anchor: ToolWindowAnchor) => Object.entries(placements).filter(([, p]) => p.anchor === anchor).map(([id]) => id),
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
      if (!cur || cur.anchor === "float") return prev;
      if (cur.mode !== "hidden") return { ...prev, [id]: { anchor: cur.anchor, mode: "hidden" } };
      const next = closeOthersOnAnchor(prev, cur.anchor, id);
      next[id] = { anchor: cur.anchor, mode: "flyout" };
      return next;
    });
  }, []);

  const pin = useCallback((id: string) => {
    setPlacements((prev) => {
      const cur = prev[id];
      if (!cur || cur.anchor === "float") return prev;
      return { ...prev, [id]: { anchor: cur.anchor, mode: "pinned" } };
    });
  }, []);

  const unpin = useCallback((id: string) => {
    setPlacements((prev) => {
      const cur = prev[id];
      if (!cur || cur.anchor === "float") return prev;
      return { ...prev, [id]: { anchor: cur.anchor, mode: "flyout" } };
    });
  }, []);

  const close = useCallback(
    (id: string) => {
      setPlacements((prev) => ({ ...prev, [id]: { anchor: homeAnchor[id] ?? "bottom", mode: "hidden" } }));
    },
    [homeAnchor]
  );

  const dockTo = useCallback((id: string, anchor: ToolWindowAnchor) => {
    setPlacements((prev) => {
      const next = closeOthersOnAnchor(prev, anchor, id);
      next[id] = { anchor, mode: "pinned" };
      return next;
    });
  }, []);

  const floatAt = useCallback((id: string, x: number, y: number) => {
    setPlacements((prev) => {
      const cur = prev[id];
      const size = cur?.anchor === "float" ? { w: cur.w, h: cur.h } : DEFAULT_FLOAT_SIZE;
      return { ...prev, [id]: { anchor: "float", x, y, ...size } };
    });
  }, []);

  return {
    panelsById,
    homeAnchor,
    idsByAnchor,
    activeInAnchor,
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
