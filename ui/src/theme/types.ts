export interface AccentColor {
  h: number;
  s: number;
  l: number;
}

export interface ThemeContextValue {
  accent: AccentColor;
  setAccent: (accent: AccentColor) => void;
  presets: readonly AccentColor[];
}
