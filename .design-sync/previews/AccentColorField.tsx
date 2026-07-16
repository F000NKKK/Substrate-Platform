import React from "react";
import { AccentColorField } from "substrate-platform-ui";

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
    <Surface>
      <AccentColorField />
    </Surface>
  );
}
