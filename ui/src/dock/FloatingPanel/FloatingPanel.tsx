import { useEffect, useRef, useState, type PointerEvent as ReactPointerEvent } from "react";
import { PanelSurface } from "../PanelSurface/PanelSurface";
import type { DockAnchor, PanelDef } from "../types";
import { DockGuides, dockTargetAt, documentsRect } from "../DockGuides";

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
/** How much of the title bar always stays on screen. */
const KEEP_VISIBLE = 80;

interface Drag {
  pointerX: number;
  pointerY: number;
  startX: number;
  startY: number;
  /** The whole window body, where the guides sit, in viewport coordinates. */
  main: DOMRect | null;
  /** The documents' area, where a tab goes. */
  documents: DOMRect | null;
}

function guideAt(drag: Drag, clientX: number, clientY: number): DockAnchor | null {
  return drag.main ? dockTargetAt(drag.main, clientX, clientY) : null;
}

/**
 * A panel pulled out of every dock — a free-floating window inside the
 * shell, like a Visual Studio floating tool window. Drag its title bar to
 * move it; while dragging, the dock guides (`DockGuides`) appear over the
 * main area — drop on one, or near an edge, to dock it there. Its pin button
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
      // The guides sit at the middle of the whole window, whatever's docked.
      main: el.closest(".sp-shell")?.querySelector(".sp-shell-body")?.getBoundingClientRect() ?? null,
      documents: documentsRect(el.closest(".sp-shell")),
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
      {dragPos && main && <DockGuides main={main} documents={dragRef.current?.documents} hover={hoverGuide} />}
    </>
  );
}
