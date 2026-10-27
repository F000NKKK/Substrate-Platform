import type { InputHTMLAttributes, Ref } from "react";
import { SuggestingInput } from "../SuggestingInput";
import "./TextField.css";

export interface TextFieldProps extends InputHTMLAttributes<HTMLInputElement> {
  ref?: Ref<HTMLInputElement>;
  /** What it offers as it's typed in (the platform's list, not the browser's `<datalist>`). */
  suggestions?: readonly string[];
}

/** The platform's one styled text input — any search box or settings field should use this rather than a bare `<input>`. */
export function TextField({ className, suggestions, ...props }: TextFieldProps) {
  const classes = ["sp-textfield", className].filter(Boolean).join(" ");
  if (suggestions) return <SuggestingInput className={classes} suggestions={suggestions} {...props} />;
  return <input className={classes} {...props} />;
}
