import { useRef, useState } from "react";
import { ColorWheel } from "./ColorWheel";
import { Button } from "./Button";
import { hslToCss, sameColor, type HslColor } from "./color";
import "./ColorPickerWindow.css";

export interface ColorPickerWindowProps {
  title?: string;
  value: HslColor;
  presets?: readonly HslColor[];
  defaultValue?: HslColor;
  onChange: (color: HslColor) => void;
  onClose: () => void;
}

/**
 * The one universal color picker in the platform — a real floating window
 * (draggable, closable), not a cramped popover. Anything that needs to let
 * the user pick a color should open this rather than rolling its own.
 */
export function ColorPickerWindow({ title = "Select Color", value, presets, defaultValue, onChange, onClose }: ColorPickerWindowProps) {
  const [pos, setPos] = useState({ x: 200, y: 140 });
  const dragOrigin = useRef<{ x: number; y: number; startX: number; startY: number } | null>(null);

  function handleHeaderPointerDown(e: React.PointerEvent) {
    e.currentTarget.setPointerCapture(e.pointerId);
    dragOrigin.current = { x: e.clientX, y: e.clientY, startX: pos.x, startY: pos.y };
  }

  function handleHeaderPointerMove(e: React.PointerEvent) {
    const origin = dragOrigin.current;
    if (!origin || e.buttons !== 1) return;
    setPos({ x: origin.startX + (e.clientX - origin.x), y: origin.startY + (e.clientY - origin.y) });
  }

  function handleHeaderPointerUp() {
    dragOrigin.current = null;
  }

  return (
    <div className="sp-color-picker" style={{ left: pos.x, top: pos.y }}>
      <div
        className="sp-color-picker-header"
        onPointerDown={handleHeaderPointerDown}
        onPointerMove={handleHeaderPointerMove}
        onPointerUp={handleHeaderPointerUp}
      >
        <span className="sp-color-picker-title">{title}</span>
        <button className="sp-color-picker-close" aria-label="Close" onClick={onClose}>
          ×
        </button>
      </div>

      <div className="sp-color-picker-body">
        <ColorWheel
          hue={value.h}
          saturation={value.s / 100}
          lightness={value.l / 100}
          onChange={(h, s) => onChange({ ...value, h, s: Math.round(s * 100) })}
        />

        <div className="sp-color-picker-field">
          <span className="sp-color-picker-label">Lightness</span>
          <input
            type="range"
            className="sp-color-picker-slider"
            min={0}
            max={100}
            value={value.l}
            onChange={(e) => onChange({ ...value, l: Number(e.target.value) })}
          />
        </div>

        {presets && presets.length > 0 && (
          <div className="sp-color-picker-field">
            <span className="sp-palette-label">Presets</span>
            <div className="sp-palette-swatches">
              {presets.map((preset) => (
                <button
                  key={hslToCss(preset)}
                  className="sp-palette-swatch"
                  data-active={sameColor(preset, value)}
                  style={{ background: hslToCss(preset) }}
                  onClick={() => onChange(preset)}
                  aria-label={hslToCss(preset)}
                />
              ))}
            </div>
          </div>
        )}

        {defaultValue && (
          <Button variant="ghost" onClick={() => onChange(defaultValue)}>
            Reset to default
          </Button>
        )}
      </div>
    </div>
  );
}
