import { useState } from "react";
import { Swatch } from "../Swatch";
import { Button } from "../Button";
import { ColorPickerWindow } from "../../windows/ColorPickerWindow";
import { useTheme, colorSchemeFor, type EditorColorKey } from "../../infra/theme";
import "./EditorColorField.css";

const LABELS: Record<EditorColorKey, string> = {
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
  const { editorColors, setEditorColor, editorTheme } = useTheme();
  const [pickerOpen, setPickerOpen] = useState(false);
  const color = editorColors[colorKey];

  return (
    <div className="sp-editor-color-field">
      <span className="sp-editor-color-field-label">{LABELS[colorKey]}</span>
      <Swatch color={color} onClick={() => setPickerOpen(true)} aria-label={`${LABELS[colorKey]} color`} />
      <Button variant="ghost" onClick={() => setPickerOpen(true)}>
        Choose Color…
      </Button>

      {pickerOpen && (
        <ColorPickerWindow
          title={LABELS[colorKey]}
          value={color}
          defaultValue={colorSchemeFor(editorTheme)[colorKey]}
          onChange={(next) => setEditorColor(colorKey, next)}
          onClose={() => setPickerOpen(false)}
        />
      )}
    </div>
  );
}
