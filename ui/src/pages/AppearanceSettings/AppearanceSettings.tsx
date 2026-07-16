import { Label } from "../../widgets/Label";
import { AccentColorField } from "../../widgets/AccentColorField";
import "./AppearanceSettings.css";

/** The Settings window's "Appearance" section content — just the accent color row for now. */
export function AppearanceSettings() {
  return (
    <div className="sp-appearance-settings">
      <Label>Accent Color</Label>
      <AccentColorField />
    </div>
  );
}
