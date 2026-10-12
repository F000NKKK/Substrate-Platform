import { useState } from "react";
import { Swatch } from "../Swatch";
import { Button } from "../Button";
import { ColorPickerWindow } from "../../windows/ColorPickerWindow";
import { useTheme, colorSchemeFor, type EditorColorKey } from "../../infra/theme";
import { useT } from "../../infra/i18n";
import "./EditorColorField.css";

/** Display name per color key (English; shown through `t`) — also what the Appearance page's settings search matches against. */
export const editorColorLabels: Record<EditorColorKey, string> = {
  background: "Background",
  foreground: "Foreground",
  selection: "Selection",
  cursor: "Cursor",
  keyword: "Keyword",
  string: "String",
  comment: "Comment",
  number: "Number",
  type: "Type",
  function: "Function",
  variable: "Variable/Identifier",
  operator: "Operator",
};

/**
 * One row of the editor's "Fonts and Colors"-style settings — a single
 * `EditorColorKey`'s current color plus a picker, same trigger pattern
 * `AccentColorField` uses for the UI's own accent. `background` applies
 * across every open file/language uniformly; this only ever recolors the
 * one category it's bound to, never any other.
 */
export function EditorColorField({ colorKey }: { colorKey: EditorColorKey }) {
  const t = useT();
  const { editorColors, setEditorColor, editorTheme } = useTheme();
  const label = t(editorColorLabels[colorKey]);
  const [pickerOpen, setPickerOpen] = useState(false);
  const color = editorColors[colorKey];

  return (
    <div className="sp-editor-color-field">
      <span className="sp-editor-color-field-label">{label}</span>
      <Swatch color={color} onClick={() => setPickerOpen(true)} aria-label={t("{name} color", { name: label })} />
      <Button variant="ghost" onClick={() => setPickerOpen(true)}>
        {t("Choose Color…")}
      </Button>

      {pickerOpen && (
        <ColorPickerWindow
          title={label}
          value={color}
          defaultValue={colorSchemeFor(editorTheme)[colorKey]}
          onChange={(next) => setEditorColor(colorKey, next)}
          onClose={() => setPickerOpen(false)}
        />
      )}
    </div>
  );
}
