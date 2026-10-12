import { useEffect, useRef } from "react";
import { PanelSurface } from "../PanelSurface/PanelSurface";
import type { PanelDef } from "../types";

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
}

/** Waits for a resize drag to settle before reporting it, rather than once per frame. */
const RESIZE_SETTLE_MS = 150;

/**
 * A panel pulled out of every dock — a free-floating window inside the
 * shell. Drag its header onto a dock strip or a main-area edge to redock it
 * there (or onto the main area's center to make it a center tab), use its
 * pin button to send it back to its home dock, or drag its bottom-right
 * corner to resize (plain CSS `resize`; the settled size is reported via
 * `onResize`).
 */
export function FloatingPanel({ panel, x, y, w, h, onDock, onClose, onResize }: FloatingPanelProps) {
  const Component = panel.component;
  const ref = useRef<HTMLDivElement>(null);
  const onResizeRef = useRef(onResize);
  onResizeRef.current = onResize;
  const sizeRef = useRef({ w, h });
  sizeRef.current = { w, h };

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

  return (
    <div ref={ref} className="sp-floating-panel" style={{ left: x, top: y, width: w, height: h }}>
      <PanelSurface panelId={panel.id} title={panel.title} pinned={false} onTogglePin={onDock} onClose={onClose}>
        <Component />
      </PanelSurface>
    </div>
  );
}
