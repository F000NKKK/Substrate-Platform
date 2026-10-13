import { useMemo, useState, type ReactNode } from "react";
import { IconButton } from "../../widgets/IconButton";
import { Tab } from "../../widgets/Tab";
import { TextField } from "../../widgets/TextField";
import { useDraggable } from "../../infra/hooks";
import { useStoredState } from "../../infra/state";
import { IconClose, IconSettings } from "../../infra/icons";
import { t, useLocale } from "../../infra/i18n";
import "./SettingsWindow.css";

export interface SettingsSection {
  id: string;
  label: string;
  content: ReactNode;
  /** The individual setting labels on this section (e.g. `editorSettingsKeywords`) — search matches these (in English and in the active language) as well as `label`, so typing "tab size" finds the section that has it. */
  keywords?: readonly string[];
}

function matches(section: SettingsSection, query: string): boolean {
  if (!query) return true;
  const keywords = (section.keywords ?? []).flatMap((k) => [k, t(k)]);
  return [section.label, ...keywords].some((text) => text.toLowerCase().includes(query));
}

export interface SettingsWindowProps {
  sections: SettingsSection[];
  /** Opens on this section — otherwise on the one last viewed (remembered in the current `StateStore`), or the first. */
  initialSectionId?: string;
  onClose: () => void;
  /** The title bar's text — "Options" by default (a project's property pages pass "{name} Properties"). */
  title?: string;
  /** Extra controls in the title bar, before the close button (e.g. "View Code"). */
  headerExtra?: ReactNode;
  /** Where the last viewed section is remembered — give each kind of window its own key. */
  sectionStateKey?: string;
  /** Hides the search box — for windows with a handful of fixed pages. */
  hideSearch?: boolean;
}

/**
 * A real Options-style window — its own title bar (draggable, closable),
 * a searchable category list on the left (search matches a section's name
 * or any of its `keywords`), and the selected category's settings on the
 * right. Any product wires its own sections in; the shell
 * (this window) is the one reusable piece.
 */
export function SettingsWindow({ sections, initialSectionId, onClose, title, headerExtra, sectionStateKey = "settings.section", hideSearch }: SettingsWindowProps) {
  const locale = useLocale();
  const { pos, handlers } = useDraggable({ x: 240, y: 100 });
  const [lastSectionId, setLastSectionId] = useStoredState<string | undefined>(
    sectionStateKey,
    undefined,
    (v) => typeof v === "string"
  );
  const [selectedId, setSelectedId] = useState(initialSectionId ?? lastSectionId ?? sections[0]?.id);
  const [search, setSearch] = useState("");

  const filtered = useMemo(() => sections.filter((s) => matches(s, search.trim().toLowerCase())), [sections, search, locale]);
  // A search that filters out the selected section shows the first match
  // instead, rather than a page the list no longer offers.
  const selected = filtered.find((s) => s.id === selectedId) ?? filtered[0];

  return (
    <div className="sp-settings-window" style={{ left: pos.x, top: pos.y }}>
      <div className="sp-settings-header" {...handlers}>
        <IconSettings size={14} />
        <span className="sp-settings-title">{title ?? t("Options")}</span>
        {headerExtra && (
          <div className="sp-settings-header-extra" onPointerDown={(e) => e.stopPropagation()}>
            {headerExtra}
          </div>
        )}
        <IconButton aria-label={t("Close")} onClick={onClose}>
          <IconClose />
        </IconButton>
      </div>

      <div className="sp-settings-body">
        <div className="sp-settings-sidebar">
          {!hideSearch && (
            <TextField
            placeholder={t("Search Settings")}
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            />
          )}
          <div className="sp-settings-categories">
            {filtered.map((section) => (
              <Tab
                key={section.id}
                orientation="list"
                active={section.id === selected?.id}
                onClick={() => {
                  setSelectedId(section.id);
                  setLastSectionId(section.id);
                }}
              >
                {section.label}
              </Tab>
            ))}
          </div>
        </div>
        <div className="sp-settings-detail">
          {selected ? selected.content : <p className="sp-settings-empty">{t('No settings match "{query}".', { query: search })}</p>}
        </div>
      </div>
    </div>
  );
}
