import { useCallback, useMemo, useState } from "react";
import type { DockAnchor, PanelDef, PanelPlacement, ToolWindowAnchor } from "../types";

const ANCHORS: readonly ToolWindowAnchor[] = ["left", "right", "bottom"];
const DEFAULT_FLOAT_SIZE = { w: 340, h: 280 };

/** Matches the `--sp-toolwindow-size`/`--sp-toolwindow-bottom-size` token defaults — the starting size before a user drags a dock's resize handle. */
const DEFAULT_ANCHOR_SIZE: Record<ToolWindowAnchor, number> = { left: 260, right: 260, bottom: 220 };
const MIN_ANCHOR_SIZE = 120;
const MAX_ANCHOR_SIZE = 900;

/** Pinned and flyout are sized independently — a pinned panel holds permanent layout space so it's often sized differently than the same panel peeking open as a temporary overlay. */
type SizeMode = "pinned" | "flyout";

function sizeKey(panelId: string, anchor: ToolWindowAnchor, mode: SizeMode): string {
  return `${panelId}:${anchor}:${mode}`;
}

/** Only one flyout may peek open per anchor at a time — pinned panels are untouched by this (several can be pinned side by side). */
function closeOtherFlyouts(
  placements: Record<string, PanelPlacement>,
  anchor: ToolWindowAnchor,
  exceptId: string
): Record<string, PanelPlacement> {
  const next = { ...placements };
  for (const [id, p] of Object.entries(placements)) {
    if (id !== exceptId && p.anchor === anchor && p.mode === "flyout") {
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
  /** All simultaneously pinned panels on this anchor, in the order they were pinned — each gets its own grid slot, side by side. */
  pinnedInAnchor: (anchor: ToolWindowAnchor) => string[];
  /** The one panel currently peeking open as a flyout on this anchor, if any. */
  flyoutInAnchor: (anchor: ToolWindowAnchor) => string | null;
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
  /**
   * A panel's own remembered width (left/right) or height (bottom) — tracked
   * per panel, per anchor, AND per pinned/flyout mode (6 independent slots
   * total across the 3 anchors, since a panel keeps a separate size for each
   * one it's visited, in each mode). Falls back to the anchor's default until
   * that specific combination has been resized at least once.
   */
  anchorSize: (panelId: string, anchor: ToolWindowAnchor, mode: SizeMode) => number;
  setAnchorSize: (panelId: string, anchor: ToolWindowAnchor, mode: SizeMode, size: number) => void;
}

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

  const [sizes, setSizes] = useState<Record<string, number>>({});

  const anchorSize = useCallback(
    (panelId: string, anchor: ToolWindowAnchor, mode: SizeMode) => sizes[sizeKey(panelId, anchor, mode)] ?? DEFAULT_ANCHOR_SIZE[anchor],
    [sizes]
  );

  const setAnchorSize = useCallback((panelId: string, anchor: ToolWindowAnchor, mode: SizeMode, size: number) => {
    const clamped = Math.min(MAX_ANCHOR_SIZE, Math.max(MIN_ANCHOR_SIZE, size));
    const key = sizeKey(panelId, anchor, mode);
    setSizes((prev) => (prev[key] === clamped ? prev : { ...prev, [key]: clamped }));
  }, []);

  const idsByAnchor = useCallback(
    (anchor: ToolWindowAnchor) => Object.entries(placements).filter(([, p]) => p.anchor === anchor).map(([id]) => id),
    [placements]
  );

  const centerIds = useMemo(
    () => Object.entries(placements).filter(([, p]) => p.anchor === "center").map(([id]) => id),
    [placements]
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

  const close = useCallback(
    (id: string) => {
      if (id === main.id) return; // the center's original tab is never closable
      setPlacements((prev) => ({ ...prev, [id]: { anchor: homeAnchor[id] ?? "bottom", mode: "hidden" } }));
      setCenterActive((cur) => (cur === id ? main.id : cur));
    },
    [homeAnchor, main.id]
  );

  const dockTo = useCallback((id: string, anchor: DockAnchor) => {
    if (anchor === "center") {
      setPlacements((prev) => ({ ...prev, [id]: { anchor: "center", mode: "pinned" } }));
      setCenterActive(id);
      return;
    }
    // Redocking joins whatever's already pinned there side by side — it
    // doesn't evict them, same as pin() above.
    setPlacements((prev) => ({ ...prev, [id]: { anchor, mode: "pinned" } }));
  }, []);

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
    dockTo,
    floatAt,
    anchorSize,
    setAnchorSize,
  };
}
