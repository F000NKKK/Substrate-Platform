import type { InputHTMLAttributes, ReactNode, Ref } from "react";
import { useT } from "../../infra/i18n";
import "./SearchField.css";

/** Visual Studio's three search options — the Aa / ab / .* toggles. */
export interface SearchOptions {
  matchCase: boolean;
  wholeWord: boolean;
  regex: boolean;
}

export interface SearchFieldProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "value" | "onChange"> {
  value: string;
  onValueChange: (value: string) => void;
  /** Shows the Match case / Match whole word / Use regular expressions toggles inside the field. */
  options?: SearchOptions;
  onOptionsChange?: (options: SearchOptions) => void;
  /** Red border — an invalid pattern, or nothing found. */
  invalid?: boolean;
  /** Extra content at the field's end, after the toggles. */
  trailing?: ReactNode;
  inputRef?: Ref<HTMLInputElement>;
}

/**
 * A search box in Quick Find's style: a sunken field whose option toggles
 * (Aa, ab, .*) sit inside it, on the right. Used by Find in Files and any
 * other search UI that should look like one family with the editor's own.
 */
export function SearchField({ value, onValueChange, options, onOptionsChange, invalid, trailing, inputRef, className, ...props }: SearchFieldProps) {
  const t = useT();
  const toggle = (key: keyof SearchOptions, label: ReactNode, title: string) =>
    options && (
      <button
        type="button"
        className="sp-search-field-toggle"
        data-on={options[key] || undefined}
        aria-pressed={options[key]}
        title={title}
        // Keep focus (and the caret) in the text field.
        onMouseDown={(e) => e.preventDefault()}
        onClick={() => onOptionsChange?.({ ...options, [key]: !options[key] })}
      >
        {label}
      </button>
    );
  return (
    <div className={["sp-search-field", className].filter(Boolean).join(" ")} data-invalid={invalid || undefined}>
      <input
        ref={inputRef}
        className="sp-search-field-input"
        spellCheck={false}
        autoComplete="off"
        value={value}
        onChange={(e) => onValueChange(e.target.value)}
        {...props}
      />
      {toggle("matchCase", "Aa", t("Match case (Alt+C)"))}
      {toggle("wholeWord", <span className="sp-search-field-word">ab</span>, t("Match whole word (Alt+W)"))}
      {toggle("regex", ".*", t("Use regular expressions (Alt+E)"))}
      {trailing}
    </div>
  );
}
