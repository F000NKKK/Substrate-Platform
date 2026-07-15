import { ColorWheel } from "./ColorWheel";
import { Button } from "./Button";
import { IconButton } from "./IconButton";
import { Label } from "./Label";
import { Swatch } from "./Swatch";
import { useDraggable } from "./useDraggable";
import { sameColor, type HslColor } from "./Color";
import { IconClose } from "../icons";
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
  const { pos, handlers } = useDraggable({ x: 200, y: 140 });

  return (
    <div className="sp-color-picker" style={{ left: pos.x, top: pos.y }}>
      <div className="sp-color-picker-header" {...handlers}>
        <Label>{title}</Label>
        <IconButton size={24} aria-label="Close" onClick={onClose}>
          <IconClose size={20} />
        </IconButton>
      </div>

      <div className="sp-color-picker-body">
        <ColorWheel
          hue={value.h}
          saturation={value.s / 100}
          lightness={value.l / 100}
          onChange={(h, s) => onChange({ ...value, h, s: Math.round(s * 100) })}
        />

        <div className="sp-color-picker-field">
          <Label>Lightness</Label>
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
            <Label>Presets</Label>
            <div className="sp-color-picker-swatches">
              {presets.map((preset) => (
                <Swatch
                  key={`${preset.h}-${preset.s}-${preset.l}`}
                  color={preset}
                  active={sameColor(preset, value)}
                  onClick={() => onChange(preset)}
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
