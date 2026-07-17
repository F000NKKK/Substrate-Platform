import React from "react";
import { WindowControls } from "substrate-platform-ui";

/** The minimize/maximize/close cluster for a `decorations: false` Tauri window — outside Tauri the buttons render but no-op. */
export function TitleBar() {
  return (
    <div style={{
      background: "var(--sp-surface-1)", padding: "0 var(--sp-space-sm)",
      height: 38, display: "flex", alignItems: "center", justifyContent: "flex-end",
      borderRadius: "var(--sp-radius-md)", width: 320,
    }}>
      <WindowControls />
    </div>
  );
}
