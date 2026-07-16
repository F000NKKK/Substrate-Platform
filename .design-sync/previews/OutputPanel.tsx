import React from "react";
import { OutputPanel } from "substrate-platform-ui";

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

export function WithLogs() {
  const logs = [
    "› cargo build --release",
    "   Compiling substrate-core v0.1.0",
    "    Compiling substrate-ui v0.2.0",
    "     Finished release [optimized] target(s) in 4.21s",
    "› running 3 tests",
    "test shell::dock::redock ... ok",
    "test ui::theme::accent ... ok",
    "test output::panel::streaming ... ok",
  ];

  return (
    <Surface style={{ padding: 0, minHeight: 220 }}>
      <OutputPanel lines={logs} />
    </Surface>
  );
}

export function Empty() {
  return (
    <Surface style={{ padding: 0, minHeight: 220 }}>
      <OutputPanel emptyLabel="No output yet." />
    </Surface>
  );
}
