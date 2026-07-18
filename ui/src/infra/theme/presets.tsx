import type { AccentColor, EditorThemeId } from "./types";
import type { HslColor } from "../color";

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

/** Selection-color swatches — deliberately separate palette from `accentPresets`, since the editor's selection highlight and the UI's brand accent are unrelated settings. Default matches Visual Studio Dark's own selection blue. */
export const selectionColorPresets: readonly HslColor[] = [
  { h: 210, s: 60, l: 45 }, // VS Dark blue (default)
  { h: 152, s: 45, l: 42 }, // emerald
  { h: 291, s: 55, l: 52 }, // purple
  { h: 44, s: 75, l: 52 }, // amber
];

export const defaultSelectionColor: HslColor = selectionColorPresets[0];
