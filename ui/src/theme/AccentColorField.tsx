import { useTheme } from "./useTheme";
import "./PalettePicker.css";

/** The accent-color controls themselves (presets + hue slider), with no popover/trigger chrome — reusable inside a Settings page or a standalone popover alike. */
export function AccentColorField() {
  const { accent, setAccent, presets } = useTheme();

  return (
    <div className="sp-accent-field">
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
  );
}
