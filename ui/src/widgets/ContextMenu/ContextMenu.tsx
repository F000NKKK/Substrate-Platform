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
}

export type ContextMenuPlacement = { mode: "viewport"; x: number; y: number } | { mode: "anchor" };

export interface ContextMenuProps {
  target: ContextMenuPlacement | null;
  items: ContextMenuItem[];
  onClose: () => void;
}

function ContextMenuList({ items, onClose }: { items: ContextMenuItem[]; onClose: () => void }) {
  return (
    <div className="sp-contextmenu-list">
      {items.map((item, i) => (
        <div className="sp-contextmenu-itemwrap" key={i}>
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
            {item.submenu && <span className="sp-contextmenu-caret">▸</span>}
          </Tab>
          {item.submenu && (
            <div className="sp-contextmenu-submenu">
              <ContextMenuList items={item.submenu} onClose={onClose} />
            </div>
          )}
        </div>
      ))}
    </div>
  );
}

const VIEWPORT_MARGIN = 4;

/** Portaled viewport-fixed popup, clamped to stay fully on-screen regardless of where the triggering click landed. */
function ViewportMenu({ x, y, items, onClose }: { x: number; y: number; items: ContextMenuItem[]; onClose: () => void }) {
  const ref = useRef<HTMLDivElement>(null);
  const [pos, setPos] = useState({ left: x, top: y });

  useLayoutEffect(() => {
    const el = ref.current;
    if (!el) return;
    const rect = el.getBoundingClientRect();
    const maxLeft = Math.max(VIEWPORT_MARGIN, window.innerWidth - rect.width - VIEWPORT_MARGIN);
    const maxTop = Math.max(VIEWPORT_MARGIN, window.innerHeight - rect.height - VIEWPORT_MARGIN);
    setPos({ left: Math.min(x, maxLeft), top: Math.min(y, maxTop) });
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
 * `mode: "anchor"` renders inline, positioned via CSS relative to whatever
 * already-positioned ancestor the consumer renders it inside — that ancestor
 * is the point, so it isn't portaled.
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
