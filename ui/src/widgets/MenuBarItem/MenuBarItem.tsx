import { Button } from "../Button";
import { ContextMenu, useContextMenu, type ContextMenuItem } from "../ContextMenu";
import "./MenuBarItem.css";

export interface MenuBarDropdownEntry {
  label: string;
  onClick?: () => void;
  disabled?: boolean;
  /** A keybinding hint shown right-aligned (e.g. `"Ctrl+S"`) — display only; the product registers the actual keybinding itself. */
  shortcut?: string;
  /** Shows a check glyph — for toggles like a tool window's visibility. */
  checked?: boolean;
  /** Draws a divider line above this entry, grouping it apart from the ones before. */
  separatorBefore?: boolean;
  /** Opens a nested submenu of these entries instead of firing `onClick` (e.g. File > Recent Projects). */
  items?: MenuBarDropdownEntry[];
}

export interface MenuBarItemProps {
  label: string;
  onClick?: () => void;
  /** When present, clicking the item opens a dropdown of these entries (e.g. Tools > Options) instead of firing onClick directly. */
  items?: MenuBarDropdownEntry[];
}

/** Maps the menu bar's own entry shape onto `ContextMenu`'s — recursively, for submenus. */
function toContextMenuItems(entries: MenuBarDropdownEntry[]): ContextMenuItem[] {
  return entries.map((entry) => ({
    label: entry.label,
    onSelect: entry.onClick,
    disabled: entry.disabled,
    shortcut: entry.shortcut,
    checked: entry.checked,
    separatorBefore: entry.separatorBefore,
    submenu: entry.items ? toContextMenuItems(entry.items) : undefined,
  }));
}

/** One top-level entry in a MenuBar (File, Edit, View, ...), optionally opening a dropdown — the menu bar itself is not a DnD surface. */
export function MenuBarItem({ label, onClick, items }: MenuBarItemProps) {
  const menu = useContextMenu<void>();

  function handleClick() {
    if (items && items.length > 0) {
      if (menu.target) menu.close();
      else menu.openAtAnchor(undefined);
    } else onClick?.();
  }

  return (
    <div className="sp-menubar-item-root">
      <Button variant="ghost" onClick={handleClick}>
        {label}
      </Button>
      <ContextMenu target={menu.target ? { mode: "anchor" } : null} onClose={menu.close} items={toContextMenuItems(items ?? [])} />
    </div>
  );
}
