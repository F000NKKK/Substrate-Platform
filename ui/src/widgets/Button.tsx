import type { ButtonHTMLAttributes } from "react";
import "./Button.css";

export type ButtonVariant = "primary" | "subtle" | "ghost";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
}

/** The platform's one hand-styled button — every clickable control should be built from this rather than a bare native `<button>`, so style stays consistent everywhere. */
export function Button({ variant = "subtle", className, ...props }: ButtonProps) {
  const classes = ["sp-btn", `sp-btn--${variant}`, className].filter(Boolean).join(" ");
  return <button className={classes} {...props} />;
}
