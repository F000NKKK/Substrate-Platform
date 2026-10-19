import { useState } from "react";
import { Icon } from "../infra/icons";
import { useT } from "../infra/i18n";
import { IconButton } from "../widgets/IconButton";
import type { ShellInfo } from "./terminalApi";
import { labelFor, type TerminalTab } from "./terminalTabs";

/** The size of a terminal's icon in the list, in pixels. */
const ICON_SIZE = 14;

/** The terminals, listed on the right: switch, rename (double-click), close. */
export function TerminalSidebar({ tabs, shells, activeId, onActivate, onRename, onClose }: {
  /** In display order: each split group's members together. */
  tabs: TerminalTab[];
  shells: ShellInfo[] | null;
  activeId: string | null;
  onActivate: (id: string) => void;
  onRename: (id: string, label: string) => void;
  onClose: (id: string) => void;
}) {
  const t = useT();
  const [renamingId, setRenamingId] = useState<string | null>(null);
  return (
    <div className="sp-terminal-sidebar">
      {tabs.map((tab, i) => {
        const label = labelFor(shells?.find((s) => s.id === tab.shellId), tab, t);
        return (
          <div
            key={tab.id}
            className="sp-terminal-sidebar-item"
            data-active={tab.id === activeId || undefined}
            data-split={(i > 0 && tabs[i - 1].groupId === tab.groupId) || undefined}
            onClick={() => onActivate(tab.id)}
          >
            <Icon name="terminal" size={ICON_SIZE} />
            {renamingId === tab.id ? (
              <input
                className="sp-terminal-sidebar-item-rename"
                autoFocus
                defaultValue={label}
                onClick={(e) => e.stopPropagation()}
                onBlur={(e) => {
                  onRename(tab.id, e.currentTarget.value);
                  setRenamingId(null);
                }}
                onKeyDown={(e) => {
                  e.stopPropagation();
                  if (e.key === "Enter") e.currentTarget.blur();
                  else if (e.key === "Escape") setRenamingId(null);
                }}
              />
            ) : (
              <span
                className="sp-terminal-sidebar-item-label"
                onDoubleClick={(e) => {
                  e.stopPropagation();
                  setRenamingId(tab.id);
                }}
              >
                {label}
              </span>
            )}
            <IconButton
              size="sm"
              title={t("Close Terminal")}
              className="sp-terminal-sidebar-item-close"
              onClick={(e) => {
                e.stopPropagation();
                onClose(tab.id);
              }}
            >
              <Icon name="close" />
            </IconButton>
          </div>
        );
      })}
    </div>
  );
}
