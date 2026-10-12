import type { ReactNode } from "react";
import { Button, type ButtonVariant } from "../Button";
import "./ComponentList.css";

/** Where a component stands — drives the status badge's color. */
export type ComponentStatus = "installed" | "missing" | "outdated" | "partial" | "checking" | "working" | "failed";

export interface ComponentListAction {
  label: string;
  onClick: () => void;
  disabled?: boolean;
  variant?: ButtonVariant;
}

export interface ComponentListItem {
  id: string;
  name: string;
  description?: ReactNode;
  status: ComponentStatus;
  /** The badge's text (already translated) — e.g. "Installed", "Update available". */
  statusLabel: string;
  /** e.g. "1.99.0" or "1.99.0 → 1.100.0". */
  version?: string;
  /** A muted second line — where it was found, what's missing. */
  detail?: ReactNode;
  /** While `status` is `"working"`: 0..1, or `null` for indeterminate. */
  progress?: number | null;
  progressLabel?: string;
  actions?: ComponentListAction[];
  /** Shown dimmed — not needed for the current configuration. */
  optional?: boolean;
  /** With `onSelectedChange`, a checkbox picks it for a batch action (Visual Studio Installer's workloads). */
  selected?: boolean;
  onSelectedChange?: (selected: boolean) => void;
}

export interface ComponentListProps {
  items: ComponentListItem[];
  emptyLabel?: string;
}

/**
 * An installer-style list of components — each with its status badge,
 * version, detail line, a progress bar while something is being done to
 * it, and its actions (Install / Update / ...). Product-neutral: the product
 * decides what the components are and what the actions do.
 */
export function ComponentList({ items, emptyLabel }: ComponentListProps) {
  if (items.length === 0 && emptyLabel) return <p className="sp-component-list-empty">{emptyLabel}</p>;
  return (
    <ul className="sp-component-list">
      {items.map((item) => (
        <li key={item.id} className={`sp-component-row${item.optional ? " is-optional" : ""}`}>
          {item.onSelectedChange && (
            <input
              type="checkbox"
              className="sp-component-select"
              checked={!!item.selected}
              aria-label={item.name}
              onChange={(e) => item.onSelectedChange?.(e.target.checked)}
            />
          )}
          <div className="sp-component-main">
            <div className="sp-component-title">
              <span className="sp-component-name">{item.name}</span>
              <span className={`sp-component-status is-${item.status}`}>{item.statusLabel}</span>
              {item.version && <span className="sp-component-version">{item.version}</span>}
            </div>
            {item.description && <div className="sp-component-description">{item.description}</div>}
            {item.detail && <div className="sp-component-detail">{item.detail}</div>}
            {item.status === "working" && (
              <div className="sp-component-progress">
                <div
                  className={`sp-component-progress-bar${item.progress == null ? " is-indeterminate" : ""}`}
                  role="progressbar"
                  aria-valuemin={0}
                  aria-valuemax={100}
                  aria-valuenow={item.progress == null ? undefined : Math.round(item.progress * 100)}
                >
                  <div className="sp-component-progress-fill" style={item.progress == null ? undefined : { width: `${Math.round(item.progress * 100)}%` }} />
                </div>
                {item.progressLabel && <span className="sp-component-progress-label">{item.progressLabel}</span>}
              </div>
            )}
          </div>
          {item.actions && item.actions.length > 0 && (
            <div className="sp-component-actions">
              {item.actions.map((action) => (
                <Button key={action.label} variant={action.variant ?? "subtle"} disabled={action.disabled} onClick={action.onClick}>
                  {action.label}
                </Button>
              ))}
            </div>
          )}
        </li>
      ))}
    </ul>
  );
}
