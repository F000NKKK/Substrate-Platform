import { useRef, useState, type CSSProperties, type PointerEvent as ReactPointerEvent } from "react";
import { DockStrip } from "../DockStrip/DockStrip";
import { PanelSurface } from "../PanelSurface/PanelSurface";
import { FlyoutFrame } from "../FlyoutFrame/FlyoutFrame";
import { Outline } from "../../infra/outline";
import type { ShellLayout } from "../hooks/useShellLayout";
import type { ToolWindowAnchor } from "../types";

export interface ToolWindowDockProps {
  anchor: ToolWindowAnchor;
  layout: ShellLayout;
}

const DEFAULT_FLOAT_POS = { x: 160, y: 120 };
const STRIP_VAR = "var(--sp-toolwindow-strip)";
const GAP_VAR = "var(--sp-space-xs)";

/**
 * Drag handle on one panel's edge facing the main area (or the next pinned
 * panel) — resizes that specific panel, in its current pinned/flyout mode,
 * independently of every other panel or mode. Left/right drag horizontally;
 * bottom drags its top edge upward to grow, matching how every other docking
 * IDE resizes a bottom panel. The panel resizes live on every pointermove,
 * same as dragging any other native window edge.
 */
function ResizeHandle({
  anchor,
  panelId,
  mode,
  layout,
  className,
  style,
}: {
  anchor: ToolWindowAnchor;
  panelId: string;
  mode: "pinned" | "flyout";
  layout: ShellLayout;
  className?: string;
  style?: CSSProperties;
}) {
  const [dragging, setDragging] = useState(false);

  function handlePointerDown(e: ReactPointerEvent) {
    e.preventDefault();
    const startSize = layout.anchorSize(panelId, anchor, mode);
    const startX = e.clientX;
    const startY = e.clientY;
    setDragging(true);

    function onMove(ev: PointerEvent) {
      if (anchor === "left") layout.setAnchorSize(panelId, anchor, mode, startSize + (ev.clientX - startX));
      else if (anchor === "right") layout.setAnchorSize(panelId, anchor, mode, startSize - (ev.clientX - startX));
      else layout.setAnchorSize(panelId, anchor, mode, startSize - (ev.clientY - startY));
    }
    function onUp() {
      setDragging(false);
      window.removeEventListener("pointermove", onMove);
      window.removeEventListener("pointerup", onUp);
    }
    window.addEventListener("pointermove", onMove);
    window.addEventListener("pointerup", onUp);
  }

  return (
    <div
      className={["sp-dock-resize", `sp-dock-resize--${anchor}`, className].filter(Boolean).join(" ")}
      style={style}
      data-dragging={dragging || undefined}
      onPointerDown={handlePointerDown}
    >
      <div className="sp-dock-resize-line" />
    </div>
  );
}

function sizeStyle(anchor: ToolWindowAnchor, size: number): CSSProperties {
  return anchor === "bottom" ? { height: size } : { width: size };
}

/**
 * Where a flyout's own resize handle sits — its main-area-facing edge, i.e.
 * the strip-side offset plus this panel's own current size. Computed
 * in-line rather than via CSS descendant selectors because the handle can't
 * live inside `.sp-dock-flyout` (that box clips anything poking past its own
 * edge, which the handle's hit area deliberately does).
 */
function flyoutHandlePositionStyle(anchor: ToolWindowAnchor, size: number): CSSProperties {
  const offsetExpr = `calc(${STRIP_VAR} + ${GAP_VAR} + ${size}px)`;
  const base: CSSProperties = { position: "absolute" };
  if (anchor === "left") return { ...base, left: offsetExpr, top: 0, bottom: 0 };
  if (anchor === "right") return { ...base, right: offsetExpr, top: 0, bottom: 0 };
  return { ...base, bottom: offsetExpr, left: 0, right: 0 };
}

/**
 * One edge's worth of VS-style tool windows. The collapsed tab strip always
 * sits at the true window edge; any panels pinned to this anchor sit between
 * it and the main area as their own slots, side by side — pinning a second one
 * does not evict the first, and each remembers its own size independently of
 * every other panel and of its own flyout size. At most one more panel can be
 * peeking open as a flyout, which floats OVER everything — at the exact same
 * offset from the strip a pinned panel sits at, so nothing visibly jumps when
 * a panel is pinned or unpinned — right next to the strip.
 *
 * Each pinned panel is a plain flex slot: the panel fills it, its own resize
 * handle overlaps the main-area-facing edge (a negative margin, so it adds no
 * layout width), and an `Outline` traces the panel. Because the handle lives
 * inside the same slot as the panel it resizes, a handle can never end up
 * controlling a different panel — the failure the earlier absolute-position +
 * separate-spacer scheme was prone to. Resize stays jitter-free without that
 * scheme because `Outline`'s `syncWith` recomputes the outline synchronously
 * in the same layout pass as the flex reflow, not a frame later.
 */
export function ToolWindowDock({ anchor, layout }: ToolWindowDockProps) {
  const panelIds = layout.idsByAnchor(anchor);
  const pinnedIds = layout.pinnedInAnchor(anchor);
  const flyoutId = layout.flyoutInAnchor(anchor);
  const flyoutPanel = flyoutId ? layout.panelsById[flyoutId] : null;

  const regionRef = useRef<HTMLDivElement>(null);
  const panelRefs = useRef<Record<string, HTMLDivElement | null>>({});

  if (panelIds.length === 0) return null;

  // A pinned panel's own header already shows its title, so it doesn't also
  // need a redundant tab in the strip. A flyout's tab, on the other hand,
  // stays in the strip — it's the notched part of the continuous outline.
  const stripIds = panelIds.filter((id) => !pinnedIds.includes(id));

  // The handle always sits on the panel's main-area-facing edge — for "left"
  // that's after the panel (strip, then panel, then main area), but "right"
  // and "bottom" render [pinned panels, strip] so their main area is on the
  // OPPOSITE side from the strip, putting the handle before the panel instead.
  const handleFirst = anchor !== "left";

  const pinnedSlots = pinnedIds.map((id) => {
    const panel = layout.panelsById[id];
    const Component = panel?.component;
    if (!panel || !Component) return null;
    const size = layout.anchorSize(id, anchor, "pinned");
    const handleEl = <ResizeHandle key="handle" anchor={anchor} panelId={id} mode="pinned" layout={layout} />;
    return (
      <div key={id} className={`sp-dock-pinned-slot sp-dock-pinned-slot--${anchor}`} style={sizeStyle(anchor, size)}>
        {handleFirst && handleEl}
        <PanelSurface
          ref={(el) => {
            panelRefs.current[id] = el;
          }}
          panelId={id}
          title={panel.title}
          pinned
          onTogglePin={() => layout.unpin(id)}
          onFloat={() => layout.floatAt(id, DEFAULT_FLOAT_POS.x, DEFAULT_FLOAT_POS.y)}
          onClose={() => layout.close(id)}
        >
          <Component />
        </PanelSurface>
        {!handleFirst && handleEl}
        <Outline regionRef={regionRef} targets={[() => panelRefs.current[id] ?? null]} className="sp-pinned-outline" syncWith={size} />
      </div>
    );
  });

  const strip = (
    <DockStrip
      anchor={anchor}
      panelIds={stripIds}
      panelsById={layout.panelsById}
      activeId={flyoutId}
      onSelect={layout.toggle}
      onDropPanel={layout.dockTo}
    />
  );

  return (
    <div className={`sp-dock sp-dock--${anchor}`} ref={regionRef}>
      {anchor === "left" && strip}
      {pinnedSlots}
      {anchor !== "left" && strip}

      {flyoutPanel && (
        <FlyoutFrame
          key={flyoutId}
          anchor={anchor}
          regionRef={regionRef}
          size={layout.anchorSize(flyoutId!, anchor, "flyout")}
          onOutsideClick={() => layout.close(flyoutId!)}
          resizeHandle={
            <ResizeHandle
              anchor={anchor}
              panelId={flyoutId!}
              mode="flyout"
              layout={layout}
              style={flyoutHandlePositionStyle(anchor, layout.anchorSize(flyoutId!, anchor, "flyout"))}
            />
          }
        >
          <PanelSurface
            panelId={flyoutId!}
            title={flyoutPanel.title}
            pinned={false}
            onTogglePin={() => layout.pin(flyoutId!)}
            onFloat={() => layout.floatAt(flyoutId!, DEFAULT_FLOAT_POS.x, DEFAULT_FLOAT_POS.y)}
            onClose={() => layout.close(flyoutId!)}
          >
            {(() => {
              const Component = flyoutPanel.component;
              return <Component />;
            })()}
          </PanelSurface>
        </FlyoutFrame>
      )}
    </div>
  );
}
