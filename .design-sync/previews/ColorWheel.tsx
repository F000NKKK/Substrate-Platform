import React from "react";
import { ColorWheel } from "substrate-platform-ui";

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
  const [color, setColor] = React.useState({ h: 160, s: 0.6, l: 0.5 });

  return (
    <Surface>
      <ColorWheel
        hue={color.h}
        saturation={color.s}
        lightness={color.l}
        onChange={(h, s) => setColor({ h, s, l: color.l })}
      />
    </Surface>
  );
}

export function Large() {
  const [color, setColor] = React.useState({ h: 210, s: 0.7, l: 0.5 });

  return (
    <Surface>
      <ColorWheel
        hue={color.h}
        saturation={color.s}
        lightness={color.l}
        size={220}
        onChange={(h, s) => setColor({ h, s, l: color.l })}
      />
    </Surface>
  );
}

export function Interactive() {
  const [color, setColor] = React.useState({ h: 42, s: 0.85, l: 0.55 });

  return (
    <Surface>
      <div style={{ display: "flex", flexDirection: "column", gap: "var(--sp-space-md)", alignItems: "center" }}>
        <ColorWheel
          hue={color.h}
          saturation={color.s}
          lightness={color.l}
          onChange={(h, s) => setColor({ h, s, l: color.l })}
        />
        <div style={{
          width: 40,
          height: 40,
          borderRadius: "50%",
          background: `hsl(${color.h} ${color.s * 100}% ${color.l * 100}%)`,
          border: "2px solid var(--sp-border)",
        }} />
      </div>
    </Surface>
  );
}
