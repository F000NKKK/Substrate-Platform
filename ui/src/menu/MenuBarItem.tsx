import { useEffect, useRef, useState } from "react";
import { Button, Tab } from "../widgets";
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
  const [open, setOpen] = useState(false);
  const rootRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) return;
    const onPointerDown = (e: PointerEvent) => {
      if (!rootRef.current?.contains(e.target as Node)) setOpen(false);
    };
    document.addEventListener("pointerdown", onPointerDown);
    return () => document.removeEventListener("pointerdown", onPointerDown);
  }, [open]);

  function handleClick() {
    if (items && items.length > 0) setOpen((v) => !v);
    else onClick?.();
  }

  return (
    <div className="sp-menubar-item-root" ref={rootRef}>
      <Button variant="ghost" onClick={handleClick}>
        {label}
      </Button>
      {open && items && (
        <div className="sp-menubar-dropdown">
          {items.map((entry) => (
            <Tab
              key={entry.label}
              orientation="list"
              onClick={() => {
                setOpen(false);
                entry.onClick?.();
              }}
            >
              {entry.label}
            </Tab>
          ))}
        </div>
      )}
    </div>
  );
}
