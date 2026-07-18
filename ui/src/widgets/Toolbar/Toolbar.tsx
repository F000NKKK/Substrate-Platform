import { IconButton } from "../IconButton";
import { Icon } from "../../infra/icons";
import "./Toolbar.css";

export interface ToolbarItem {
  icon: string;
  label: string;
  onClick?: () => void;
  disabled?: boolean;
}

/** A group of related items, visually separated from the next group by a thin divider — VS's own toolbar "islands". */
export type ToolbarSection = ToolbarItem[];

export interface ToolbarProps {
  sections: ToolbarSection[];
}

/**
 * A VS-style toolbar "island" — one or more icon-button groups in a row,
 * each group divided from the next. Purely presentational: a product
 * supplies whatever actions it wants (save, undo/redo, format, hot reload,
 * ...) as plain `{icon, label, onClick}` entries, the same "commands as
 * props" shape used throughout the platform.
 */
export function Toolbar({ sections }: ToolbarProps) {
  return (
    <div className="sp-toolbar">
      {sections.map((section, i) => (
        <div className="sp-toolbar-section" key={i}>
          {section.map((item) => (
            <IconButton key={item.label} size={26} title={item.label} disabled={item.disabled} onClick={item.onClick}>
              <Icon name={item.icon} size={15} />
            </IconButton>
          ))}
        </div>
      ))}
    </div>
  );
}
