import type { ButtonHTMLAttributes } from "react";
import "./IconButton.css";

export interface IconButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  size?: number;
}

/** A small square icon-only ghost button — every pin/close/float/settings-trigger control in the platform should use this instead of a bespoke `.xxx-btn` class per component. */
export function IconButton({ size = 20, className, style, ...props }: IconButtonProps) {
  const classes = ["sp-icon-btn", className].filter(Boolean).join(" ");
  return <button className={classes} style={{ width: size, height: size, ...style }} {...props} />;
}
