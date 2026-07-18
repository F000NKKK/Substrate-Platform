import type { HslColor } from "../color";

/** The theme's accent is just an HslColor — kept as an alias (not a redeclared shape) so theme and widgets never drift apart. */
export type AccentColor = HslColor;

/** Which built-in editor color scheme the text editor starts from — individual `EditorColorKey`s can still be overridden on top (see `editorColors`/`setEditorColor`). */
export type EditorThemeId = "vs-dark" | "classic";

/**
 * Every independently colorable part of the code editor — one shared value
 * per key across every open file/language (e.g. `background` is the same
 * for a `.rs` file as a `.json` file), but each key is its own setting, the
 * same way Visual Studio's "Fonts and Colors" page lets you recolor
 * "Keyword" independently of "Comment" independently of "Selected Text".
 */
export type EditorColorKey =
  | "background"
  | "foreground"
  | "selection"
  | "cursor"
  | "keyword"
  | "string"
  | "comment"
  | "number"
  | "type"
  | "function"
  | "variable"
  | "operator";

export type EditorColorScheme = Record<EditorColorKey, HslColor>;

export interface ThemeContextValue {
  accent: AccentColor;
  setAccent: (accent: AccentColor) => void;
  presets: readonly AccentColor[];
  /** The built-in color-scheme profile last selected — switching this resets every `editorColors` entry to that profile's defaults. */
  editorTheme: EditorThemeId;
  setEditorTheme: (id: EditorThemeId) => void;
  editorThemePresets: readonly EditorThemeId[];
  /** The live, possibly individually-overridden color for every editor category. */
  editorColors: EditorColorScheme;
  setEditorColor: (key: EditorColorKey, color: HslColor) => void;
  resetEditorColors: () => void;
}
