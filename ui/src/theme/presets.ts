import type { AccentColor } from "./types";

/** Named accent swatches offered by the color picker, alongside the free color wheel. */
export const accentPresets: readonly AccentColor[] = [
  { h: 152, s: 45, l: 42 }, // emerald — Yog's default brand accent
  { h: 291, s: 55, l: 52 }, // studio purple
  { h: 210, s: 70, l: 52 }, // azure
  { h: 12, s: 68, l: 54 }, // ember
  { h: 44, s: 75, l: 52 }, // amber
];

export const defaultAccent: AccentColor = accentPresets[0];
