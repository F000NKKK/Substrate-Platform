import { useEffect, useRef, useState, type MouseEvent as ReactMouseEvent } from "react";

export type ContextMenuTarget<T> =
  | { mode: "viewport"; node: T; x: number; y: number }
  | { mode: "anchor"; node: T };

/**
 * State/positioning for a right-click popup (`openAtPoint`, viewport-fixed at
 * the click coordinates) or a click-to-open dropdown (`openAtAnchor`,
 * CSS-relative to a positioned ancestor the consumer already renders). Closes
 * on outside pointerdown, Escape, resize, or a scroll that moves the spot it
 * was opened from — the same triggers `Tree` originally implemented on its
 * own; every consumer gets them for free.
 *
 * Only *that* scroll counts: the IDE scrolls other things on its own all the
 * time (the Output panel following a build, a tree revealing a node), and a
 * menu has to survive those. Anchor dropdowns render inline and move with
 * their ancestor, so no scroll closes them.
 */
export function useContextMenu<T = void>() {
  const [target, setTarget] = useState<ContextMenuTarget<T> | null>(null);
  /** What a viewport menu was opened over — see `scrollMoves`. */
  const origin = useRef<MenuOrigin | null>(null);

  function openAtPoint(node: T, e: ReactMouseEvent | MouseEvent) {
    e.preventDefault();
    origin.current = originOf(e.target instanceof Node ? e.target : document.elementFromPoint(e.clientX, e.clientY));
    setTarget({ mode: "viewport", node, x: e.clientX, y: e.clientY });
  }

  function openAtAnchor(node: T) {
    origin.current = null;
    setTarget({ mode: "anchor", node });
  }

  function close() {
    setTarget(null);
  }

  useEffect(() => {
    if (!target) return;
    const viewport = target.mode === "viewport";
    function handleDismiss(e: Event) {
      if (e instanceof KeyboardEvent && e.key !== "Escape") return;
      if (e.type === "scroll" && !(viewport && scrollMoves(e.target, origin.current))) return;
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

/** Where a viewport menu was opened: the element under the pointer and its on-screen spot then. */
interface MenuOrigin {
  node: Node;
  top: number;
  left: number;
}

function originOf(node: Node | null): MenuOrigin | null {
  const element = node instanceof Element ? node : node?.parentElement;
  if (!node || !element) return null;
  const rect = element.getBoundingClientRect();
  return { node, top: rect.top, left: rect.left };
}

/** Whether scrolling `scrolled` (a scroll event's target) actually moved `origin` on screen — a scroll elsewhere, one that left it in place (a content refresh nudging `scrollTop`), or one after a refresh replaced it doesn't. */
export function scrollMoves(scrolled: EventTarget | null, origin: MenuOrigin | null): boolean {
  if (!origin) return true;
  if (!origin.node.isConnected) return false;
  const inside = scrolled === document || scrolled === window || (scrolled instanceof Node && scrolled.contains(origin.node));
  const now = inside && originOf(origin.node);
  return !!now && (now.top !== origin.top || now.left !== origin.left);
}
