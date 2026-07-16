import React from "react";
import {
  IconButton,
  IconClose,
  IconSettings,
  IconPin,
  IconFloat,
  IconTerminal,
} from "substrate-platform-ui";

function Surface({ children, style }: { children: React.ReactNode; style?: React.CSSProperties }) {
  return (
    <div style={{
      background: "var(--sp-surface-1)", color: "var(--sp-text)",
      fontFamily: "var(--sp-font-ui)", fontSize: "var(--sp-font-size)",
      padding: "var(--sp-space-lg)", borderRadius: "var(--sp-radius-md)",
      display: "flex", flexWrap: "wrap", gap: "var(--sp-space-md)", alignItems: "center",
      color: "var(--sp-text)",
      ...style,
    }}>{children}</div>
  );
}

export function Default() {
  return (
    <Surface>
      <IconButton aria-label="Close">
        <IconClose size={18} style={{ color: "var(--sp-text)" }} />
      </IconButton>
    </Surface>
  );
}

export function Sizes() {
  return (
    <Surface>
      <IconButton size={22} aria-label="Close (small)">
        <IconClose size={14} style={{ color: "var(--sp-text)" }} />
      </IconButton>
      <IconButton size={28} aria-label="Close (default)">
        <IconClose size={18} style={{ color: "var(--sp-text)" }} />
      </IconButton>
      <IconButton size={36} aria-label="Close (large)">
        <IconClose size={24} style={{ color: "var(--sp-text)" }} />
      </IconButton>
    </Surface>
  );
}

export function Toolbar() {
  return (
    <Surface>
      <IconButton size={30} aria-label="Pin panel">
        <IconPin size={18} style={{ color: "var(--sp-text)" }} />
      </IconButton>
      <IconButton size={30} aria-label="Float panel">
        <IconFloat size={18} style={{ color: "var(--sp-text)" }} />
      </IconButton>
      <IconButton size={30} aria-label="Open terminal">
        <IconTerminal size={18} style={{ color: "var(--sp-text)" }} />
      </IconButton>
      <IconButton size={30} aria-label="Settings">
        <IconSettings size={18} style={{ color: "var(--sp-text)" }} />
      </IconButton>
      <IconButton size={30} aria-label="Close">
        <IconClose size={18} style={{ color: "var(--sp-text)" }} />
      </IconButton>
    </Surface>
  );
}
