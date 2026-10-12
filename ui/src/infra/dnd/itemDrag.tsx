import { useEffect, useRef, type PointerEvent as ReactPointerEvent, type RefObject } from "react";
import "./itemDrag.css";

/**
 * Dragging an item (a toolbox entry) from one part of the IDE onto another
 * (a design surface) — done with pointer events rather than HTML drag and
 * drop, which webviews handle unreliably (WebKitGTK hides custom types
 * mid-drag; Windows' WebView2 hands drags to the window's file drop).
 * The source calls `beginItemDrag` from pointerdown; targets register
 * with `useItemDropTarget`.
 */

export interface DraggedItem {
  /** What kind of item — a target only reacts to the types it accepts. */
  type: string;
  data: string;
  /** Shown next to the pointer while dragging. */
  label: string;
}

export interface ItemDropTarget {
  accepts: (type: string) => boolean;
  /** The pointer moved over the target with an accepted item. */
  over?: (clientX: number, clientY: number, item: DraggedItem) => void;
  /** The pointer left it, or the drag ended or was cancelled. */
  leave?: () => void;
  drop: (clientX: number, clientY: number, item: DraggedItem) => void;
}

const targets = new Map<Element, ItemDropTarget>();

/** Makes `ref`'s element a drop target while `target` is set. */
export function useItemDropTarget(ref: RefObject<HTMLElement | null>, target: ItemDropTarget | null): void {
  const targetRef = useRef(target);
  targetRef.current = target;
  const enabled = !!target;
  useEffect(() => {
    const el = ref.current;
    if (!el || !enabled) return;
    // Always the latest callbacks, without re-registering every render.
    const proxy: ItemDropTarget = {
      accepts: (type) => !!targetRef.current?.accepts(type),
      over: (x, y, item) => targetRef.current?.over?.(x, y, item),
      leave: () => targetRef.current?.leave?.(),
      drop: (x, y, item) => targetRef.current?.drop(x, y, item),
    };
    targets.set(el, proxy);
    return () => {
      targets.delete(el);
    };
  }, [ref, enabled]);
}

/** How far (px) the pointer travels before a press becomes a drag. */
const DRAG_THRESHOLD = 5;

function targetAt(x: number, y: number, type: string): ItemDropTarget | null {
  for (let el = document.elementFromPoint(x, y); el; el = el.parentElement) {
    const target = targets.get(el);
    if (target?.accepts(type)) return target;
  }
  return null;
}

/**
 * Starts tracking a possible drag from a pointerdown (left button only); it
 * becomes a drag once the pointer moves a few pixels, so clicks and
 * double-clicks on the source still work. Escape cancels.
 */
export function beginItemDrag(e: ReactPointerEvent | PointerEvent, item: DraggedItem): void {
  if (e.button !== 0) return;
  const startX = e.clientX;
  const startY = e.clientY;
  let ghost: HTMLDivElement | null = null;
  let current: ItemDropTarget | null = null;

  function setCurrent(next: ItemDropTarget | null) {
    if (next === current) return;
    current?.leave?.();
    current = next;
  }
  function cleanup() {
    window.removeEventListener("pointermove", onMove);
    window.removeEventListener("pointerup", onUp);
    window.removeEventListener("keydown", onKey, true);
    ghost?.remove();
    ghost = null;
    document.documentElement.classList.remove("sp-item-dragging", "sp-item-drag-none");
    setCurrent(null);
  }
  function onMove(ev: PointerEvent) {
    if (!ghost) {
      if (Math.hypot(ev.clientX - startX, ev.clientY - startY) < DRAG_THRESHOLD) return;
      ghost = document.createElement("div");
      ghost.className = "sp-item-drag-ghost";
      ghost.textContent = item.label;
      document.body.appendChild(ghost);
      document.documentElement.classList.add("sp-item-dragging");
      window.getSelection()?.removeAllRanges();
    }
    ghost.style.left = `${ev.clientX + 12}px`;
    ghost.style.top = `${ev.clientY + 8}px`;
    const target = targetAt(ev.clientX, ev.clientY, item.type);
    setCurrent(target);
    document.documentElement.classList.toggle("sp-item-drag-none", !target);
    target?.over?.(ev.clientX, ev.clientY, item);
  }
  function onUp(ev: PointerEvent) {
    const dragging = !!ghost;
    const target = dragging ? targetAt(ev.clientX, ev.clientY, item.type) : null;
    cleanup();
    target?.drop(ev.clientX, ev.clientY, item);
  }
  function onKey(ev: KeyboardEvent) {
    if (ev.key !== "Escape" || !ghost) return;
    ev.preventDefault();
    ev.stopPropagation();
    cleanup();
  }
  window.addEventListener("pointermove", onMove);
  window.addEventListener("pointerup", onUp);
  window.addEventListener("keydown", onKey, true);
}
