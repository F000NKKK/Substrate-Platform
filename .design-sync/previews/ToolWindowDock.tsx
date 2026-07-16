import React from "react";
import { ToolWindowDock, useShellLayout, type PanelDef } from "substrate-platform-ui";

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

function MainEditor() {
  return (
    <div style={{
      padding: "var(--sp-space-md)",
      fontFamily: "var(--sp-font-mono)",
      fontSize: "13px",
      color: "var(--sp-text)",
    }}>
      fn main() {"{"}{"}"}
    </div>
  );
}

function Explorer() {
  return (
    <div style={{
      padding: "var(--sp-space-md)",
      color: "var(--sp-text)",
      fontSize: "13px",
    }}>
      <div style={{ color: "var(--sp-accent)", marginBottom: "4px" }}>src/</div>
      <div style={{ paddingLeft: "20px" }}>main.rs</div>
      <div>Cargo.toml</div>
    </div>
  );
}

function Terminal() {
  return (
    <div style={{
      padding: "var(--sp-space-md)",
      fontFamily: "var(--sp-font-mono)",
      fontSize: "12px",
      color: "var(--sp-text-muted)",
    }}>
      $ cargo run
      <br />
      <span style={{ color: "var(--sp-text-success)" }}>Hello, world!</span>
    </div>
  );
}

export function LeftDock() {
  const mainPanel: PanelDef = {
    id: "editor",
    title: "main.rs",
    component: MainEditor,
  };

  const layout = useShellLayout(mainPanel, {
    left: [
      {
        id: "explorer",
        title: "Explorer",
        component: Explorer,
      },
    ],
  }, { left: "explorer" });

  return (
    <Surface style={{ height: 260, width: "100%" }}>
      <ToolWindowDock anchor="left" layout={layout} />
    </Surface>
  );
}

export function BottomDock() {
  const mainPanel: PanelDef = {
    id: "editor",
    title: "main.rs",
    component: MainEditor,
  };

  const layout = useShellLayout(mainPanel, {
    bottom: [
      {
        id: "terminal",
        title: "Terminal",
        component: Terminal,
      },
    ],
  }, { bottom: "terminal" });

  return (
    <Surface style={{ height: 180, width: "100%" }}>
      <ToolWindowDock anchor="bottom" layout={layout} />
    </Surface>
  );
}

export function RightDock() {
  const mainPanel: PanelDef = {
    id: "editor",
    title: "main.rs",
    component: MainEditor,
  };

  const layout = useShellLayout(mainPanel, {
    right: [
      {
        id: "search",
        title: "Search",
        component: () => (
          <div style={{ padding: "var(--sp-space-md)", color: "var(--sp-text-muted)" }}>
            No results
          </div>
        ),
      },
    ],
  });

  return (
    <Surface style={{ height: 260, width: "100%" }}>
      <ToolWindowDock anchor="right" layout={layout} />
    </Surface>
  );
}
