import { useEffect, useRef, useState, type PointerEvent as ReactPointerEvent } from "react";
import { PanelSurface } from "../PanelSurface/PanelSurface";
import type { DockAnchor, PanelDef } from "../types";

export interface FloatingPanelProps {
  panel: PanelDef;
  x: number;
  y: number;
  w: number;
  h: number;
  onDock: () => void;
  onClose: () => void;
  /** Called (debounced) with the new size after the user resizes the panel — lets the layout remember (and persist) it. */
  onResize?: (w: number, h: number) => void;
  /** The title bar was dragged somewhere that isn't a dock guide — the panel's new spot. */
  onMove?: (x: number, y: number) => void;
  /** The title bar was dropped on one of the dock guides. */
  onDockTo?: (anchor: DockAnchor) => void;
}

/** Waits for a resize drag to settle before reporting it, rather than once per frame. */
const RESIZE_SETTLE_MS = 150;
/** The dock guide buttons' size and spacing (px). */
const GUIDE_SIZE = 32;
const GUIDE_STEP = 38;
/** How much of the title bar always stays on screen. */
const KEEP_VISIBLE = 80;

const GUIDE_OFFSETS: Record<DockAnchor, { dx: number; dy: number }> = {
  center: { dx: 0, dy: 0 },
  left: { dx: -GUIDE_STEP, dy: 0 },
  right: { dx: GUIDE_STEP, dy: 0 },
  bottom: { dx: 0, dy: GUIDE_STEP },
};

interface Drag {
  pointerX: number;
  pointerY: number;
  startX: number;
  startY: number;
  /** The main area, where the guides sit, in viewport coordinates. */
  main: DOMRect | null;
}

function guideAt(drag: Drag, clientX: number, clientY: number): DockAnchor | null {
  if (!drag.main) return null;
  const cx = drag.main.left + drag.main.width / 2;
  const cy = drag.main.top + drag.main.height / 2;
  for (const [anchor, { dx, dy }] of Object.entries(GUIDE_OFFSETS) as [DockAnchor, { dx: number; dy: number }][]) {
    if (Math.abs(clientX - (cx + dx)) <= GUIDE_SIZE / 2 && Math.abs(clientY - (cy + dy)) <= GUIDE_SIZE / 2) return anchor;
  }
  return null;
}

/** The part of the main area a guide would dock into, for the preview under it. */
function previewRect(main: DOMRect, anchor: DockAnchor) {
  const edge = 0.3;
  if (anchor === "left") return { left: main.left, top: main.top, width: main.width * edge, height: main.height };
  if (anchor === "right") return { left: main.right - main.width * edge, top: main.top, width: main.width * edge, height: main.height };
  if (anchor === "bottom") return { left: main.left, top: main.bottom - main.height * edge, width: main.width, height: main.height * edge };
  return { left: main.left, top: main.top, width: main.width, height: main.height };
}

/**
 * A panel pulled out of every dock — a free-floating window inside the
 * shell, like a Visual Studio floating tool window. Drag its title bar to
 * move it; while dragging, dock guides appear over the main area — drop on
 * one to dock it to that edge or as a tab in the center. Its pin button
 * sends it back to its home dock, and its bottom-right corner resizes it
 * (plain CSS `resize`; the settled size is reported via `onResize`).
 */
export function FloatingPanel({ panel, x, y, w, h, onDock, onClose, onResize, onMove, onDockTo }: FloatingPanelProps) {
  const Component = panel.component;
  const ref = useRef<HTMLDivElement>(null);
  const onResizeRef = useRef(onResize);
  onResizeRef.current = onResize;
  const sizeRef = useRef({ w, h });
  sizeRef.current = { w, h };
  const dragRef = useRef<Drag | null>(null);
  const [dragPos, setDragPos] = useState<{ x: number; y: number; main: DOMRect | null } | null>(null);
  const [hoverGuide, setHoverGuide] = useState<DockAnchor | null>(null);

  useEffect(() => {
    const el = ref.current;
    if (!el || typeof ResizeObserver === "undefined") return;
    let timer: ReturnType<typeof setTimeout> | undefined;
    const observer = new ResizeObserver(() => {
      clearTimeout(timer);
      timer = setTimeout(() => {
        const nextW = Math.round(el.offsetWidth);
        const nextH = Math.round(el.offsetHeight);
        if (nextW !== sizeRef.current.w || nextH !== sizeRef.current.h) onResizeRef.current?.(nextW, nextH);
      }, RESIZE_SETTLE_MS);
    });
    observer.observe(el);
    return () => {
      clearTimeout(timer);
      observer.disconnect();
    };
  }, []);

  function handleHeaderPointerDown(e: ReactPointerEvent<HTMLDivElement>) {
    // Clicks on the header's own buttons stay clicks.
    if (e.button !== 0 || (e.target as HTMLElement).closest("button, input, [role='button']")) return;
    const el = ref.current;
    if (!el) return;
    e.preventDefault();
    dragRef.current = {
      pointerX: e.clientX,
      pointerY: e.clientY,
      startX: x,
      startY: y,
      main: el.closest(".sp-shell")?.querySelector(".sp-shell-main")?.getBoundingClientRect() ?? null,
    };

    function onPointerMove(ev: PointerEvent) {
      const drag = dragRef.current;
      if (!drag) return;
      setDragPos({ x: drag.startX + ev.clientX - drag.pointerX, y: drag.startY + ev.clientY - drag.pointerY, main: drag.main });
      setHoverGuide(guideAt(drag, ev.clientX, ev.clientY));
    }
    function onPointerUp(ev: PointerEvent) {
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp);
      window.removeEventListener("pointercancel", onPointerUp);
      const drag = dragRef.current;
      dragRef.current = null;
      setDragPos(null);
      setHoverGuide(null);
      if (!drag || ev.type === "pointercancel") return;
      const guide = guideAt(drag, ev.clientX, ev.clientY);
      if (guide) onDockTo?.(guide);
      else onMove?.(drag.startX + ev.clientX - drag.pointerX, drag.startY + ev.clientY - drag.pointerY);
    }
    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", onPointerUp);
    window.addEventListener("pointercancel", onPointerUp);
  }

  const pos = dragPos ?? { x, y };
  // However far it was dragged (or the window shrank), the title bar stays reachable.
  const parentW = ref.current?.offsetParent?.clientWidth ?? window.innerWidth;
  const parentH = ref.current?.offsetParent?.clientHeight ?? window.innerHeight;
  const left = Math.min(Math.max(pos.x, KEEP_VISIBLE - w), parentW - KEEP_VISIBLE);
  const top = Math.min(Math.max(pos.y, 0), parentH - 34);
  const main = dragPos?.main;

  return (
    <>
      <div
        ref={ref}
        className="sp-floating-panel"
        data-dragging={dragPos ? true : undefined}
        style={{ left, top, width: w, height: h }}
      >
        <PanelSurface
          panelId={panel.id}
          title={panel.title}
          pinned={false}
          onTogglePin={onDock}
          onClose={onClose}
          onHeaderPointerDown={onDockTo || onMove ? handleHeaderPointerDown : undefined}
        >
          <Component />
        </PanelSurface>
      </div>
      {dragPos && main && (
        <>
          {hoverGuide && <div className="sp-dock-preview" style={previewRect(main, hoverGuide)} />}
          <div className="sp-dock-guides" style={{ left: main.left + main.width / 2, top: main.top + main.height / 2 }}>
            {(Object.keys(GUIDE_OFFSETS) as DockAnchor[]).map((anchor) => (
              <div
                key={anchor}
                className={`sp-dock-guide-button sp-dock-guide-button--${anchor}`}
                data-hover={hoverGuide === anchor || undefined}
                style={{
                  width: GUIDE_SIZE,
                  height: GUIDE_SIZE,
                  left: GUIDE_OFFSETS[anchor].dx - GUIDE_SIZE / 2,
                  top: GUIDE_OFFSETS[anchor].dy - GUIDE_SIZE / 2,
                }}
              />
            ))}
          </div>
        </>
      )}
    </>
  );
}
