import { useState } from "react";
import { Swatch } from "../Swatch";
import { Button } from "../Button";
import { ColorPickerWindow } from "../../windows/ColorPickerWindow";
import { useTheme, defaultAccent } from "../../infra/theme";
import { useT } from "../../infra/i18n";
import "./AccentColorField.css";

/**
 * The accent-color row for a Settings page: shows the current color and a
 * button that opens the universal ColorPickerWindow — this component owns
 * no color-picking logic itself, only the trigger.
 */
export function AccentColorField() {
  const t = useT();
  const { accent, setAccent, presets } = useTheme();
  const [pickerOpen, setPickerOpen] = useState(false);

  return (
    <div className="sp-accent-field">
      <Swatch color={accent} onClick={() => setPickerOpen(true)} aria-label={t("Current accent color")} />
      <Button variant="ghost" onClick={() => setPickerOpen(true)}>
        {t("Choose Color…")}
      </Button>

      {pickerOpen && (
        <ColorPickerWindow
          title={t("Accent Color")}
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
