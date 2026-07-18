import { useState } from "react";
import { Swatch } from "../Swatch";
import { Button } from "../Button";
import { ColorPickerWindow } from "../../windows/ColorPickerWindow";
import { useTheme, defaultAccent } from "../../infra/theme";
import "./AccentColorField.css";

/**
 * The accent-color row for a Settings page: shows the current color and a
 * button that opens the universal ColorPickerWindow — this component owns
 * no color-picking logic itself, only the trigger.
 */
export function AccentColorField() {
  const { accent, setAccent, presets } = useTheme();
  const [pickerOpen, setPickerOpen] = useState(false);

  return (
    <div className="sp-accent-field">
      <Swatch color={accent} onClick={() => setPickerOpen(true)} aria-label="Current accent color" />
      <Button variant="ghost" onClick={() => setPickerOpen(true)}>
        Choose Color…
      </Button>

      {pickerOpen && (
        <ColorPickerWindow
          title="Accent Color"
          value={accent}
          presets={presets}
          defaultValue={defaultAccent}
          onChange={setAccent}
          onClose={() => setPickerOpen(false)}
        />
      )}
    </div>
  );
}
