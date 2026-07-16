import React from "react";
import {
  PlatformShell,
  MenuBar,
  MenuBarItem,
  type PanelDef,
} from "substrate-platform-ui";

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
    println!("Hello, IDE!");
}`;

function Editor() {
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

function Explorer() {
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
      </div>
      <div style={{ color: "var(--sp-accent)" }}>Cargo.toml</div>
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
      <div>$ cargo build</div>
      <div style={{ color: "var(--sp-text-success)", marginTop: "4px" }}>
        Finished dev [unoptimized + debuginfo]
      </div>
      <div>$ cargo run</div>
      <div style={{ color: "var(--sp-text)" }}>Hello, IDE!</div>
    </div>
  );
}

function Problems() {
  return (
    <div style={{
      padding: "var(--sp-space-md)",
      color: "var(--sp-text)",
      fontSize: "13px",
    }}>
      <div style={{ color: "var(--sp-text-muted)", marginBottom: "8px" }}>
        No problems detected
      </div>
    </div>
  );
}

export function IDE() {
  const mainPanel: PanelDef = {
    id: "editor",
    title: "main.rs",
    component: Editor,
  };

  const toolWindows = {
    left: [
      {
        id: "explorer",
        title: "Explorer",
        component: Explorer,
      },
    ],
    bottom: [
      {
        id: "terminal",
        title: "Terminal",
        component: Terminal,
      },
    ],
    right: [
      {
        id: "problems",
        title: "Problems",
        component: Problems,
      },
    ],
  };

  const menu = (
    <MenuBar title="IDE">
      <MenuBarItem label="File" items={[
        { label: "New" },
        { label: "Open" },
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
        { label: "Terminal" },
        { label: "Problems" },
      ]} />
    </MenuBar>
  );

  return (
    <Surface style={{ height: 460, width: "100%" }}>
      <PlatformShell
        main={mainPanel}
        toolWindows={toolWindows}
        defaultPinned={{ left: "explorer" }}
        menu={menu}
      />
    </Surface>
  );
}
