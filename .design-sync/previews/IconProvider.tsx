import React from "react";
import { Icon, IconProvider, IconGrid } from "substrate-platform-ui";

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

/** Scoping a custom "folder" glyph to one subtree — everywhere outside the provider keeps the built-in icon. */
export function ScopedOverride() {
  return (
    <Surface>
      <div style={{ display: "flex", flexDirection: "column", alignItems: "center", gap: "var(--sp-space-sm)" }}>
        <Icon name="folder" size={24} style={{ color: "var(--sp-text)" }} />
        <span style={{ color: "var(--sp-text-muted)", fontSize: 11 }}>Default</span>
      </div>
      <IconProvider icons={{ folder: IconGrid }}>
        <div style={{ display: "flex", flexDirection: "column", alignItems: "center", gap: "var(--sp-space-sm)" }}>
          <Icon name="folder" size={24} style={{ color: "var(--sp-accent)" }} />
          <span style={{ color: "var(--sp-text-muted)", fontSize: 11 }}>Inside IconProvider override</span>
        </div>
      </IconProvider>
    </Surface>
  );
}
