import type { ReactNode } from "react";
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
  /** Arbitrary content rendered before the icon sections, in its own divided slot — for something with its own state/behavior (a run-target picker, a search box, ...) that belongs in the same row without `Toolbar` itself knowing anything about it. */
  leading?: ReactNode;
}

/**
 * A VS-style toolbar "island" — one or more icon-button groups in a row,
 * each group divided from the next, plus an optional `leading` slot for
 * something more bespoke. Purely presentational and deliberately dumb: a
 * product supplies whatever actions it wants (save, undo/redo, format, hot
 * reload, ...) as plain `{icon, label, onClick}` entries, the same
 * "commands as props" shape used throughout the platform — this component
 * has no opinion about what any of them *do*, and anything with its own
 * responsibility (like picking a run target) stays its own component
 * composed in via `leading`, not logic bolted onto this one.
 */
export function Toolbar({ sections, leading }: ToolbarProps) {
  return (
    <div className="sp-toolbar">
      {leading && <div className="sp-toolbar-section">{leading}</div>}
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
