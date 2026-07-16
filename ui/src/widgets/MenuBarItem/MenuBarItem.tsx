import { Button } from "../Button";
import { ContextMenu, useContextMenu } from "../ContextMenu";
import "./MenuBarItem.css";

export interface MenuBarDropdownEntry {
  label: string;
  onClick?: () => void;
}

export interface MenuBarItemProps {
  label: string;
  onClick?: () => void;
  /** When present, clicking the item opens a dropdown of these entries (e.g. Tools > Options) instead of firing onClick directly. */
  items?: MenuBarDropdownEntry[];
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
      <ContextMenu
        target={menu.target ? { mode: "anchor" } : null}
        onClose={menu.close}
        items={(items ?? []).map((entry) => ({ label: entry.label, onSelect: entry.onClick }))}
      />
    </div>
  );
}
