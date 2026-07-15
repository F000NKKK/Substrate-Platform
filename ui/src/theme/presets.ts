import type { AccentColor } from "./types";

/** Named accent swatches offered by the palette picker, alongside a free hue slider. */
export const accentPresets: readonly AccentColor[] = [
  { h: 291, s: 55, l: 52 }, // studio purple (default)
  { h: 210, s: 70, l: 52 }, // azure
  { h: 152, s: 45, l: 42 }, // emerald
  { h: 12, s: 68, l: 54 }, // ember
  { h: 44, s: 75, l: 52 }, // amber
];

export const defaultAccent: AccentColor = accentPresets[0];
