import React from "react";
import { Swatch } from "substrate-platform-ui";

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

export function Palette() {
  return (
    <Surface>
      <Swatch color={{ h: 160, s: 60, l: 45 }} />
      <Swatch color={{ h: 210, s: 70, l: 50 }} active />
      <Swatch color={{ h: 15, s: 75, l: 52 }} />
      <Swatch color={{ h: 42, s: 85, l: 55 }} />
      <Swatch color={{ h: 280, s: 65, l: 48 }} />
    </Surface>
  );
}

export function Single() {
  return (
    <Surface>
      <Swatch color={{ h: 210, s: 70, l: 50 }} active />
    </Surface>
  );
}

export function Inactive() {
  return (
    <Surface>
      <Swatch color={{ h: 160, s: 60, l: 45 }} />
      <Swatch color={{ h: 15, s: 75, l: 52 }} />
    </Surface>
  );
}
