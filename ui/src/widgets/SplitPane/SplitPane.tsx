import { useRef, useState, type CSSProperties, type KeyboardEvent as ReactKeyboardEvent, type PointerEvent as ReactPointerEvent, type ReactNode } from "react";
import { useT } from "../../infra/i18n";
import "./SplitPane.css";

export interface SplitPaneProps {
  /** `"column"` stacks the panes top/bottom (a horizontal splitter), `"row"` puts them side by side. */
  direction: "row" | "column";
  first: ReactNode;
  second: ReactNode;
  /** The first pane's share of the space, 0–1. */
  ratio: number;
  /** Called once a drag (or a keyboard resize) ends, with the new ratio — the host persists it. */
  onRatioChange?: (ratio: number) => void;
  /** Shows only one pane, the other kept mounted but hidden (its state survives). */
  collapsed?: "first" | "second" | null;
  /** Smallest size of either pane while dragging, in pixels. Default 60. */
  minSize?: number;
  /**
   * Content of the splitter itself — a strip of tabs/buttons sitting on the
   * boundary, like Visual Studio's Design/XAML bar. Its empty area still
   * drags. Without it the splitter is a thin line.
   */
  handle?: ReactNode;
  className?: string;
}

/**
 * Two panes with a draggable splitter between them — a designer over its
 * source, a list over a detail view. Every pane stays mounted when
 * collapsed, so a text editor keeps its undo history and scroll position.
 */
export function SplitPane({ direction, first, second, ratio, onRatioChange, collapsed = null, minSize = 60, handle, className }: SplitPaneProps) {
  const t = useT();
  const rootRef = useRef<HTMLDivElement>(null);
  // The ratio while dragging — the host only hears about it at the end.
  const [live, setLive] = useState<number | null>(null);
  const shown = clamp(live ?? ratio);
  const row = direction === "row";

  const ratioAt = (clientX: number, clientY: number): number => {
    const rect = rootRef.current?.getBoundingClientRect();
    if (!rect) return shown;
    const size = row ? rect.width : rect.height;
    if (size <= 0) return shown;
    const min = Math.min(0.45, minSize / size);
    const at = row ? clientX - rect.left : clientY - rect.top;
    return Math.min(1 - min, Math.max(min, at / size));
  };

  const onPointerDown = (e: ReactPointerEvent<HTMLDivElement>) => {
    if (e.button !== 0 || (e.target as HTMLElement).closest("button, input, select, a, [role='tab']")) return;
    e.preventDefault();
    const el = e.currentTarget;
    el.setPointerCapture(e.pointerId);
    let last = shown;
    const move = (ev: PointerEvent) => {
      last = ratioAt(ev.clientX, ev.clientY);
      setLive(last);
    };
    const up = () => {
      el.removeEventListener("pointermove", move);
      el.removeEventListener("pointerup", up);
      el.removeEventListener("pointercancel", up);
      setLive(null);
      if (last !== ratio) onRatioChange?.(last);
    };
    el.addEventListener("pointermove", move);
    el.addEventListener("pointerup", up);
    el.addEventListener("pointercancel", up);
  };

  const onKeyDown = (e: ReactKeyboardEvent<HTMLDivElement>) => {
    const back = row ? "ArrowLeft" : "ArrowUp";
    const forward = row ? "ArrowRight" : "ArrowDown";
    if (e.key !== back && e.key !== forward) return;
    e.preventDefault();
    onRatioChange?.(clamp(shown + (e.key === back ? -0.05 : 0.05)));
  };

  // Grow factors rather than sizes: the handle takes its own space first.
  const firstStyle: CSSProperties = { flex: `${shown} 1 0` };
  const secondStyle: CSSProperties = { flex: `${1 - shown} 1 0` };
  const classes = ["sp-split-pane", `sp-split-pane--${direction}`, collapsed ? `sp-split-pane--only-${collapsed === "first" ? "second" : "first"}` : "", live !== null ? "sp-split-pane--dragging" : "", className]
    .filter(Boolean)
    .join(" ");
  return (
    <div ref={rootRef} className={classes}>
      <div className="sp-split-pane-first" style={firstStyle}>
        {first}
      </div>
      <div
        className={`sp-split-pane-handle${handle ? " sp-split-pane-handle--bar" : ""}`}
        role="separator"
        aria-orientation={row ? "vertical" : "horizontal"}
        aria-valuenow={Math.round(shown * 100)}
        aria-label={t("Resize panes")}
        tabIndex={collapsed ? -1 : 0}
        onPointerDown={collapsed ? undefined : onPointerDown}
        onDoubleClick={(e) => !collapsed && !(e.target as HTMLElement).closest("button") && onRatioChange?.(0.5)}
        onKeyDown={collapsed ? undefined : onKeyDown}
      >
        {handle}
      </div>
      <div className="sp-split-pane-second" style={secondStyle}>
        {second}
      </div>
    </div>
  );
}

const clamp = (r: number) => (Number.isFinite(r) ? Math.min(0.95, Math.max(0.05, r)) : 0.5);
