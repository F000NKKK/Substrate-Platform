import type { HslColor } from "../color";

/** The theme's accent is just an HslColor — kept as an alias (not a redeclared shape) so theme and widgets never drift apart. */
export type AccentColor = HslColor;

/** Which built-in CodeMirror color profile the text editor renders with — kept separate from the app's own accent, since a code editor's syntax palette and a UI's brand accent serve different purposes and VS itself never conflates them. */
export type EditorThemeId = "vs-dark" | "classic";

export interface ThemeContextValue {
  accent: AccentColor;
  setAccent: (accent: AccentColor) => void;
  presets: readonly AccentColor[];
  /** The editor's syntax-highlighting color profile. */
  editorTheme: EditorThemeId;
  setEditorTheme: (id: EditorThemeId) => void;
  editorThemePresets: readonly EditorThemeId[];
  /** The editor's text-selection highlight color — independent of `accent`. */
  selectionColor: HslColor;
  setSelectionColor: (color: HslColor) => void;
  selectionColorPresets: readonly HslColor[];
}
