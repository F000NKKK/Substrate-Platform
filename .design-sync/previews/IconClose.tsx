import React from "react";
import { IconClose } from "substrate-platform-ui";

function Surface({ children }: { children: React.ReactNode }) {
  return (
    <div style={{
      background: "var(--sp-surface-1)", color: "var(--sp-text)",
      fontFamily: "var(--sp-font-ui)", fontSize: "var(--sp-font-size)",
      padding: "var(--sp-space-lg)", borderRadius: "var(--sp-radius-md)",
      display: "flex", gap: "var(--sp-space-lg)", alignItems: "center",
    }}>{children}</div>
  );
}

export function Sizes() {
  return (
    <Surface>
      <div style={{ display: "flex", flexDirection: "column", alignItems: "center", gap: "var(--sp-space-sm)" }}>
        <IconClose size={16} style={{ color: "var(--sp-text)" }} />
        <span style={{ color: "var(--sp-text-muted)", fontSize: 11 }}>16</span>
      </div>
      <div style={{ display: "flex", flexDirection: "column", alignItems: "center", gap: "var(--sp-space-sm)" }}>
        <IconClose size={24} style={{ color: "var(--sp-text)" }} />
        <span style={{ color: "var(--sp-text-muted)", fontSize: 11 }}>24</span>
      </div>
      <div style={{ display: "flex", flexDirection: "column", alignItems: "center", gap: "var(--sp-space-sm)" }}>
        <IconClose size={32} style={{ color: "var(--sp-text)" }} />
        <span style={{ color: "var(--sp-text-muted)", fontSize: 11 }}>32</span>
      </div>
    </Surface>
  );
}

export function Accent() {
  return (
    <Surface>
      <IconClose size={28} style={{ color: "var(--sp-accent)" }} />
    </Surface>
  );
}
