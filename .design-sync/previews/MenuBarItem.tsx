import React from "react";
import { MenuBarItem, MenuBar } from "substrate-platform-ui";

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

export function Simple() {
  return (
    <Surface style={{ padding: 0, alignItems: "stretch" }}>
      <MenuBar>
        <MenuBarItem label="File" onClick={() => console.log("File clicked")} />
      </MenuBar>
    </Surface>
  );
}

export function WithDropdown() {
  return (
    <Surface style={{ padding: 0, alignItems: "stretch" }}>
      <MenuBar>
        <MenuBarItem
          label="File"
          items={[
            { label: "New File", onClick: () => console.log("New File") },
            { label: "Open…", onClick: () => console.log("Open") },
            { label: "Save", onClick: () => console.log("Save") },
            { label: "Close", onClick: () => console.log("Close") },
          ]}
        />
      </MenuBar>
    </Surface>
  );
}
