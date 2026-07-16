import React from "react";
import { ColorPickerWindow } from "substrate-platform-ui";

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

export function Open() {
  const [color, setColor] = React.useState({ h: 210, s: 70, l: 50 });
  const presets = [
    { h: 160, s: 60, l: 45 },
    { h: 210, s: 70, l: 50 },
    { h: 15, s: 75, l: 52 },
    { h: 42, s: 85, l: 55 },
  ];

  return (
    <Surface style={{ position: "relative", minHeight: 360, minWidth: 300 }}>
      <ColorPickerWindow
        title="Select Accent Color"
        value={color}
        presets={presets}
        defaultValue={{ h: 210, s: 70, l: 50 }}
        onChange={setColor}
        onClose={() => {}}
      />
    </Surface>
  );
}
