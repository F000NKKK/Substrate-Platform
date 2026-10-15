import type { DockAnchor } from "../types";

/**
 * Where a dragged panel would go — the one docking mechanism for every
 * drag (a tab from a dock strip or the center, a floating window's title
 * bar): a compass of guides at the middle of the main area (center: a tab
 * there; left/right/bottom: that edge), and bands along the edges that do
 * the same when the pointer's simply near one. `null`: neither — the panel
 * floats where it's dropped.
 */
export function dockTargetAt(main: DOMRect, clientX: number, clientY: number): DockAnchor | null {
  const cx = main.left + main.width / 2;
  const cy = main.top + main.height / 2;
  for (const [anchor, { dx, dy }] of Object.entries(GUIDE_OFFSETS) as [DockAnchor, { dx: number; dy: number }][]) {
    if (Math.abs(clientX - (cx + dx)) <= GUIDE_SIZE / 2 && Math.abs(clientY - (cy + dy)) <= GUIDE_SIZE / 2) return anchor;
  }
  if (clientX < main.left || clientX > main.right || clientY < main.top || clientY > main.bottom) return null;
  const band = Math.min(EDGE_BAND, main.width / 4, main.height / 4);
  if (clientX - main.left < band) return "left";
  if (main.right - clientX < band) return "right";
  if (main.bottom - clientY < band) return "bottom";
  return null;
}

/** The guides' size and spacing (px), and how near an edge docks to it. */
const GUIDE_SIZE = 44;
const GUIDE_STEP = 52;
const EDGE_BAND = 56;

const GUIDE_OFFSETS: Record<DockAnchor, { dx: number; dy: number }> = {
  center: { dx: 0, dy: 0 },
  left: { dx: -GUIDE_STEP, dy: 0 },
  right: { dx: GUIDE_STEP, dy: 0 },
  bottom: { dx: 0, dy: GUIDE_STEP },
};

/** The part of the main area `anchor` docks into — the preview under the pointer. */
function previewRect(main: DOMRect, anchor: DockAnchor) {
  const edge = 0.3;
  if (anchor === "left") return { left: main.left, top: main.top, width: main.width * edge, height: main.height };
  if (anchor === "right") return { left: main.right - main.width * edge, top: main.top, width: main.width * edge, height: main.height };
  if (anchor === "bottom") return { left: main.left, top: main.bottom - main.height * edge, width: main.width, height: main.height * edge };
  return { left: main.left, top: main.top, width: main.width, height: main.height };
}

/** The guides over `main` while a panel is dragged, `hover` (from `dockTargetAt`) previewed. */
export function DockGuides({ main, hover }: { main: DOMRect; hover: DockAnchor | null }) {
  return (
    <>
      {hover && <div key={hover} className="sp-dock-preview" style={previewRect(main, hover)} />}
      <div className="sp-dock-guides" style={{ left: main.left + main.width / 2, top: main.top + main.height / 2 }}>
        {(Object.keys(GUIDE_OFFSETS) as DockAnchor[]).map((anchor) => (
          <div
            key={anchor}
            className={`sp-dock-guide-button sp-dock-guide-button--${anchor}`}
            data-hover={hover === anchor || undefined}
            style={{
              width: GUIDE_SIZE,
              height: GUIDE_SIZE,
              left: GUIDE_OFFSETS[anchor].dx - GUIDE_SIZE / 2,
              top: GUIDE_OFFSETS[anchor].dy - GUIDE_SIZE / 2,
            }}
          />
        ))}
      </div>
    </>
  );
}
