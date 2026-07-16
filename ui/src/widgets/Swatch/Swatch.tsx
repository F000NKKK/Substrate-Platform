import type { ButtonHTMLAttributes } from "react";
import { hslToCss, type HslColor } from "../../infra/color";
import "./Swatch.css";

export interface SwatchProps extends Omit<ButtonHTMLAttributes<HTMLButtonElement>, "color"> {
  color: HslColor;
  active?: boolean;
}

/** A single round color swatch button — every "pick one of these colors" list (presets, recent colors, ...) renders its buttons through this. */
export function Swatch({ color, active, className, style, ...props }: SwatchProps) {
  const classes = ["sp-swatch", className].filter(Boolean).join(" ");
  return (
    <button
      className={classes}
      data-active={active}
      style={{ background: hslToCss(color), ...style }}
      aria-label={hslToCss(color)}
      {...props}
    />
  );
}
