import { useRef, useState, type PointerEvent } from "react";

export interface DraggableHandlers {
  onPointerDown: (e: PointerEvent) => void;
  onPointerMove: (e: PointerEvent) => void;
  onPointerUp: (e: PointerEvent) => void;
}

export interface Draggable {
  pos: { x: number; y: number };
  handlers: DraggableHandlers;
}

/**
 * The one "drag this element by its header to reposition" implementation —
 * every floating window (ColorPickerWindow, SettingsWindow, ...) wires its
 * header through this instead of re-declaring the same pointer-tracking
 * state each time.
 */
export function useDraggable(initial: { x: number; y: number }): Draggable {
  const [pos, setPos] = useState(initial);
  const origin = useRef<{ x: number; y: number; startX: number; startY: number } | null>(null);

  return {
    pos,
    handlers: {
      onPointerDown: (e) => {
        e.currentTarget.setPointerCapture(e.pointerId);
        origin.current = { x: e.clientX, y: e.clientY, startX: pos.x, startY: pos.y };
      },
      onPointerMove: (e) => {
        const o = origin.current;
        if (!o || e.buttons !== 1) return;
        setPos({ x: o.startX + (e.clientX - o.x), y: o.startY + (e.clientY - o.y) });
      },
      onPointerUp: () => {
        origin.current = null;
      },
    },
  };
}
