import type { PointerEvent as ReactPointerEvent } from "react";

/** Follows a pointer drag started by `e` on its target: `onMove` for each move, `onEnd` once it's released or cancelled. */
export function trackPointer(e: ReactPointerEvent<HTMLElement>, onMove: (ev: PointerEvent) => void, onEnd?: () => void): void {
  const target = e.currentTarget;
  target.setPointerCapture(e.pointerId);
  const onUp = () => {
    target.removeEventListener("pointermove", onMove);
    target.removeEventListener("pointerup", onUp);
    target.removeEventListener("pointercancel", onUp);
    onEnd?.();
  };
  target.addEventListener("pointermove", onMove);
  target.addEventListener("pointerup", onUp);
  target.addEventListener("pointercancel", onUp);
}
