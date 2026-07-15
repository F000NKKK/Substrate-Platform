import type { HslColor } from "../widgets";

/** The theme's accent is just an HslColor — kept as an alias (not a redeclared shape) so theme and widgets never drift apart. */
export type AccentColor = HslColor;

export interface ThemeContextValue {
  accent: AccentColor;
  setAccent: (accent: AccentColor) => void;
  presets: readonly AccentColor[];
}
