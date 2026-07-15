import type { LabelHTMLAttributes } from "react";
import "./Label.css";

export interface LabelProps extends LabelHTMLAttributes<HTMLSpanElement> {}

/** Small muted uppercase caption — every field caption in the platform (Settings, color picker, ...) should use this instead of a one-off styled span. */
export function Label({ className, ...props }: LabelProps) {
  const classes = ["sp-label", className].filter(Boolean).join(" ");
  return <span className={classes} {...props} />;
}
