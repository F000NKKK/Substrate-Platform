import React from "react";
import { Label, TextField } from "substrate-platform-ui";

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
    <Surface style={{ flexDirection: "column", alignItems: "flex-start" }}>
      <Label>Accent Color</Label>
      <span style={{ color: "var(--sp-text-secondary)" }}>A vibrant blue hue</span>
    </Surface>
  );
}

export function WithField() {
  return (
    <Surface style={{ flexDirection: "column", alignItems: "flex-start", gap: "var(--sp-space-sm)" }}>
      <Label>Search Files</Label>
      <TextField placeholder="Search files…" style={{ width: 200 }} />
    </Surface>
  );
}

export function Multiple() {
  return (
    <Surface style={{ flexDirection: "column", alignItems: "flex-start", gap: "var(--sp-space-lg)" }}>
      <div>
        <Label>Font Family</Label>
        <span style={{ color: "var(--sp-text-secondary)" }}>System Sans</span>
      </div>
      <div>
        <Label>Font Size</Label>
        <span style={{ color: "var(--sp-text-secondary)" }}>14px</span>
      </div>
    </Surface>
  );
}
