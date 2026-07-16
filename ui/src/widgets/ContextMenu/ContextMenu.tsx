import { useLayoutEffect, useRef, useState, type ReactNode } from "react";
import { createPortal } from "react-dom";
import { Tab } from "../Tab";
import "./ContextMenu.css";

export interface ContextMenuItem {
  label: ReactNode;
  onSelect?: () => void;
  destructive?: boolean;
  disabled?: boolean;
  /** Shows a check glyph — used for toggles like a column's visibility. */
  checked?: boolean;
  /** Renders a nested panel instead of firing `onSelect` directly. */
  submenu?: ContextMenuItem[];
  /** Draws a divider line above this item — for grouping unrelated actions within one menu. */
  separatorBefore?: boolean;
}

export type ContextMenuPlacement = { mode: "viewport"; x: number; y: number } | { mode: "anchor" };

export interface ContextMenuProps {
  target: ContextMenuPlacement | null;
  items: ContextMenuItem[];
  onClose: () => void;
}

const VIEWPORT_MARGIN = 4;
const SUBMENU_CLOSE_DELAY = 150;

function clampToViewport(x: number, y: number, width: number, height: number) {
  const maxLeft = Math.max(VIEWPORT_MARGIN, window.innerWidth - width - VIEWPORT_MARGIN);
  const maxTop = Math.max(VIEWPORT_MARGIN, window.innerHeight - height - VIEWPORT_MARGIN);
  return { left: Math.min(x, maxLeft), top: Math.min(y, maxTop) };
}

/** A submenu, portaled and positioned from its trigger row's rect — same viewport-clamping `ViewportMenu` gives the root menu, so a deep list can't run off the edge of the screen either. Flips to the trigger's left when there isn't room on the right. */
function SubmenuPanel({
  anchorRect,
  items,
  onClose,
  onPointerEnter,
  onPointerLeave,
}: {
  anchorRect: DOMRect;
  items: ContextMenuItem[];
  onClose: () => void;
  onPointerEnter: () => void;
  onPointerLeave: () => void;
}) {
  const ref = useRef<HTMLDivElement>(null);
  const [pos, setPos] = useState({ left: anchorRect.right, top: anchorRect.top });

  useLayoutEffect(() => {
    const el = ref.current;
    if (!el) return;
    const rect = el.getBoundingClientRect();
    const fitsRight = anchorRect.right + rect.width + VIEWPORT_MARGIN <= window.innerWidth;
    const left = fitsRight ? anchorRect.right : Math.max(VIEWPORT_MARGIN, anchorRect.left - rect.width);
    const clamped = clampToViewport(left, anchorRect.top, rect.width, rect.height);
    setPos(clamped);
  }, [anchorRect]);

  return createPortal(
    <div
      ref={ref}
      className="sp-contextmenu-submenu"
      style={{ left: pos.left, top: pos.top }}
      onPointerDown={(e) => e.stopPropagation()}
      onPointerEnter={onPointerEnter}
      onPointerLeave={onPointerLeave}
    >
      <ContextMenuList items={items} onClose={onClose} />
    </div>,
    document.body
  );
}

function MenuItemRow({ item, onClose }: { item: ContextMenuItem; onClose: () => void }) {
  const rowRef = useRef<HTMLDivElement>(null);
  const [open, setOpen] = useState(false);
  const closeTimer = useRef<number | undefined>(undefined);

  function scheduleClose() {
    window.clearTimeout(closeTimer.current);
    closeTimer.current = window.setTimeout(() => setOpen(false), SUBMENU_CLOSE_DELAY);
  }
  function cancelClose() {
    window.clearTimeout(closeTimer.current);
  }

  return (
    <div
      className="sp-contextmenu-itemwrap"
      ref={rowRef}
      onPointerEnter={() => {
        if (!item.submenu) return;
        cancelClose();
        setOpen(true);
      }}
      onPointerLeave={() => {
        if (!item.submenu) return;
        scheduleClose();
      }}
    >
      {item.separatorBefore && <div className="sp-contextmenu-separator" />}
      <Tab
        orientation="list"
        disabled={item.disabled}
        className={item.destructive ? "sp-contextmenu-item--destructive" : undefined}
        onClick={() => {
          if (item.disabled) return;
          if (item.submenu) return;
          item.onSelect?.();
          onClose();
        }}
      >
        {item.checked && <span className="sp-contextmenu-check">✓</span>}
        {item.label}
        {item.submenu && <span className="sp-contextmenu-dot" />}
      </Tab>
      {item.submenu && open && rowRef.current && (
        <SubmenuPanel
          anchorRect={rowRef.current.getBoundingClientRect()}
          items={item.submenu}
          onClose={onClose}
          onPointerEnter={cancelClose}
          onPointerLeave={scheduleClose}
        />
      )}
    </div>
  );
}

function ContextMenuList({ items, onClose }: { items: ContextMenuItem[]; onClose: () => void }) {
  return (
    <div className="sp-contextmenu-list">
      {items.map((item, i) => (
        <MenuItemRow item={item} onClose={onClose} key={i} />
      ))}
    </div>
  );
}

/** Portaled viewport-fixed popup, clamped to stay fully on-screen regardless of where the triggering click landed. */
function ViewportMenu({ x, y, items, onClose }: { x: number; y: number; items: ContextMenuItem[]; onClose: () => void }) {
  const ref = useRef<HTMLDivElement>(null);
  const [pos, setPos] = useState({ left: x, top: y });

  useLayoutEffect(() => {
    const el = ref.current;
    if (!el) return;
    const rect = el.getBoundingClientRect();
    setPos(clampToViewport(x, y, rect.width, rect.height));
  }, [x, y]);

  return createPortal(
    <div ref={ref} className="sp-contextmenu" style={{ left: pos.left, top: pos.top }} onPointerDown={(e) => e.stopPropagation()}>
      <ContextMenuList items={items} onClose={onClose} />
    </div>,
    document.body
  );
}

/**
 * The one context-menu/dropdown implementation in the platform — backs
 * `Tree`'s right-click node menu, `MenuBarItem`'s dropdown, and `DataGrid`'s
 * grid/header menus. Pair with `useContextMenu` for the open/close state;
 * this component only renders. `mode: "viewport"` portals to `document.body`
 * and positions at the click coordinates (fixed), clamped to the window so it
 * can't render partially or fully off-screen near an edge or a multi-monitor
 * boundary — a right-click popup can also be triggered from inside a panel
 * with `overflow: hidden` or `backdrop-filter` (both of which trap
 * `position: fixed` to that ancestor instead of the viewport), so escaping
 * via portal is what keeps it from being clipped or mispositioned there too.
 * Submenus get the same portal + clamp treatment, anchored to their trigger
 * row instead of a click point. `mode: "anchor"` renders inline, positioned
 * via CSS relative to whatever already-positioned ancestor the consumer
 * renders it inside — that ancestor is the point, so it isn't portaled.
 */
export function ContextMenu({ target, items, onClose }: ContextMenuProps) {
  if (!target) return null;

  if (target.mode === "anchor") {
    return (
      <div className="sp-contextmenu sp-contextmenu--anchor" onPointerDown={(e) => e.stopPropagation()}>
        <ContextMenuList items={items} onClose={onClose} />
      </div>
    );
  }

  return <ViewportMenu x={target.x} y={target.y} items={items} onClose={onClose} />;
}
