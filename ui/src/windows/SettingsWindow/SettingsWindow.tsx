import { useMemo, useState, type ReactNode } from "react";
import { IconButton } from "../../widgets/IconButton";
import { Tab } from "../../widgets/Tab";
import { TextField } from "../../widgets/TextField";
import { useDraggable } from "../../infra/hooks";
import { useStoredState } from "../../infra/state";
import { IconClose, IconSettings } from "../../infra/icons";
import "./SettingsWindow.css";

export interface SettingsSection {
  id: string;
  label: string;
  content: ReactNode;
  /** The individual setting labels on this section (e.g. `editorSettingsKeywords`) — search matches these as well as `label`, so typing "tab size" finds the section that has it. */
  keywords?: readonly string[];
}

function matches(section: SettingsSection, query: string): boolean {
  if (!query) return true;
  return [section.label, ...(section.keywords ?? [])].some((text) => text.toLowerCase().includes(query));
}

export interface SettingsWindowProps {
  sections: SettingsSection[];
  /** Opens on this section — otherwise on the one last viewed (remembered in the current `StateStore`), or the first. */
  initialSectionId?: string;
  onClose: () => void;
}

/**
 * A real Options-style window — its own title bar (draggable, closable),
 * a searchable category list on the left (search matches a section's name
 * or any of its `keywords`), and the selected category's settings on the
 * right. Any product wires its own sections in; the shell
 * (this window) is the one reusable piece.
 */
export function SettingsWindow({ sections, initialSectionId, onClose }: SettingsWindowProps) {
  const { pos, handlers } = useDraggable({ x: 240, y: 100 });
  const [lastSectionId, setLastSectionId] = useStoredState<string | undefined>(
    "settings.section",
    undefined,
    (v) => typeof v === "string"
  );
  const [selectedId, setSelectedId] = useState(initialSectionId ?? lastSectionId ?? sections[0]?.id);
  const [search, setSearch] = useState("");

  const filtered = useMemo(() => sections.filter((s) => matches(s, search.trim().toLowerCase())), [sections, search]);
  // A search that filters out the selected section shows the first match
  // instead, rather than a page the list no longer offers.
  const selected = filtered.find((s) => s.id === selectedId) ?? filtered[0];

  return (
    <div className="sp-settings-window" style={{ left: pos.x, top: pos.y }}>
      <div className="sp-settings-header" {...handlers}>
        <IconSettings size={14} />
        <span className="sp-settings-title">Options</span>
        <IconButton size={24} aria-label="Close" onClick={onClose}>
          <IconClose size={20} />
        </IconButton>
      </div>

      <div className="sp-settings-body">
        <div className="sp-settings-sidebar">
          <TextField
            placeholder="Search Settings"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
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
          {selected ? selected.content : <p className="sp-settings-empty">No settings match "{search}".</p>}
        </div>
      </div>
    </div>
  );
}
