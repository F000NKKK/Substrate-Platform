import { useState } from "react";
import { Label } from "../../widgets/Label";
import { TextField } from "../../widgets/TextField";
import { Button } from "../../widgets/Button";
import { useTheme } from "../../infra/theme";
import { useT } from "../../infra/i18n";
import "./EditorSettings.css";

const LABELS = {
  fontFamily: "Font Family",
  fontSize: "Font Size",
  tabSize: "Tab Size",
  wordWrap: "Word Wrap",
} as const;

/** Every setting label on this page (English; the Settings window's search also matches their translations) — pass as the section's `keywords` so the Settings window's search finds the page by any of them. */
export const editorSettingsKeywords: readonly string[] = Object.values(LABELS);

/**
 * A number field that only commits values inside `[min, max]` — typing
 * through an intermediate out-of-range value (clearing "14" to type "20")
 * doesn't clamp mid-edit; leaving the field snaps it back to the committed
 * value.
 */
function NumberSetting({ value, min, max, onCommit }: { value: number; min: number; max: number; onCommit: (n: number) => void }) {
  const [draft, setDraft] = useState<string | null>(null);
  // The value changed from elsewhere (Undo/Redo): show it, not the typing.
  if (draft !== null && draft !== "" && Number(draft) !== value && Number.isInteger(Number(draft)) && Number(draft) >= min && Number(draft) <= max) setDraft(null);
  return (
    <TextField
      className="sp-editor-settings-number"
      type="number"
      min={min}
      max={max}
      value={draft ?? String(value)}
      onChange={(e) => {
        setDraft(e.target.value);
        const n = Number(e.target.value);
        if (e.target.value !== "" && Number.isInteger(n) && n >= min && n <= max) onCommit(n);
      }}
      onBlur={() => setDraft(null)}
    />
  );
}

/** The Settings window's "Editor" section — font, tab size and word wrap for every code editor, applied live (see `ThemeContextValue.editorPreferences`). */
export function EditorSettings() {
  const t = useT();
  const { editorPreferences: prefs, setEditorPreferences, resetEditorPreferences } = useTheme();

  return (
    <div className="sp-editor-settings">
      <Label>{t(LABELS.fontFamily)}</Label>
      <TextField
        value={prefs.fontFamily}
        placeholder={t('Default monospace — or e.g. "JetBrains Mono", monospace')}
        onChange={(e) => setEditorPreferences({ fontFamily: e.target.value })}
      />

      <Label>{t(LABELS.fontSize)}</Label>
      <NumberSetting value={prefs.fontSize} min={8} max={40} onCommit={(fontSize) => setEditorPreferences({ fontSize })} />

      <Label>{t(LABELS.tabSize)}</Label>
      <NumberSetting value={prefs.tabSize} min={1} max={16} onCommit={(tabSize) => setEditorPreferences({ tabSize })} />

      <label className="sp-editor-settings-check">
        <input type="checkbox" checked={prefs.wordWrap} onChange={(e) => setEditorPreferences({ wordWrap: e.target.checked })} />
        {t(LABELS.wordWrap)}
      </label>

      <div className="sp-editor-settings-divider" />
      <div>
        <Button variant="ghost" onClick={resetEditorPreferences}>
          {t("Reset to Defaults")}
        </Button>
      </div>
    </div>
  );
}
