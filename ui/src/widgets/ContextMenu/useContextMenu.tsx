import { useEffect, useState, type MouseEvent as ReactMouseEvent } from "react";

export type ContextMenuTarget<T> =
  | { mode: "viewport"; node: T; x: number; y: number }
  | { mode: "anchor"; node: T };

/**
 * State/positioning for a right-click popup (`openAtPoint`, viewport-fixed at
 * the click coordinates) or a click-to-open dropdown (`openAtAnchor`,
 * CSS-relative to a positioned ancestor the consumer already renders). Closes
 * on outside pointerdown, Escape, scroll, or resize — the same four triggers
 * `Tree` originally implemented on its own; every consumer gets them for free.
 */
export function useContextMenu<T = void>() {
  const [target, setTarget] = useState<ContextMenuTarget<T> | null>(null);

  function openAtPoint(node: T, e: ReactMouseEvent | MouseEvent) {
    e.preventDefault();
    setTarget({ mode: "viewport", node, x: e.clientX, y: e.clientY });
  }

  function openAtAnchor(node: T) {
    setTarget({ mode: "anchor", node });
  }

  function close() {
    setTarget(null);
  }

  useEffect(() => {
    if (!target) return;
    function handleDismiss(e: Event) {
      if (e instanceof KeyboardEvent && e.key !== "Escape") return;
      setTarget(null);
    }
    window.addEventListener("pointerdown", handleDismiss);
    window.addEventListener("keydown", handleDismiss);
    window.addEventListener("scroll", handleDismiss, true);
    window.addEventListener("resize", handleDismiss);
    return () => {
      window.removeEventListener("pointerdown", handleDismiss);
      window.removeEventListener("keydown", handleDismiss);
      window.removeEventListener("scroll", handleDismiss, true);
      window.removeEventListener("resize", handleDismiss);
    };
  }, [target]);

  return { target, openAtPoint, openAtAnchor, close };
}
