import { useState } from "react";
import { useTheme } from "./useTheme";
import "./PalettePicker.css";

/** Reusable accent-color control: preset swatches + a free hue slider, backed by ThemeProvider. */
export function PalettePicker() {
  const { accent, setAccent, presets } = useTheme();
  const [open, setOpen] = useState(false);

  return (
    <div className="sp-palette-root">
      <button
        className="sp-palette-trigger"
        aria-label="Accent color"
        onClick={() => setOpen((v) => !v)}
      />
      {open && (
        <div className="sp-palette-popover" role="dialog">
          <div>
            <div className="sp-palette-label">Presets</div>
            <div className="sp-palette-swatches">
              {presets.map((preset) => (
                <button
                  key={`${preset.h}-${preset.s}-${preset.l}`}
                  className="sp-palette-swatch"
                  data-active={preset.h === accent.h && preset.s === accent.s && preset.l === accent.l}
                  style={{ background: `hsl(${preset.h} ${preset.s}% ${preset.l}%)` }}
                  onClick={() => setAccent(preset)}
                  aria-label={`Hue ${preset.h}`}
                />
              ))}
            </div>
          </div>
          <div>
            <div className="sp-palette-label">Hue</div>
            <input
              className="sp-palette-slider"
              type="range"
              min={0}
              max={360}
              value={accent.h}
              onChange={(e) => setAccent({ ...accent, h: Number(e.target.value) })}
            />
          </div>
        </div>
      )}
    </div>
  );
}
