import React from "react";
import { IconButton } from "substrate-platform-ui";
import { IconClose, IconSettings } from "substrate-platform-ui/icons";

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
      <IconButton aria-label="Close">
        <IconClose />
      </IconButton>
    </Surface>
  );
}

export function Sizes() {
  return (
    <Surface>
      <IconButton size={16} aria-label="Close (small)">
        <IconClose size={14} />
      </IconButton>
      <IconButton size={20} aria-label="Close (default)">
        <IconClose size={16} />
      </IconButton>
      <IconButton size={28} aria-label="Close (large)">
        <IconClose size={24} />
      </IconButton>
    </Surface>
  );
}

export function WithSettings() {
  return (
    <Surface>
      <IconButton aria-label="Settings">
        <IconSettings />
      </IconButton>
    </Surface>
  );
}
