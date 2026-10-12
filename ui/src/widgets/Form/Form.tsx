import { useEffect, useState, type ReactNode } from "react";
import { TextField } from "../TextField";
import { IconButton } from "../IconButton";
import { Button } from "../Button";
import { Icon } from "../../infra/icons";
import { useT } from "../../infra/i18n";
import "./Form.css";

/**
 * Building blocks for settings/properties pages laid out as forms (Visual
 * Studio's project property pages, an Options page): a page, titled
 * sections, labelled fields with a hint and an inline error, and inputs
 * that commit on Enter/blur — so a value is only written once it's whole,
 * and a rejected one stays in the field with the reason under it.
 */

/** What a commit handler returns: nothing (accepted), or the reason it was rejected. */
export type CommitResult = void | string | null | undefined;

export function FormPage({ title, description, actions, children }: { title?: string; description?: ReactNode; actions?: ReactNode; children: ReactNode }) {
  return (
    <div className="sp-form-page">
      {(title || actions) && (
        <div className="sp-form-page-header">
          {title && <h2 className="sp-form-page-title">{title}</h2>}
          {actions && <div className="sp-form-page-actions">{actions}</div>}
        </div>
      )}
      {description && <p className="sp-form-page-description">{description}</p>}
      {children}
    </div>
  );
}

export function FormSection({ title, description, actions, children }: { title: string; description?: ReactNode; actions?: ReactNode; children: ReactNode }) {
  return (
    <section className="sp-form-section">
      <div className="sp-form-section-header">
        <h3 className="sp-form-section-title">{title}</h3>
        {actions && <div className="sp-form-section-actions">{actions}</div>}
      </div>
      {description && <p className="sp-form-section-description">{description}</p>}
      <div className="sp-form-section-body">{children}</div>
    </section>
  );
}

export interface FormFieldProps {
  label: string;
  /** Help for the field — the label's tooltip, and shown under the control while it's being edited. */
  hint?: ReactNode;
  /** Shown under the control in the danger color (replaces nothing — the hint stays). */
  error?: string | null;
  /** A field required for the form to make sense — marks the label. */
  required?: boolean;
  children: ReactNode;
}

/** One labelled row: the label on the left, the control (plus hint/error) on the right. */
export function FormField({ label, hint, error, required, children }: FormFieldProps) {
  return (
    <div className={`sp-form-field${error ? " has-error" : ""}`}>
      <div className="sp-form-field-label" title={typeof hint === "string" ? hint : undefined}>
        {label}
        {required && <span className="sp-form-field-required"> *</span>}
      </div>
      <div className="sp-form-field-control">
        {children}
        {error && (
          <div className="sp-form-field-error" role="alert">
            {error}
          </div>
        )}
        {hint && <div className="sp-form-field-hint">{hint}</div>}
      </div>
    </div>
  );
}

/** Runs a commit handler, turning a throw or a rejection into its message. */
async function runCommit(commit: () => CommitResult | Promise<CommitResult>): Promise<string | null> {
  try {
    const result = await commit();
    return typeof result === "string" && result ? result : null;
  } catch (err) {
    return err instanceof Error ? err.message : String(err);
  }
}

export interface CommitTextFieldProps {
  value: string;
  /** Called with the edited text on Enter or blur, when it changed. Return/throw an error message to reject it — the draft and the message stay. */
  onCommit: (value: string) => CommitResult | Promise<CommitResult>;
  /** Checked as you type — an error here also blocks the commit. */
  validate?: (value: string) => string | null;
  placeholder?: string;
  monospace?: boolean;
  multiline?: boolean;
  disabled?: boolean;
  /** Shows errors itself (default) — turn off when a `FormField` around it shows `onError`'s message. */
  onError?: (error: string | null) => void;
  "aria-label"?: string;
}

/** A text input that writes its value only once you're done with it (Enter or leaving the field); Escape reverts. */
export function CommitTextField({ value, onCommit, validate, placeholder, monospace, multiline, disabled, onError, ...rest }: CommitTextFieldProps) {
  const [draft, setDraft] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const shown = draft ?? value;

  // A new value from outside (another field's edit re-read the file) drops a stale draft.
  useEffect(() => {
    setDraft(null);
    setError(null);
  }, [value]);

  useEffect(() => onError?.(error), [error, onError]);

  async function commit() {
    if (draft === null || draft === value) {
      setDraft(null);
      setError(null);
      return;
    }
    const invalid = validate?.(draft) ?? null;
    if (invalid) {
      setError(invalid);
      return;
    }
    const failed = await runCommit(() => onCommit(draft));
    setError(failed);
    if (!failed) setDraft(null);
  }

  function change(text: string) {
    setDraft(text);
    setError(validate?.(text) ?? null);
  }

  const className = ["sp-form-input", monospace ? "is-mono" : "", error ? "is-invalid" : ""].filter(Boolean).join(" ");
  const common = {
    value: shown,
    placeholder,
    disabled,
    "aria-label": rest["aria-label"],
    "aria-invalid": !!error,
    onBlur: () => void commit(),
  };
  return (
    <div className="sp-form-input-wrap">
      {multiline ? (
        <textarea
          {...common}
          className={`sp-textfield ${className} sp-form-textarea`}
          rows={3}
          onChange={(e) => change(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Escape") {
              setDraft(null);
              setError(null);
            } else if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) void commit();
          }}
        />
      ) : (
        <TextField
          {...common}
          className={className}
          onChange={(e) => change(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") void commit();
            else if (e.key === "Escape") {
              setDraft(null);
              setError(null);
            }
          }}
        />
      )}
      {error && !onError && <div className="sp-form-field-error">{error}</div>}
    </div>
  );
}

export interface StringListFieldProps {
  value: readonly string[];
  /** The whole new list — return/throw a message to reject it. */
  onCommit: (value: string[]) => CommitResult | Promise<CommitResult>;
  validateItem?: (item: string) => string | null;
  placeholder?: string;
  addLabel?: string;
  monospace?: boolean;
}

/** An editable list of strings — one committed field per item, remove buttons, and an "Add" row. */
export function StringListField({ value, onCommit, validateItem, placeholder, addLabel, monospace }: StringListFieldProps) {
  const t = useT();
  const [adding, setAdding] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function commitList(next: string[]): Promise<CommitResult> {
    const failed = await runCommit(() => onCommit(next));
    setError(failed);
    return failed;
  }

  return (
    <div className="sp-form-list">
      {value.map((item, i) => (
        <div key={`${i}:${item}`} className="sp-form-list-row">
          <CommitTextField
            value={item}
            monospace={monospace}
            validate={validateItem}
            aria-label={`${i + 1}`}
            onCommit={(text) => (text.trim() ? commitList(value.map((v, j) => (j === i ? text : v))) : commitList(value.filter((_, j) => j !== i)))}
          />
          <IconButton size={24} title={t("Remove")} aria-label={t("Remove")} onClick={() => void commitList(value.filter((_, j) => j !== i))}>
            <Icon name="trash" size={14} />
          </IconButton>
        </div>
      ))}
      {adding ? (
        <div className="sp-form-list-row">
          <CommitTextField
            value=""
            monospace={monospace}
            placeholder={placeholder}
            validate={validateItem}
            onCommit={async (text) => {
              if (!text.trim()) {
                setAdding(false);
                return;
              }
              const failed = await commitList([...value, text]);
              if (!failed) setAdding(false);
              return failed;
            }}
          />
          <IconButton size={24} title={t("Cancel")} aria-label={t("Cancel")} onClick={() => setAdding(false)}>
            <Icon name="close" size={14} />
          </IconButton>
        </div>
      ) : (
        <div>
          <Button variant="ghost" className="sp-form-list-add" onClick={() => setAdding(true)}>
            <Icon name="plus" size={12} /> {addLabel ?? t("Add")}
          </Button>
        </div>
      )}
      {error && <div className="sp-form-field-error">{error}</div>}
    </div>
  );
}

export function CheckField({ checked, onChange, label, disabled }: { checked: boolean; onChange: (checked: boolean) => void; label: ReactNode; disabled?: boolean }) {
  return (
    <label className={`sp-form-check${disabled ? " is-disabled" : ""}`}>
      <input type="checkbox" checked={checked} disabled={disabled} onChange={(e) => onChange(e.target.checked)} />
      <span>{label}</span>
    </label>
  );
}

export interface SelectOption {
  value: string;
  label: string;
}

export function SelectField({ value, options, onChange, disabled, "aria-label": ariaLabel }: { value: string; options: readonly SelectOption[]; onChange: (value: string) => void; disabled?: boolean; "aria-label"?: string }) {
  return (
    <select className="sp-form-select" value={value} disabled={disabled} aria-label={ariaLabel} onChange={(e) => onChange(e.target.value)}>
      {options.map((o) => (
        <option key={o.value} value={o.value}>
          {o.label}
        </option>
      ))}
    </select>
  );
}
