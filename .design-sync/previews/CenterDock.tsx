import React from "react";
import { CenterDock, useShellLayout, type PanelDef } from "substrate-platform-ui";

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

const MAIN_RS = `fn main() {
    let msg = "Hello";
    println!("{}!", msg);
}`;

function MainEditor() {
  return (
    <pre style={{
      margin: 0,
      padding: "var(--sp-space-md)",
      fontFamily: "var(--sp-font-mono)",
      fontSize: "13px",
      lineHeight: "1.6",
      color: "var(--sp-text)",
    }}>{MAIN_RS}</pre>
  );
}

export function Default() {
  const mainPanel: PanelDef = {
    id: "main.rs",
    title: "main.rs",
    component: MainEditor,
  };

  const layout = useShellLayout(mainPanel, {});

  return (
    <Surface style={{ height: 320, width: "100%" }}>
      <CenterDock layout={layout} />
    </Surface>
  );
}
