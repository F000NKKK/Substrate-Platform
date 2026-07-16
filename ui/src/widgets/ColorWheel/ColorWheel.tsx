import { useCallback, useEffect, useRef } from "react";
import { hslToRgb } from "../../infra/color";
import "./ColorWheel.css";

export interface ColorWheelProps {
  /** Hue in degrees [0, 360). */
  hue: number;
  /** Saturation as a fraction [0, 1]. */
  saturation: number;
  /** Lightness as a fraction [0, 1] — only affects the cursor's own color swatch, not the wheel's rendering. */
  lightness: number;
  onChange: (hue: number, saturation: number) => void;
  size?: number;
}

/**
 * A universal HSV-style color wheel — angle picks hue, distance from
 * center picks saturation. Every color selector in the platform should go
 * through this rather than a single hue slider, so any product gets a
 * real "click anywhere for any color" picker.
 *
 * Rendered on a canvas (not layered CSS gradients + blend-mode) because
 * blend-mode compositing of stacked alpha gradients renders inconsistently
 * across engines — a canvas fill gives a pixel-exact, guaranteed-circular
 * result everywhere.
 */
export function ColorWheel({ hue, saturation, lightness, onChange, size = 160 }: ColorWheelProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const radius = size / 2;

  useEffect(() => {
    const canvas = canvasRef.current;
    const ctx = canvas?.getContext("2d");
    if (!canvas || !ctx) return;
    canvas.width = size;
    canvas.height = size;

    ctx.save();
    ctx.beginPath();
    ctx.arc(radius, radius, radius, 0, Math.PI * 2);
    ctx.clip();

    const image = ctx.createImageData(size, size);
    for (let y = 0; y < size; y++) {
      for (let x = 0; x < size; x++) {
        const dx = x - radius;
        const dy = y - radius;
        const dist = Math.hypot(dx, dy);
        const idx = (y * size + x) * 4;
        if (dist > radius) continue; // stays transparent (clip handles it too, but skip the work)
        const angle = (Math.atan2(dy, dx) * 180) / Math.PI;
        const [r, g, b] = hslToRgb((angle + 360) % 360, Math.min(1, dist / radius), 0.5);
        image.data[idx] = r;
        image.data[idx + 1] = g;
        image.data[idx + 2] = b;
        image.data[idx + 3] = 255;
      }
    }
    ctx.putImageData(image, 0, 0);
    ctx.restore();
  }, [size, radius]);

  const updateFromPoint = useCallback(
    (clientX: number, clientY: number) => {
      const el = containerRef.current;
      if (!el) return;
      const rect = el.getBoundingClientRect();
      const dx = clientX - (rect.left + radius);
      const dy = clientY - (rect.top + radius);
      const nextHue = (Math.atan2(dy, dx) * 180) / Math.PI;
      const nextSat = Math.min(1, Math.hypot(dx, dy) / radius);
      onChange((nextHue + 360) % 360, nextSat);
    },
    [onChange, radius]
  );

  function handlePointerDown(e: React.PointerEvent) {
    e.currentTarget.setPointerCapture(e.pointerId);
    updateFromPoint(e.clientX, e.clientY);
  }

  function handlePointerMove(e: React.PointerEvent) {
    if (e.buttons !== 1) return;
    updateFromPoint(e.clientX, e.clientY);
  }

  const angle = (hue * Math.PI) / 180;
  const dist = saturation * radius;
  const cursorX = radius + Math.cos(angle) * dist;
  const cursorY = radius + Math.sin(angle) * dist;

  return (
    <div
      ref={containerRef}
      className="sp-color-wheel"
      style={{ width: size, height: size }}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
    >
      <canvas ref={canvasRef} className="sp-color-wheel-canvas" />
      <div
        className="sp-color-wheel-cursor"
        style={{
          left: cursorX,
          top: cursorY,
          background: `hsl(${hue} ${saturation * 100}% ${lightness * 100}%)`,
        }}
      />
    </div>
  );
}
