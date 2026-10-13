import type { ButtonHTMLAttributes } from "react";
import "./IconButton.css";

export interface IconButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  /**
   * Hit-box size. `"md"` (default) is the 24px command button for toolbars, strips and panel-header actions;
   * `"sm"` is the 20px box for tool-window captions, tabs and dense rows. A number is an explicit pixel box
   * (only for controls that must line up with something of a fixed size, e.g. a 28px input). The glyph inside
   * is always sized by `--sp-icon-size`, regardless of the `size` the `<Icon>` was given.
   */
  size?: "md" | "sm" | number;
}

/** A square icon-only ghost button — every pin/close/float/settings-trigger control in the platform should use this instead of a bespoke `.xxx-btn` class per component. */
export function IconButton({ size = "md", className, style, ...props }: IconButtonProps) {
  const classes = ["sp-icon-btn", size === "sm" && "sp-icon-btn--sm", className].filter(Boolean).join(" ");
  const box = typeof size === "number" ? { width: size, height: size } : undefined;
  return <button type="button" className={classes} style={box || style ? { ...box, ...style } : undefined} {...props} />;
}
