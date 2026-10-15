import type { MouseEvent, ReactNode } from "react";
import { IconButton } from "../IconButton";
import { Button } from "../Button";
import { SelectField, type SelectOption } from "../Form";
import { Icon } from "../../infra/icons";
import { t as translate, useT, withShortcut } from "../../infra/i18n";
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
  /** Icon-button groups, the shell's own command bar. An editor's bar builds its row from the `Toolbar*` parts as `children` instead. */
  sections?: ToolbarSection[];
  /** The row, built from `ToolbarGroup`, `ToolbarButton`, `ToolbarModes`, `ToolbarSelect`, `ToolbarCheck`, `ToolbarLabel`, `ToolbarInput`, and `Spacer` / `Divider`. */
  children?: ReactNode;
  className?: string;
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
export function Toolbar({ sections = [], leading, children, className }: ToolbarProps) {
  return (
    <div className={["sp-toolbar", className].filter(Boolean).join(" ")} role="toolbar">
      {leading && <div className="sp-toolbar-section">{leading}</div>}
      {children}
      {sections.map((section, i) => (
        <div className="sp-toolbar-section" key={i}>
          {section.map((item) => (
            <IconButton key={item.label} title={item.label} disabled={item.disabled} onClick={item.onClick}>
              <Icon name={item.icon} />
            </IconButton>
          ))}
        </div>
      ))}
    </div>
  );
}

// -- An editor's command bar, part by part ------------------------------------
//
// Every editor's bar is made of the same parts, so they look and behave
// alike: 24px controls, the same gaps, groups split by a separator, labels
// and shortcuts translated in one place. What's on a bar is the editor's.

/** Controls that belong together; a separator follows it (unless it's last). */
export function ToolbarGroup({ children }: { children: ReactNode }) {
  return <div className="sp-toolbar-section">{children}</div>;
}

export interface ToolbarButtonProps {
  /** An icon name (`infra/icons`). */
  icon?: string;
  /** What it does — translated here; the tooltip, and the text when shown. */
  label: string;
  /** Shown beside the icon (default: only when there's no icon). */
  showLabel?: boolean;
  /** A longer explanation for the tooltip (translated) — default the label. */
  tooltip?: string;
  /** Its shortcut as keybindings write it (`"ctrl+d"`, `"g"`) — added to the tooltip, localized. */
  shortcut?: string;
  /** A state it toggles is on (a mode, a view option): drawn pressed. */
  active?: boolean;
  /** The main action of the bar: drawn in the accent. */
  primary?: boolean;
  disabled?: boolean;
  /** A number beside it (incoming commits, problems) — none when 0. */
  count?: number;
  /** It opens a menu or picker: a chevron after it; the click's event anchors the menu. */
  dropdown?: boolean;
  onClick?: (e: MouseEvent<HTMLButtonElement>) => void;
}

/** A command (or an on/off option) — an icon button, text, or both. */
export function ToolbarButton({ icon, label, showLabel, tooltip, shortcut, active, primary, disabled, count, dropdown, onClick }: ToolbarButtonProps) {
  useT();
  const text = tooltip ?? label;
  const title = shortcut ? withShortcut(text, shortcut) : translate(text);
  const pressed = active === undefined ? undefined : active;
  const badge = !!count && <span className="sp-toolbar-count">{count}</span>;
  if (icon && !showLabel && !badge && !dropdown) {
    return (
      <IconButton title={title} aria-label={title} aria-pressed={pressed} className={active ? "sp-toolbar-active" : undefined} disabled={disabled} onClick={onClick}>
        <Icon name={icon} />
      </IconButton>
    );
  }
  return (
    <Button
      variant={primary ? "primary" : "ghost"}
      title={title}
      aria-pressed={pressed}
      className={["sp-toolbar-button", active && "sp-toolbar-active"].filter(Boolean).join(" ")}
      disabled={disabled}
      onClick={onClick}
    >
      {icon && <Icon name={icon} />}
      {(showLabel || !icon) && <span className="sp-toolbar-button-text">{translate(label)}</span>}
      {badge}
      {dropdown && <Icon name="chevronRight" className="sp-toolbar-chevron" />}
    </Button>
  );
}

export interface ToolbarMode<T extends string> {
  value: T;
  label: string;
  icon?: string;
  shortcut?: string;
  disabled?: boolean;
}

/** One of several modes (Move / Turn / Size, Model / Paint): joined buttons, the current one pressed. */
export function ToolbarModes<T extends string>({ value, modes, onChange, showLabels }: { value: T | null; modes: readonly ToolbarMode<T>[]; onChange: (value: T) => void; showLabels?: boolean }) {
  return (
    <div className="sp-toolbar-modes" role="radiogroup">
      {modes.map((m) => (
        <ToolbarButton key={m.value} icon={m.icon} label={m.label} showLabel={showLabels} shortcut={m.shortcut} active={m.value === value} disabled={m.disabled} onClick={() => onChange(m.value)} />
      ))}
    </div>
  );
}

/** A choice from a list, with an optional label before it (translated). */
export function ToolbarSelect({ label, value, options, onChange, disabled }: { label?: string; value: string; options: readonly SelectOption[]; onChange: (value: string) => void; disabled?: boolean }) {
  useT();
  return (
    <label className="sp-toolbar-field">
      {label && <span className="sp-toolbar-field-label">{translate(label)}</span>}
      <SelectField value={value} options={options} onChange={onChange} disabled={disabled} aria-label={label ? translate(label) : undefined} />
    </label>
  );
}

/** An on/off option shown as a check box with its label (translated). */
export function ToolbarCheck({ label, checked, onChange, disabled }: { label: string; checked: boolean; onChange: (checked: boolean) => void; disabled?: boolean }) {
  useT();
  return (
    <label className={`sp-toolbar-check${disabled ? " is-disabled" : ""}`}>
      <input type="checkbox" checked={checked} disabled={disabled} onChange={(e) => onChange(e.target.checked)} />
      <span>{translate(label)}</span>
    </label>
  );
}

/** Text on the bar: a status, a size, a caption — muted; `tone="error"` for what went wrong. */
export function ToolbarLabel({ children, tone }: { children: ReactNode; tone?: "error" }) {
  return <span className={tone === "error" ? "sp-toolbar-label is-error" : "sp-toolbar-label"}>{children}</span>;
}

/**
 * A short text typed on the bar (a name for something new): Enter submits
 * it, Esc cancels — with ✓ and ✕ beside it.
 */
export function ToolbarInput({ value, onChange, onSubmit, onCancel, placeholder }: { value: string; onChange: (value: string) => void; onSubmit: (value: string) => void; onCancel: () => void; placeholder?: string }) {
  const t = useT();
  const ok = value.trim() !== "";
  return (
    <span className="sp-toolbar-input">
      <input
        autoFocus
        value={value}
        placeholder={placeholder}
        spellCheck={false}
        onChange={(e) => onChange(e.target.value)}
        onKeyDown={(e) => {
          if (e.key === "Enter" && ok) onSubmit(value);
          else if (e.key === "Escape") onCancel();
        }}
      />
      <IconButton title={t("Create")} disabled={!ok} onClick={() => onSubmit(value)}>
        <Icon name="check" />
      </IconButton>
      <IconButton title={t("Cancel")} onClick={onCancel}>
        <Icon name="close" />
      </IconButton>
    </span>
  );
}
