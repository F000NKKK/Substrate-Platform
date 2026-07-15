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
