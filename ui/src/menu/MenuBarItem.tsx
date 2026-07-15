import { Button } from "../widgets";

export interface MenuBarItemProps {
  label: string;
  onClick?: () => void;
}

/** One top-level entry in a MenuBar (File, Edit, View, ...). Not a DnD surface — the menu bar is the one part of the shell that stays fixed. */
export function MenuBarItem({ label, onClick }: MenuBarItemProps) {
  return (
    <Button variant="ghost" onClick={onClick}>
      {label}
    </Button>
  );
}
