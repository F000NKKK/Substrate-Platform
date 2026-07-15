export interface MenuBarItemProps {
  label: string;
  onClick?: () => void;
}

/** One top-level entry in a MenuBar (File, Edit, View, ...). Not a DnD surface — the menu bar is the one part of the shell that stays fixed. */
export function MenuBarItem({ label, onClick }: MenuBarItemProps) {
  return (
    <button className="sp-menubar-item" onClick={onClick}>
      {label}
    </button>
  );
}
