import { useEffect, useLayoutEffect, useRef, useState, type PointerEvent as ReactPointerEvent, type ReactNode } from "react";
import { useT } from "../../infra/i18n";
import { Toolbar, ToolbarButton, ToolbarGroup } from "../Toolbar";
import { fitZoom, parseZoomPercent, stepZoom, zoomPercent } from "./zoom";
import "./ZoomView.css";

/** The mouse button that pans (the right one). */
const PAN_BUTTON = 2;

/** A point of the content kept under the pointer through a zoom. */
interface Anchor {
  x: number;
  y: number;
  clientX: number;
  clientY: number;
}

/**
 * Content of `width`×`height` pixels shown zoomed: fitted to the view until
 * the user zooms — Ctrl+wheel about the pointer, or a percent typed on the
 * bar — and panned by dragging with the right mouse button. Pixels stay
 * sharp from 100% up. `children` fills the zoomed box.
 */
export function ZoomView({ width, height, children, className }: { width: number; height: number; children: ReactNode; className?: string }) {
  const portRef = useRef<HTMLDivElement>(null);
  const pageRef = useRef<HTMLDivElement>(null);
  const [box, setBox] = useState<[number, number]>([0, 0]);
  const [chosen, setChosen] = useState<number | null>(null);
  const zoom = chosen ?? fitZoom(width, height, box[0], box[1]);
  const zoomRef = useRef(zoom);
  zoomRef.current = zoom;
  const anchor = useRef<Anchor | null>(null);

  useLayoutEffect(() => {
    const port = portRef.current!;
    const measure = () => setBox([port.clientWidth, port.clientHeight]);
    const ro = new ResizeObserver(measure);
    ro.observe(port);
    measure();
    return () => ro.disconnect();
  }, []);

  // After a zoom, scroll so the anchored point is back under the pointer.
  useLayoutEffect(() => {
    const a = anchor.current;
    if (!a) return;
    anchor.current = null;
    const r = pageRef.current!.getBoundingClientRect();
    portRef.current!.scrollLeft += r.left + a.x * zoom - a.clientX;
    portRef.current!.scrollTop += r.top + a.y * zoom - a.clientY;
  }, [zoom]);

  // Ctrl+wheel (a trackpad's pinch): a native listener, as React's wheel
  // events are passive and the webview would zoom the whole window.
  useEffect(() => {
    const port = portRef.current!;
    const onWheel = (e: WheelEvent) => {
      if (!(e.ctrlKey || e.metaKey) || e.deltaY === 0) return;
      e.preventDefault();
      const from = zoomRef.current;
      const r = pageRef.current!.getBoundingClientRect();
      anchor.current = { x: (e.clientX - r.left) / from, y: (e.clientY - r.top) / from, clientX: e.clientX, clientY: e.clientY };
      setChosen(stepZoom(from, -e.deltaY));
    };
    port.addEventListener("wheel", onWheel, { passive: false });
    return () => port.removeEventListener("wheel", onWheel);
  }, []);

  const startPan = (e: ReactPointerEvent<HTMLDivElement>) => {
    if (e.button !== PAN_BUTTON) return;
    const port = e.currentTarget;
    const start = { x: e.clientX, y: e.clientY, left: port.scrollLeft, top: port.scrollTop };
    port.setPointerCapture(e.pointerId);
    port.dataset.panning = "true";
    const move = (ev: PointerEvent) => {
      port.scrollLeft = start.left - (ev.clientX - start.x);
      port.scrollTop = start.top - (ev.clientY - start.y);
    };
    const end = () => {
      delete port.dataset.panning;
      port.removeEventListener("pointermove", move);
      port.removeEventListener("pointerup", end);
      port.removeEventListener("pointercancel", end);
    };
    port.addEventListener("pointermove", move);
    port.addEventListener("pointerup", end);
    port.addEventListener("pointercancel", end);
  };

  return (
    <div className={["sp-zoom-view", className].filter(Boolean).join(" ")}>
      <div className="sp-zoom-view-port" ref={portRef} onPointerDown={startPan} onContextMenu={(e) => e.preventDefault()}>
        <div className="sp-zoom-view-page" ref={pageRef} data-pixelated={zoom >= 1 || undefined} style={{ width: width * zoom, height: height * zoom }}>
          {children}
        </div>
      </div>
      <ZoomBar zoom={zoom} fitted={chosen === null} onZoom={setChosen} />
    </div>
  );
}

/** The bar: the zoom as a percent to type over, fit, and the original size. */
function ZoomBar({ zoom, fitted, onZoom }: { zoom: number; fitted: boolean; onZoom: (zoom: number | null) => void }) {
  const t = useT();
  const [text, setText] = useState<string | null>(null);
  const commit = () => {
    const z = text === null ? null : parseZoomPercent(text);
    if (z !== null) onZoom(z);
    setText(null);
  };
  return (
    <Toolbar className="sp-zoom-view-bar">
      <ToolbarGroup>
        <input
          className="sp-zoom-view-percent"
          aria-label={t("Zoom")}
          title={t("Zoom")}
          spellCheck={false}
          value={text ?? zoomPercent(zoom)}
          onFocus={(e) => e.currentTarget.select()}
          onChange={(e) => setText(e.target.value)}
          onBlur={commit}
          onKeyDown={(e) => {
            if (e.key === "Enter") e.currentTarget.blur();
            else if (e.key === "Escape") (setText(null), e.currentTarget.blur());
          }}
        />
        <ToolbarButton label="Fit to window" showLabel active={fitted} onClick={() => onZoom(null)} />
        <ToolbarButton label="Actual size" showLabel onClick={() => onZoom(1)} />
      </ToolbarGroup>
    </Toolbar>
  );
}
