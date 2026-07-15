import { useMemo, useState, type ReactNode } from "react";
import { IconButton, Tab, TextField, useDraggable } from "../widgets";
import { IconClose, IconSettings } from "../icons";
import "./SettingsWindow.css";

export interface SettingsSection {
  id: string;
  label: string;
  content: ReactNode;
}

export interface SettingsWindowProps {
  sections: SettingsSection[];
  initialSectionId?: string;
  onClose: () => void;
}

/**
 * A real Options-style window — its own title bar (draggable, closable),
 * a searchable category list on the left, and the selected category's
 * settings on the right. Any product wires its own sections in; the shell
 * (this window) is the one reusable piece.
 */
export function SettingsWindow({ sections, initialSectionId, onClose }: SettingsWindowProps) {
  const { pos, handlers } = useDraggable({ x: 240, y: 100 });
  const [selectedId, setSelectedId] = useState(initialSectionId ?? sections[0]?.id);
  const [search, setSearch] = useState("");

  const filtered = useMemo(
    () => sections.filter((s) => s.label.toLowerCase().includes(search.toLowerCase())),
    [sections, search]
  );
  const selected = sections.find((s) => s.id === selectedId) ?? filtered[0];

  return (
    <div className="sp-settings-window" style={{ left: pos.x, top: pos.y }}>
      <div className="sp-settings-header" {...handlers}>
        <IconSettings size={14} />
        <span className="sp-settings-title">Options</span>
        <IconButton size={20} aria-label="Close" onClick={onClose}>
          <IconClose size={13} />
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
                onClick={() => setSelectedId(section.id)}
              >
                {section.label}
              </Tab>
            ))}
          </div>
        </div>
        <div className="sp-settings-detail">{selected?.content}</div>
      </div>
    </div>
  );
}
