import React from "react";
import { PanelSurface } from "substrate-platform-ui";

function Surface({ children, style }: { children: React.ReactNode; style?: React.CSSProperties }) {
  return (
    <div style={{
      background: "var(--sp-surface-0)",
      color: "var(--sp-text)",
      fontFamily: "var(--sp-font-ui)",
      fontSize: "var(--sp-font-size)",
      borderRadius: "var(--sp-radius-md)",
      overflow: "hidden",
      ...style,
    }}>{children}</div>
  );
}

function ExplorerContent() {
  return (
    <div style={{
      padding: "var(--sp-space-md)",
      color: "var(--sp-text)",
      fontSize: "13px",
    }}>
      <div style={{ color: "var(--sp-accent)", marginBottom: "4px" }}>src/</div>
      <div style={{ paddingLeft: "20px", marginBottom: "8px" }}>
        <div>main.rs</div>
        <div>lib.rs</div>
        <div>shell.rs</div>
      </div>
      <div style={{ color: "var(--sp-accent)" }}>Cargo.toml</div>
      <div style={{ color: "var(--sp-accent)" }}>README.md</div>
    </div>
  );
}

export function Pinned() {
  return (
    <Surface style={{ width: 320, height: 240 }}>
      <PanelSurface
        panelId="explorer"
        title="Explorer"
        pinned={true}
        onTogglePin={() => {}}
        onClose={() => {}}
        onFloat={() => {}}
      >
        <ExplorerContent />
      </PanelSurface>
    </Surface>
  );
}

export function Unpinned() {
  return (
    <Surface style={{ width: 320, height: 240 }}>
      <PanelSurface
        panelId="explorer"
        title="Explorer"
        pinned={false}
        onTogglePin={() => {}}
        onClose={() => {}}
        onFloat={() => {}}
      >
        <ExplorerContent />
      </PanelSurface>
    </Surface>
  );
}
