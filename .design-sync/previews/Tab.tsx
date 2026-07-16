import React from "react";
import { Tab } from "substrate-platform-ui";

function Surface({ children, style }: { children: React.ReactNode; style?: React.CSSProperties }) {
  return (
    <div style={{
      background: "var(--sp-surface-1)", color: "var(--sp-text)",
      fontFamily: "var(--sp-font-ui)", fontSize: "var(--sp-font-size)",
      padding: "var(--sp-space-lg)", borderRadius: "var(--sp-radius-md)",
      display: "flex", flexWrap: "wrap", gap: "var(--sp-space-md)", alignItems: "center",
      ...style,
    }}>{children}</div>
  );
}

export function Horizontal() {
  return (
    <Surface style={{ flexDirection: "row", gap: 0 }}>
      <Tab active orientation="horizontal">Editor</Tab>
      <Tab orientation="horizontal">Terminal</Tab>
      <Tab orientation="horizontal">Output</Tab>
    </Surface>
  );
}

export function List() {
  return (
    <Surface style={{ flexDirection: "column", alignItems: "flex-start", gap: 0 }}>
      <Tab active orientation="list">General</Tab>
      <Tab orientation="list">Appearance</Tab>
      <Tab orientation="list">Keybindings</Tab>
    </Surface>
  );
}

export function Closable() {
  return (
    <Surface style={{ flexDirection: "row", gap: 0 }}>
      <Tab active orientation="horizontal" onRequestClose={() => {}}>Editor</Tab>
      <Tab orientation="horizontal" onRequestClose={() => {}}>Terminal</Tab>
    </Surface>
  );
}

export function HorizontalDock() {
  return (
    <Surface style={{ flexDirection: "row", gap: 0 }}>
      <Tab active orientation="horizontal-dock">File 1</Tab>
      <Tab orientation="horizontal-dock">File 2</Tab>
      <Tab orientation="horizontal-dock">File 3</Tab>
    </Surface>
  );
}
