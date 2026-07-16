/** A plain HSL color — the one shape every color control in the platform speaks, so pickers, swatches, and product-level types (like the theme's AccentColor) all stay interchangeable. */
export interface HslColor {
  h: number;
  s: number;
  l: number;
}

export function hslToCss(c: HslColor): string {
  return `hsl(${c.h} ${c.s}% ${c.l}%)`;
}

export function sameColor(a: HslColor, b: HslColor): boolean {
  return a.h === b.h && a.s === b.s && a.l === b.l;
}

/** h in [0,360), s/l in [0,1] — returns 0-255 RGB channels, for canvas ImageData work where CSS color strings aren't an option. */
export function hslToRgb(h: number, s: number, l: number): [number, number, number] {
  const c = (1 - Math.abs(2 * l - 1)) * s;
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
  const m = l - c / 2;
  let [r, g, b] = [0, 0, 0];
  if (h < 60) [r, g, b] = [c, x, 0];
  else if (h < 120) [r, g, b] = [x, c, 0];
  else if (h < 180) [r, g, b] = [0, c, x];
  else if (h < 240) [r, g, b] = [0, x, c];
  else if (h < 300) [r, g, b] = [x, 0, c];
  else [r, g, b] = [c, 0, x];
  return [Math.round((r + m) * 255), Math.round((g + m) * 255), Math.round((b + m) * 255)];
}
