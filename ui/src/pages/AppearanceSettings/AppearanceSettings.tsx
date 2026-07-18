import { Label } from "../../widgets/Label";
import { AccentColorField } from "../../widgets/AccentColorField";
import { EditorColorField } from "../../widgets/EditorColorField";
import { Button } from "../../widgets/Button";
import { useTheme, editorColorKeys, type EditorThemeId } from "../../infra/theme";
import "./AppearanceSettings.css";

const EDITOR_THEME_LABELS: Record<EditorThemeId, string> = {
  "vs-dark": "VS Dark",
  classic: "Classic (CodeMirror default)",
};

/** The Settings window's "Appearance" section — the UI's accent color, and the code editor's own color scheme (a starting profile plus a fully independent color per syntax category, VS's "Fonts and Colors" model). */
export function AppearanceSettings() {
  const { editorTheme, setEditorTheme, editorThemePresets, resetEditorColors } = useTheme();

  return (
    <div className="sp-appearance-settings">
      <Label>Accent Color</Label>
      <AccentColorField />

      <div className="sp-appearance-settings-divider" />

      <Label>Editor Color Profile</Label>
      <div className="sp-appearance-settings-row">
        <select
          className="sp-appearance-settings-select"
          value={editorTheme}
          onChange={(e) => setEditorTheme(e.target.value as EditorThemeId)}
        >
          {editorThemePresets.map((id) => (
            <option key={id} value={id}>
              {EDITOR_THEME_LABELS[id]}
            </option>
          ))}
        </select>
        <Button variant="ghost" onClick={resetEditorColors}>
          Reset to Profile Defaults
        </Button>
      </div>

      <Label>Editor Colors</Label>
      <div className="sp-appearance-settings-colors">
        {editorColorKeys.map((key) => (
          <EditorColorField key={key} colorKey={key} />
        ))}
      </div>
    </div>
  );
}
