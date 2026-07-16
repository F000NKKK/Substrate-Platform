import React from "react";
import { SettingsWindow, AppearanceSettings, Label, TextField } from "substrate-platform-ui";

function Surface({ children, style }: { children: React.ReactNode; style?: React.CSSProperties }) {
  return (
    <div style={{
      background: "var(--sp-surface-1)", color: "var(--sp-text)",
      fontFamily: "var(--sp-font-ui)", fontSize: "var(--sp-font-size)",
      padding: "var(--sp-space-lg)", borderRadius: "var(--sp-radius-md)",
      display: "flex", flexDirection: "column", gap: "var(--sp-space-md)", alignItems: "stretch",
      ...style,
    }}>{children}</div>
  );
}

function SettingsSection({ children, style }: { children: React.ReactNode; style?: React.CSSProperties }) {
  return (
    <div style={{
      display: "flex", flexDirection: "column", gap: "var(--sp-space-md)",
      ...style,
    }}>
      {children}
    </div>
  );
}

export function Open() {
  const sections = [
    {
      id: "appearance",
      label: "Appearance",
      content: <AppearanceSettings />,
    },
    {
      id: "editor",
      label: "Editor",
      content: (
        <SettingsSection>
          <div>
            <Label>Font Family</Label>
            <TextField value="Fira Code" readOnly />
          </div>
          <div>
            <Label>Font Size</Label>
            <TextField value="12px" readOnly />
          </div>
          <div>
            <Label>Line Height</Label>
            <TextField value="1.5" readOnly />
          </div>
        </SettingsSection>
      ),
    },
    {
      id: "terminal",
      label: "Terminal",
      content: (
        <SettingsSection>
          <div>
            <Label>Shell</Label>
            <TextField value="/bin/bash" readOnly />
          </div>
          <div>
            <Label>Font Size</Label>
            <TextField value="11px" readOnly />
          </div>
        </SettingsSection>
      ),
    },
    {
      id: "keybindings",
      label: "Keybindings",
      content: (
        <SettingsSection>
          <div>
            <Label>Edit keybindings.json</Label>
            <TextField value="Click to open file" readOnly />
          </div>
        </SettingsSection>
      ),
    },
  ];

  return (
    <Surface style={{ position: "relative", minHeight: 420, minWidth: 560, padding: 0 }}>
      <SettingsWindow
        sections={sections}
        initialSectionId="appearance"
        onClose={() => console.log("Settings closed")}
      />
    </Surface>
  );
}
