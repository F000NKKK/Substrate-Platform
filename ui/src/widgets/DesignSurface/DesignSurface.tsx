import { useRef, useState, type CSSProperties, type DragEvent, type KeyboardEvent as ReactKeyboardEvent, type MouseEvent as ReactMouseEvent, type PointerEvent as ReactPointerEvent, type ReactNode } from "react";
import "./DesignSurface.css";

/** A rectangle in design units (the designed thing's own pixels, before `scale`). */
export interface DesignRect {
  x: number;
  y: number;
  w: number;
  h: number;
}

export interface DesignPoint {
  x: number;
  y: number;
}

export type DesignHandle = "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw";

const ALL_HANDLES: DesignHandle[] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];
const SIZE_HANDLES: DesignHandle[] = ["e", "se", "s"];

/**
 * One selectable object on a `DesignSurface` — a control on a form, a shape
 * on a diagram. The surface only knows geometry and hierarchy; what an item
 * looks like is whatever the product renders as the surface's `children`.
 */
export interface DesignItem {
  id: string;
  /** Where it is, in design units, relative to the surface's origin. */
  rect: DesignRect;
  /** The container it sits in — its siblings and that container's `contentRect` are what it snaps to. */
  parentId?: string | null;
  /** Accepts children: a drop (an item moved onto it, or something dragged in from outside) can target it. */
  container?: boolean;
  /** A container's inner box (inside its padding), for snapping — defaults to `rect`. */
  contentRect?: DesignRect;
  /**
   * `"free"` — dragged anywhere, snapping to siblings and the grid (WinForms'
   * absolute layout). `"flow"` — placed by its container (a stack panel): it
   * can still be dragged, and the drop is reported with the pointer position
   * so the product can work out where it goes. `"fixed"` — never moves (the
   * root form); dragging on it draws a selection rectangle instead.
   */
  placement: "free" | "flow" | "fixed";
  /** Resize handles offered while this is the primary selection. Default: all eight for `"free"`, right/bottom for the others. `[]` = not resizable. */
  handles?: DesignHandle[];
  /** Shown as the item's tooltip. */
  label?: string;
}

/** Items dragged to new places — `rects` holds every moved item's new rect; `target` is the container under the pointer (`null` over nothing). */
export interface DesignMove {
  kind: "move";
  ids: string[];
  rects: Record<string, DesignRect>;
  target: string | null;
  point: DesignPoint;
}

export interface DesignResize {
  kind: "resize";
  id: string;
  handle: DesignHandle;
  rect: DesignRect;
}

export type DesignChange = DesignMove | DesignResize;

export interface DesignSurfaceProps {
  /** The design area's size in design units. */
  width: number;
  height: number;
  /** Screen pixels per design unit. */
  scale: number;
  /** Every item, in paint order (parents before their children, later siblings on top). */
  items: DesignItem[];
  /** Selected ids, the primary (handles, Properties) first. */
  selection: string[];
  onSelectionChange: (ids: string[]) => void;
  /** Live geometry while dragging (`null` when the drag ends or is cancelled) — a product can re-render with it for live feedback. */
  onPreview?: (change: DesignChange | null) => void;
  /** A finished drag. */
  onCommit: (change: DesignChange) => void;
  /** Grid step in design units (0 = none) — positions snap to it when no snapline is near. */
  grid?: number;
  /** Snap to siblings' edges and centers, drawing guides (Visual Studio's snaplines). Hold Alt while dragging to suspend snapping. Default `true`. */
  snaplines?: boolean;
  /** Accepts HTML drag-and-drop of `mime` (a toolbox item) — `onDrop` gets its data, where it was dropped and the container there. */
  externalDrop?: { mime: string; onDrop: (data: string, point: DesignPoint, target: string | null) => void };
  /** Right-click — the item under the pointer is selected first (`null` over empty space). */
  onContextMenu?: (e: ReactMouseEvent, id: string | null) => void;
  /** Draw a faint grid in the background (only when `grid` is set). */
  showGrid?: boolean;
  /** The rendered content, laid out at `scale` with the design origin at its top-left. */
  children?: ReactNode;
  className?: string;
}

type Drag =
  | { type: "move"; start: DesignPoint; ids: string[]; origin: Record<string, DesignRect>; moved: boolean; clickId: string; additive: boolean }
  | { type: "resize"; start: DesignPoint; id: string; handle: DesignHandle; origin: DesignRect }
  | { type: "marquee"; start: DesignPoint; base: string[]; moved: boolean };

interface Guide {
  axis: "x" | "y";
  at: number;
  from: number;
  to: number;
}

const DRAG_THRESHOLD_PX = 3;
const SNAP_PX = 6;
const MARGIN_PX = 32;

function contains(r: DesignRect, p: DesignPoint): boolean {
  return p.x >= r.x && p.y >= r.y && p.x <= r.x + r.w && p.y <= r.y + r.h;
}

function intersects(a: DesignRect, b: DesignRect): boolean {
  return a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h;
}

function union(rects: DesignRect[]): DesignRect {
  const x = Math.min(...rects.map((r) => r.x));
  const y = Math.min(...rects.map((r) => r.y));
  const r = Math.max(...rects.map((r) => r.x + r.w));
  const b = Math.max(...rects.map((r) => r.y + r.h));
  return { x, y, w: r - x, h: b - y };
}

function normalize(a: DesignPoint, b: DesignPoint): DesignRect {
  return { x: Math.min(a.x, b.x), y: Math.min(a.y, b.y), w: Math.abs(a.x - b.x), h: Math.abs(a.y - b.y) };
}

/** Lines along one axis something can snap to: [position, extent from, extent to]. */
type SnapLine = [number, number, number];

/**
 * Finds the smallest shift (within `threshold`) that puts one of `edges`
 * onto one of `lines` — `null` when nothing is close enough.
 */
function nearest(edges: number[], lines: SnapLine[], threshold: number): { shift: number; line: SnapLine } | null {
  let best: { shift: number; line: SnapLine } | null = null;
  for (const e of edges) {
    for (const line of lines) {
      const shift = line[0] - e;
      if (Math.abs(shift) <= threshold && (!best || Math.abs(shift) < Math.abs(best.shift))) best = { shift, line };
    }
  }
  return best;
}

/**
 * A WinForms/WPF-style design surface: a scrollable, zoomed page on which
 * the product draws its content (`children`), with selection frames and
 * resize handles, rubber-band selection, dragging with grid snapping and
 * snaplines to siblings, and HTML drops from a toolbox. Product-neutral —
 * it only reports geometry (`onCommit`), the product decides what an edit
 * means for its document. Keyboard commands (nudge, delete, copy) are the
 * product's: the surface is focusable, bind them to "focus is inside".
 */
export function DesignSurface({
  width,
  height,
  scale,
  items,
  selection,
  onSelectionChange,
  onPreview,
  onCommit,
  grid = 0,
  snaplines = true,
  externalDrop,
  onContextMenu,
  showGrid = false,
  children,
  className,
}: DesignSurfaceProps) {
  const pageRef = useRef<HTMLDivElement>(null);
  const dragRef = useRef<Drag | null>(null);
  const [preview, setPreview] = useState<DesignChange | null>(null);
  const [guides, setGuides] = useState<Guide[]>([]);
  const [marquee, setMarquee] = useState<DesignRect | null>(null);
  const [target, setTarget] = useState<string | null>(null);
  const [hover, setHover] = useState<string | null>(null);

  const byId = new Map(items.map((i) => [i.id, i]));
  const primary = selection.length > 0 ? byId.get(selection[0]) : undefined;

  function pointOf(e: { clientX: number; clientY: number }): DesignPoint {
    const r = pageRef.current!.getBoundingClientRect();
    return { x: (e.clientX - r.left) / scale, y: (e.clientY - r.top) / scale };
  }

  function isWithin(id: string, ancestor: string): boolean {
    for (let cur: string | null | undefined = id; cur; cur = byId.get(cur)?.parentId) if (cur === ancestor) return true;
    return false;
  }

  function hitTest(p: DesignPoint, skip: (item: DesignItem) => boolean = () => false): DesignItem | null {
    for (let i = items.length - 1; i >= 0; i--) {
      const item = items[i];
      if (!skip(item) && contains(item.rect, p)) return item;
    }
    return null;
  }

  function containerAt(p: DesignPoint, moving: string[]): string | null {
    return hitTest(p, (item) => !item.container || moving.some((m) => isWithin(item.id, m)))?.id ?? null;
  }

  function snapTo(id: string, moving: string[]): { x: SnapLine[]; y: SnapLine[] } {
    const parentId = byId.get(id)?.parentId;
    const parent = parentId ? byId.get(parentId) : undefined;
    const lines = { x: [] as SnapLine[], y: [] as SnapLine[] };
    const add = (r: DesignRect) => {
      for (const x of [r.x, r.x + r.w / 2, r.x + r.w]) lines.x.push([x, r.y, r.y + r.h]);
      for (const y of [r.y, r.y + r.h / 2, r.y + r.h]) lines.y.push([y, r.x, r.x + r.w]);
    };
    if (parent) add(parent.contentRect ?? parent.rect);
    for (const item of items) if (item.parentId === parentId && !moving.includes(item.id) && item.placement !== "fixed") add(item.rect);
    return lines;
  }

  /** Snaps a moving box: returns the adjusted delta and the guides to draw. */
  function snapMove(box: DesignRect, dx: number, dy: number, id: string, moving: string[], free: boolean): { dx: number; dy: number; guides: Guide[] } {
    const moved = { ...box, x: box.x + dx, y: box.y + dy };
    const found: Guide[] = [];
    if (free && snaplines) {
      const lines = snapTo(id, moving);
      const threshold = SNAP_PX / scale;
      const sx = nearest([moved.x, moved.x + moved.w / 2, moved.x + moved.w], lines.x, threshold);
      const sy = nearest([moved.y, moved.y + moved.h / 2, moved.y + moved.h], lines.y, threshold);
      if (sx) dx += sx.shift;
      else if (grid > 0) dx = Math.round((box.x + dx) / grid) * grid - box.x;
      if (sy) dy += sy.shift;
      else if (grid > 0) dy = Math.round((box.y + dy) / grid) * grid - box.y;
      const final = { ...box, x: box.x + dx, y: box.y + dy };
      if (sx) found.push({ axis: "x", at: sx.line[0], from: Math.min(sx.line[1], final.y), to: Math.max(sx.line[2], final.y + final.h) });
      if (sy) found.push({ axis: "y", at: sy.line[0], from: Math.min(sy.line[1], final.x), to: Math.max(sy.line[2], final.x + final.w) });
    } else if (free && grid > 0) {
      dx = Math.round((box.x + dx) / grid) * grid - box.x;
      dy = Math.round((box.y + dy) / grid) * grid - box.y;
    }
    return { dx: Math.round(dx), dy: Math.round(dy), guides: found };
  }

  function resized(origin: DesignRect, handle: DesignHandle, dx: number, dy: number, id: string, snap: boolean): { rect: DesignRect; guides: Guide[] } {
    let { x, y } = origin;
    let r = origin.x + origin.w;
    let b = origin.y + origin.h;
    if (handle.includes("w")) x += dx;
    if (handle.includes("e")) r += dx;
    if (handle.includes("n")) y += dy;
    if (handle.includes("s")) b += dy;
    const found: Guide[] = [];
    const lines = snap && snaplines ? snapTo(id, [id]) : null;
    const threshold = SNAP_PX / scale;
    const snapEdge = (value: number, axis: "x" | "y"): number => {
      const hit = lines ? nearest([value], lines[axis], threshold) : null;
      if (hit) {
        found.push({ axis, at: hit.line[0], from: hit.line[1], to: hit.line[2] });
        return hit.line[0];
      }
      return grid > 0 && snap ? Math.round(value / grid) * grid : value;
    };
    if (handle.includes("w")) x = snapEdge(x, "x");
    if (handle.includes("e")) r = snapEdge(r, "x");
    if (handle.includes("n")) y = snapEdge(y, "y");
    if (handle.includes("s")) b = snapEdge(b, "y");
    x = Math.round(x);
    y = Math.round(y);
    r = Math.round(r);
    b = Math.round(b);
    if (r - x < 1) handle.includes("w") ? (x = r - 1) : (r = x + 1);
    if (b - y < 1) handle.includes("n") ? (y = b - 1) : (b = y + 1);
    for (const g of found) {
      if (g.axis === "x") {
        g.from = Math.min(g.from, y);
        g.to = Math.max(g.to, b);
      } else {
        g.from = Math.min(g.from, x);
        g.to = Math.max(g.to, r);
      }
    }
    return { rect: { x, y, w: r - x, h: b - y }, guides: found };
  }

  function showPreview(change: DesignChange | null) {
    setPreview(change);
    onPreview?.(change);
  }

  function onPointerDown(e: ReactPointerEvent<HTMLDivElement>) {
    if (e.button !== 0) return;
    e.currentTarget.focus();
    const p = pointOf(e);
    const handle = (e.target as HTMLElement).dataset.handle as DesignHandle | undefined;
    e.currentTarget.setPointerCapture(e.pointerId);
    if (handle && primary) {
      dragRef.current = { type: "resize", start: p, id: primary.id, handle, origin: primary.rect };
      return;
    }
    const hit = hitTest(p);
    const additive = e.ctrlKey || e.shiftKey || e.metaKey;
    if (!hit || hit.placement === "fixed") {
      const base = additive ? selection : [];
      if (!additive) onSelectionChange(hit ? [hit.id] : []);
      dragRef.current = { type: "marquee", start: p, base, moved: false };
      return;
    }
    let next = selection;
    if (additive) {
      next = selection.includes(hit.id) ? selection.filter((id) => id !== hit.id) : [hit.id, ...selection];
      onSelectionChange(next);
      if (!next.includes(hit.id)) return;
    } else if (!selection.includes(hit.id)) {
      next = [hit.id];
      onSelectionChange(next);
    }
    // Drag what's selected, minus fixed items and items whose container moves with them anyway.
    const ids = next.filter((id) => {
      const item = byId.get(id);
      return item && item.placement !== "fixed" && !next.some((other) => other !== id && isWithin(id, other));
    });
    const origin: Record<string, DesignRect> = {};
    for (const id of ids) origin[id] = byId.get(id)!.rect;
    dragRef.current = { type: "move", start: p, ids, origin, moved: false, clickId: hit.id, additive };
  }

  function onPointerMove(e: ReactPointerEvent<HTMLDivElement>) {
    const drag = dragRef.current;
    const p = pointOf(e);
    if (!drag) {
      setHover(hitTest(p)?.id ?? null);
      return;
    }
    const dxPx = (p.x - drag.start.x) * scale;
    const dyPx = (p.y - drag.start.y) * scale;
    if (drag.type !== "resize" && !drag.moved && Math.hypot(dxPx, dyPx) < DRAG_THRESHOLD_PX) return;
    const dx = p.x - drag.start.x;
    const dy = p.y - drag.start.y;
    if (drag.type === "marquee") {
      drag.moved = true;
      const box = normalize(drag.start, p);
      setMarquee(box);
      const inside = items.filter((i) => i.placement !== "fixed" && intersects(i.rect, box) && contains(box, { x: i.rect.x, y: i.rect.y }) && contains(box, { x: i.rect.x + i.rect.w, y: i.rect.y + i.rect.h }));
      // Only the outermost: a container and its children aren't picked together.
      const outer = inside.filter((i) => !inside.some((o) => o !== i && isWithin(i.id, o.id)));
      onSelectionChange([...drag.base, ...outer.map((i) => i.id).filter((id) => !drag.base.includes(id))]);
      return;
    }
    if (drag.type === "move") {
      if (drag.ids.length === 0) return;
      drag.moved = true;
      const lead = byId.get(drag.ids[0])!;
      const box = union(drag.ids.map((id) => drag.origin[id]));
      const free = lead.placement === "free";
      const snap = e.altKey ? { dx: Math.round(dx), dy: Math.round(dy), guides: [] } : snapMove(box, dx, dy, lead.id, drag.ids, free);
      const rects: Record<string, DesignRect> = {};
      for (const id of drag.ids) rects[id] = { ...drag.origin[id], x: drag.origin[id].x + snap.dx, y: drag.origin[id].y + snap.dy };
      const over = containerAt(p, drag.ids);
      setGuides(snap.guides);
      setTarget(over !== lead.parentId || !free ? over : null);
      showPreview({ kind: "move", ids: drag.ids, rects, target: over, point: p });
      return;
    }
    const { rect, guides } = resized(drag.origin, drag.handle, dx, dy, drag.id, !e.altKey);
    setGuides(guides);
    showPreview({ kind: "resize", id: drag.id, handle: drag.handle, rect });
  }

  function endDrag(commit: boolean) {
    const drag = dragRef.current;
    dragRef.current = null;
    const change = preview;
    setGuides([]);
    setMarquee(null);
    setTarget(null);
    if (preview) showPreview(null);
    if (!drag) return;
    if (drag.type === "move" && !drag.moved && !drag.additive && selection.length > 1) {
      // A plain click on one of several selected items makes it the primary.
      onSelectionChange([drag.clickId, ...selection.filter((id) => id !== drag.clickId)]);
    }
    if (commit && change && drag.type !== "marquee") onCommit(change);
  }

  function onKeyDown(e: ReactKeyboardEvent) {
    if (e.key === "Escape" && dragRef.current) {
      e.stopPropagation();
      endDrag(false);
    }
  }

  function onDragOver(e: DragEvent) {
    if (!externalDrop || !Array.from(e.dataTransfer.types).includes(externalDrop.mime)) return;
    // Claim it, so drop targets around the surface (a dock) don't react too.
    e.preventDefault();
    e.stopPropagation();
    e.dataTransfer.dropEffect = "copy";
    setTarget(containerAt(pointOf(e), []));
  }

  function onDrop(e: DragEvent) {
    if (!externalDrop) return;
    const data = e.dataTransfer.getData(externalDrop.mime);
    setTarget(null);
    if (!data) return;
    e.preventDefault();
    e.stopPropagation();
    const p = pointOf(e);
    externalDrop.onDrop(data, { x: Math.round(p.x), y: Math.round(p.y) }, containerAt(p, []));
    pageRef.current?.parentElement?.focus();
  }

  function handleContextMenu(e: ReactMouseEvent) {
    if (!onContextMenu) return;
    e.preventDefault();
    const hit = hitTest(pointOf(e));
    if (hit && !selection.includes(hit.id)) onSelectionChange([hit.id]);
    onContextMenu(e, hit?.id ?? null);
  }

  // Geometry as currently shown — the in-flight drag applied.
  const shown = (id: string): DesignRect | undefined => {
    if (preview?.kind === "move" && preview.rects[id]) return preview.rects[id];
    if (preview?.kind === "resize" && preview.id === id) return preview.rect;
    return byId.get(id)?.rect;
  };
  const px = (r: DesignRect): CSSProperties => ({ left: r.x * scale, top: r.y * scale, width: r.w * scale, height: r.h * scale });

  const primaryRect = primary ? shown(primary.id) : undefined;
  const handles = primary ? (primary.handles ?? (primary.placement === "free" ? ALL_HANDLES : SIZE_HANDLES)) : [];
  const hoverItem = hover && !dragRef.current && !selection.includes(hover) ? byId.get(hover) : undefined;
  const resizeBadge = preview?.kind === "resize" ? preview.rect : null;
  const gridStyle: CSSProperties | undefined =
    showGrid && grid > 0 && grid * scale >= 4 ? ({ "--sp-design-grid": `${grid * scale}px` } as CSSProperties) : undefined;

  return (
    <div
      className={["sp-design-surface", className].filter(Boolean).join(" ")}
      tabIndex={0}
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={() => endDrag(true)}
      onPointerCancel={() => endDrag(false)}
      onPointerLeave={() => setHover(null)}
      onKeyDown={onKeyDown}
      onDragEnter={onDragOver}
      onDragOver={onDragOver}
      onDragLeave={() => setTarget(null)}
      onDrop={onDrop}
      onContextMenu={handleContextMenu}
    >
      <div className="sp-design-surface-canvas" style={{ width: width * scale + MARGIN_PX * 2, height: height * scale + MARGIN_PX * 2 }}>
        <div ref={pageRef} className="sp-design-surface-page" style={{ left: MARGIN_PX, top: MARGIN_PX, width: width * scale, height: height * scale }}>
          <div className="sp-design-surface-content">{children}</div>
          {gridStyle && <div className="sp-design-surface-grid" style={gridStyle} />}
          <div className="sp-design-surface-overlay">
            {hoverItem && <div className="sp-design-surface-hover" style={px(hoverItem.rect)} title={hoverItem.label} />}
            {target && byId.get(target) && <div className="sp-design-surface-target" style={px(byId.get(target)!.rect)} />}
            {preview?.kind === "move" &&
              Object.entries(preview.rects).map(([id, r]) => <div key={`ghost-${id}`} className="sp-design-surface-ghost" style={px(r)} />)}
            {selection.map((id, i) => {
              const r = shown(id);
              return r ? <div key={id} className={`sp-design-surface-frame${i === 0 ? " primary" : ""}`} style={px(r)} /> : null;
            })}
            {primaryRect &&
              !(preview?.kind === "move") &&
              handles.map((h) => (
                <div
                  key={h}
                  data-handle={h}
                  className={`sp-design-surface-handle sp-design-surface-handle-${h}`}
                  style={{
                    left: (primaryRect.x + (h.includes("w") ? 0 : h.includes("e") ? primaryRect.w : primaryRect.w / 2)) * scale,
                    top: (primaryRect.y + (h.includes("n") ? 0 : h.includes("s") ? primaryRect.h : primaryRect.h / 2)) * scale,
                  }}
                />
              ))}
            {guides.map((g, i) => (
              <div
                key={i}
                className="sp-design-surface-guide"
                style={
                  g.axis === "x"
                    ? { left: g.at * scale, top: g.from * scale, width: 1, height: (g.to - g.from) * scale }
                    : { left: g.from * scale, top: g.at * scale, width: (g.to - g.from) * scale, height: 1 }
                }
              />
            ))}
            {marquee && <div className="sp-design-surface-marquee" style={px(marquee)} />}
            {resizeBadge && (
              <div className="sp-design-surface-badge" style={{ left: (resizeBadge.x + resizeBadge.w) * scale + 6, top: (resizeBadge.y + resizeBadge.h) * scale + 6 }}>
                {resizeBadge.w} × {resizeBadge.h}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
