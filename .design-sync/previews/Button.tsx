import React from "react";
import { Button } from "substrate-platform-ui";

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

export function Primary() {
  return (
    <Surface>
      <Button variant="primary">Save</Button>
    </Surface>
  );
}

export function Subtle() {
  return (
    <Surface>
      <Button variant="subtle">Cancel</Button>
    </Surface>
  );
}

export function Ghost() {
  return (
    <Surface>
      <Button variant="ghost">Reset</Button>
    </Surface>
  );
}

export function Disabled() {
  return (
    <Surface>
      <Button variant="primary" disabled>Save (disabled)</Button>
    </Surface>
  );
}

export function Variants() {
  return (
    <Surface>
      <Button variant="primary">Save</Button>
      <Button variant="subtle">Cancel</Button>
      <Button variant="ghost">Reset</Button>
    </Surface>
  );
}
