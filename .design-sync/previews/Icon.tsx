import React from "react";
import { Icon } from "substrate-platform-ui";

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

/** Resolves a built-in glyph by its registry name — the by-name counterpart to importing an `Icon*` component directly. */
export function ByName() {
  return (
    <Surface>
      {["folder", "file", "chevronRight", "pin", "close", "settings"].map((name) => (
        <div key={name} style={{ display: "flex", flexDirection: "column", alignItems: "center", gap: "var(--sp-space-sm)" }}>
          <Icon name={name} size={22} style={{ color: "var(--sp-text)" }} />
          <span style={{ color: "var(--sp-text-muted)", fontSize: 11 }}>{name}</span>
        </div>
      ))}
    </Surface>
  );
}

/** An unregistered name renders nothing rather than throwing — the surrounding UI keeps working. */
export function UnknownName() {
  return (
    <Surface>
      <Icon name="doesNotExist" size={22} style={{ color: "var(--sp-text)" }} />
      <span style={{ color: "var(--sp-text-muted)", fontSize: 11 }}>Unregistered name → renders nothing, logs a warning</span>
    </Surface>
  );
}
