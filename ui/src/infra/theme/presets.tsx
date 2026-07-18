import type { AccentColor, EditorColorKey, EditorColorScheme, EditorThemeId } from "./types";

/** Named accent swatches offered by the color picker, alongside the free color wheel. */
export const accentPresets: readonly AccentColor[] = [
  { h: 152, s: 45, l: 42 }, // emerald — Yog's default brand accent
  { h: 291, s: 55, l: 52 }, // studio purple
  { h: 210, s: 70, l: 52 }, // azure
  { h: 12, s: 68, l: 54 }, // ember
  { h: 44, s: 75, l: 52 }, // amber
];

export const defaultAccent: AccentColor = accentPresets[0];

export const editorThemePresets: readonly EditorThemeId[] = ["vs-dark", "classic"];

export const defaultEditorTheme: EditorThemeId = "vs-dark";

/** Every color key a settings page should list, in the order it should list them — background/foreground/selection/cursor first (the "shared chrome" of the editor), then one row per syntax category. */
export const editorColorKeys: readonly EditorColorKey[] = [
  "background",
  "foreground",
  "selection",
  "cursor",
  "keyword",
  "string",
  "comment",
  "number",
  "type",
  "function",
  "variable",
  "operator",
];

/** Visual Studio Dark+'s own palette, converted to HSL — the "vs-dark" profile's defaults. */
export const vsDarkColorScheme: EditorColorScheme = {
  background: { h: 0, s: 0, l: 12 },
  foreground: { h: 0, s: 0, l: 83 },
  selection: { h: 210, s: 60, l: 45 },
  cursor: { h: 0, s: 0, l: 83 },
  keyword: { h: 207, s: 68, l: 59 },
  string: { h: 15, s: 51, l: 64 },
  comment: { h: 99, s: 28, l: 47 },
  number: { h: 168, s: 51, l: 55 },
  type: { h: 168, s: 51, l: 55 },
  function: { h: 60, s: 53, l: 76 },
  variable: { h: 201, s: 94, l: 80 },
  operator: { h: 0, s: 0, l: 83 },
};

/** The "classic" profile — plain CodeMirror defaults for everything except background/foreground/selection/cursor, which still need *some* value since every editor instance always applies one of these two schemes. */
export const classicColorScheme: EditorColorScheme = {
  background: { h: 220, s: 13, l: 18 },
  foreground: { h: 220, s: 14, l: 71 },
  selection: { h: 210, s: 60, l: 45 },
  cursor: { h: 220, s: 14, l: 71 },
  keyword: { h: 286, s: 60, l: 67 },
  string: { h: 95, s: 38, l: 62 },
  comment: { h: 220, s: 10, l: 45 },
  number: { h: 29, s: 54, l: 61 },
  type: { h: 39, s: 67, l: 69 },
  function: { h: 207, s: 82, l: 66 },
  variable: { h: 0, s: 0, l: 83 },
  operator: { h: 187, s: 47, l: 55 },
};

export function colorSchemeFor(id: EditorThemeId): EditorColorScheme {
  return id === "vs-dark" ? vsDarkColorScheme : classicColorScheme;
}
