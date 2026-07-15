import { useCallback, useRef } from "react";
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
 */
export function ColorWheel({ hue, saturation, lightness, onChange, size = 160 }: ColorWheelProps) {
  const ref = useRef<HTMLDivElement>(null);
  const radius = size / 2;

  const updateFromPoint = useCallback(
    (clientX: number, clientY: number) => {
      const el = ref.current;
      if (!el) return;
      const rect = el.getBoundingClientRect();
      const cx = rect.left + radius;
      const cy = rect.top + radius;
      const dx = clientX - cx;
      const dy = clientY - cy;
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
      ref={ref}
      className="sp-color-wheel"
      style={{ width: size, height: size }}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
    >
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
