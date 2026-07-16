import React from "react";
import { TextField } from "substrate-platform-ui";

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

export function Default() {
  return (
    <Surface>
      <TextField placeholder="Search files…" />
    </Surface>
  );
}

export function WithValue() {
  return (
    <Surface>
      <TextField defaultValue="example.tsx" />
    </Surface>
  );
}

export function Disabled() {
  return (
    <Surface>
      <TextField placeholder="Disabled input" disabled />
    </Surface>
  );
}

export function Multiple() {
  return (
    <Surface style={{ flexDirection: "column", alignItems: "stretch", gap: "var(--sp-space-md)" }}>
      <TextField placeholder="Name" />
      <TextField placeholder="Email" type="email" />
      <TextField placeholder="Message" />
    </Surface>
  );
}
