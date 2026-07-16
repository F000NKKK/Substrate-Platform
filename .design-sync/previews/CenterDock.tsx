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

function MainEditor() {
  return (
    <div style={{
      padding: "var(--sp-space-md)",
      fontFamily: "var(--sp-font-mono)",
      fontSize: "13px",
      lineHeight: "1.6",
      color: "var(--sp-text)",
    }}>
      <span style={{ color: "var(--sp-text-muted)" }}>1</span>
      {" "}<span style={{ color: "var(--sp-accent)" }}>fn</span>{" main() {"{"}"}
      <br />
      <span style={{ color: "var(--sp-text-muted)" }}>2</span>
      {" "}&nbsp;&nbsp;&nbsp;&nbsp;<span style={{ color: "var(--sp-text-success)" }}>let</span>{" msg = "}{'"'}{"Hello"}{'"'};
      <br />
      <span style={{ color: "var(--sp-text-muted)" }}>3</span>
      {" "}&nbsp;&nbsp;&nbsp;&nbsp;<span style={{ color: "var(--sp-accent)" }}>println!</span>
      <span style={{ color: "var(--sp-text-accent)" }}>("{"{}"}!", msg)</span>;
      <br />
      <span style={{ color: "var(--sp-text-muted)" }}>4</span>
      {" "}{"}"}{"}"}
    </div>
  );
}

function SecondFile() {
  return (
    <div style={{
      padding: "var(--sp-space-md)",
      fontFamily: "var(--sp-font-mono)",
      fontSize: "13px",
      color: "var(--sp-text-muted)",
    }}>
      // lib.rs
      <br />
      #[cfg(test)]
      <br />
      mod tests {"{"}
      <br />
      &nbsp;&nbsp;#[test]
      <br />
      &nbsp;&nbsp;fn it_works() {"{"}
      <br />
      &nbsp;&nbsp;{"}"}
      <br />
      {"}"}
    </div>
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
