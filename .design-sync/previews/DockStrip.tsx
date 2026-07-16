import React from "react";
import { DockStrip, type PanelDef } from "substrate-platform-ui";

function Surface({ children, style }: { children: React.ReactNode; style?: React.CSSProperties }) {
  return (
    <div style={{
      background: "var(--sp-surface-0)",
      color: "var(--sp-text)",
      fontFamily: "var(--sp-font-ui)",
      fontSize: "var(--sp-font-size)",
      borderRadius: "var(--sp-radius-md)",
      overflow: "hidden",
      ...style,
    }}>{children}</div>
  );
}

const panelDefs: Record<string, PanelDef> = {
  explorer: { id: "explorer", title: "Explorer", component: () => null },
  search: { id: "search", title: "Search", component: () => null },
  sourceControl: { id: "source-control", title: "Source Control", component: () => null },
};

export function Default() {
  return (
    <Surface style={{ width: 64, height: 280 }}>
      <DockStrip
        anchor="left"
        panelIds={["explorer", "search", "source-control"]}
        panelsById={panelDefs}
        activeId="explorer"
        onSelect={() => {}}
        onDropPanel={() => {}}
      />
    </Surface>
  );
}

export function Horizontal() {
  return (
    <Surface style={{ width: 100, height: 64 }}>
      <DockStrip
        anchor="bottom"
        panelIds={["explorer", "search"]}
        panelsById={panelDefs}
        activeId={null}
        onSelect={() => {}}
        onDropPanel={() => {}}
      />
    </Surface>
  );
}
