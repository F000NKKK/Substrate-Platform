import React from "react";
import { FloatingPanel, type PanelDef } from "substrate-platform-ui";

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

function TerminalContent() {
  return (
    <div style={{
      padding: "var(--sp-space-md)",
      fontFamily: "var(--sp-font-mono)",
      fontSize: "12px",
      color: "var(--sp-text-muted)",
    }}>
      <div>$ cargo test</div>
      <div style={{ color: "var(--sp-text-success)", marginTop: "4px" }}>
        test result: ok. 12 passed
      </div>
    </div>
  );
}

export function Floating() {
  const terminalPanel: PanelDef = {
    id: "output",
    title: "Output",
    component: TerminalContent,
  };

  return (
    <Surface style={{ position: "relative", minHeight: 300, minWidth: 420 }}>
      <FloatingPanel
        panel={terminalPanel}
        x={20}
        y={20}
        w={360}
        h={220}
        onDock={() => {}}
        onClose={() => {}}
      />
    </Surface>
  );
}
