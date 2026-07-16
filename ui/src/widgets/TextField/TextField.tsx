import type { InputHTMLAttributes } from "react";
import "./TextField.css";

export interface TextFieldProps extends InputHTMLAttributes<HTMLInputElement> {}

/** The platform's one styled text input — any search box or settings field should use this rather than a bare `<input>`. */
export function TextField({ className, ...props }: TextFieldProps) {
  const classes = ["sp-textfield", className].filter(Boolean).join(" ");
  return <input className={classes} {...props} />;
}
