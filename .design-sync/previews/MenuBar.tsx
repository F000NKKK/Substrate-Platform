import React from "react";
import { MenuBar, MenuBarItem, IconButton } from "substrate-platform-ui";
import { IconSettings } from "substrate-platform-ui/icons";

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

export function Default() {
  return (
    <Surface style={{ padding: 0, alignItems: "stretch" }}>
      <MenuBar
        title="Substrate"
        actions={
          <IconButton aria-label="Settings" size={20}>
            <IconSettings size={16} />
          </IconButton>
        }
      >
        <MenuBarItem label="File" items={[
          { label: "New File" },
          { label: "Open…" },
          { label: "Save" },
          { label: "Exit" },
        ]} />
        <MenuBarItem label="Edit" items={[
          { label: "Undo" },
          { label: "Redo" },
          { label: "Cut" },
          { label: "Copy" },
          { label: "Paste" },
        ]} />
        <MenuBarItem label="View" items={[
          { label: "Explorer" },
          { label: "Search" },
          { label: "Source Control" },
          { label: "Debug" },
        ]} />
        <MenuBarItem label="Run" items={[
          { label: "Start Debugging" },
          { label: "Run Without Debugging" },
          { label: "Stop" },
        ]} />
      </MenuBar>
    </Surface>
  );
}
