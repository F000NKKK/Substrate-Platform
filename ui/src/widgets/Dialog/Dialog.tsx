import { useState, useSyncExternalStore, type KeyboardEvent as ReactKeyboardEvent, type ReactNode } from "react";
import { Button, type ButtonVariant } from "../Button";
import { TextField } from "../TextField";
import { ViewportPortal } from "../../infra/portal";
import { t } from "../../infra/i18n";
import "./Dialog.css";

export interface DialogButton {
  /** What `showDialog` resolves with when this button is chosen. */
  id: string;
  label: string;
  /** Default `"subtle"`; the `primary` button of a dialog is also what Enter activates unless another one is marked `default`. */
  variant?: ButtonVariant;
  /** Activated by Enter (default: the first `primary` button). */
  default?: boolean;
  /** Activated by Escape (default: a button with id `"cancel"`, if any). */
  cancel?: boolean;
}

export interface DialogInput {
  label?: string;
  defaultValue?: string;
  placeholder?: string;
  /** Returns an error message to show, which also disables confirming (`""` blocks without a message — e.g. for an empty name), or `null` when the value is acceptable. */
  validate?: (value: string) => string | null;
}

export interface DialogOptions {
  title: string;
  message?: ReactNode;
  /** Adds a single-line text input — the dialog resolves with its value. */
  input?: DialogInput;
  /** Left to right; at least one. */
  buttons: DialogButton[];
}

export interface DialogResult {
  /** The chosen button's `id` — the `cancel` button's (or `"cancel"`) when dismissed with Escape. */
  button: string;
  /** The text input's value (`""` when the dialog has no input). */
  value: string;
}

interface PendingDialog {
  key: number;
  options: DialogOptions;
  resolve: (result: DialogResult) => void;
}

// Module-level, like `outputBuffer`: a dialog can be requested from anywhere
// (an event handler, a context provider, a plain async function), not just
// from components that could reach a React context — `DialogHost` is the one
// place that renders whatever's queued.
let queue: PendingDialog[] = [];
let nextKey = 0;
const subscribers = new Set<() => void>();

function setQueue(next: PendingDialog[]) {
  queue = next;
  subscribers.forEach((notify) => notify());
}

/**
 * Shows an in-app modal dialog and resolves once the user picks a button —
 * the platform's replacement for `window.alert`/`confirm`/`prompt` (which a
 * Tauri webview renders as an unstyled OS box, or not at all). Dialogs queue:
 * a second request waits until the first is answered. Requires a mounted
 * `DialogHost` (`PlatformShell` mounts one).
 */
export function showDialog(options: DialogOptions): Promise<DialogResult> {
  return new Promise((resolve) => {
    setQueue([...queue, { key: nextKey++, options, resolve }]);
  });
}

/** Asks a yes/no question — resolves `true` only for the confirm button. */
export async function confirmDialog(options: {
  title: string;
  message?: ReactNode;
  confirmLabel?: string;
  cancelLabel?: string;
}): Promise<boolean> {
  const result = await showDialog({
    title: options.title,
    message: options.message,
    buttons: [
      { id: "cancel", label: options.cancelLabel ?? t("Cancel") },
      { id: "confirm", label: options.confirmLabel ?? t("OK"), variant: "primary" },
    ],
  });
  return result.button === "confirm";
}

/** Asks for one line of text — resolves `null` when cancelled. */
export async function promptDialog(options: {
  title: string;
  message?: ReactNode;
  label?: string;
  defaultValue?: string;
  placeholder?: string;
  confirmLabel?: string;
  validate?: (value: string) => string | null;
}): Promise<string | null> {
  const result = await showDialog({
    title: options.title,
    message: options.message,
    input: { label: options.label, defaultValue: options.defaultValue, placeholder: options.placeholder, validate: options.validate },
    buttons: [
      { id: "cancel", label: t("Cancel") },
      { id: "confirm", label: options.confirmLabel ?? t("OK"), variant: "primary" },
    ],
  });
  return result.button === "confirm" ? result.value : null;
}

/** Shows a message with a single dismiss button. */
export async function messageDialog(options: { title: string; message?: ReactNode; okLabel?: string }): Promise<void> {
  await showDialog({ title: options.title, message: options.message, buttons: [{ id: "ok", label: options.okLabel ?? t("OK"), variant: "primary", cancel: true }] });
}

function subscribe(notify: () => void) {
  subscribers.add(notify);
  return () => {
    subscribers.delete(notify);
  };
}

/** Renders the front of `showDialog`'s queue — mount exactly one per app (`PlatformShell` already does). */
export function DialogHost() {
  const pending = useSyncExternalStore(subscribe, () => queue);
  const current = pending[0];
  if (!current) return null;
  return (
    <DialogView
      key={current.key}
      options={current.options}
      onDone={(result) => {
        setQueue(queue.filter((d) => d !== current));
        current.resolve(result);
      }}
    />
  );
}

function DialogView({ options, onDone: resolve }: { options: DialogOptions; onDone: (result: DialogResult) => void }) {
  const { title, message, input, buttons } = options;
  // Whatever had focus before (typically an editor) gets it back afterward.
  const [returnFocus] = useState(() => (document.activeElement instanceof HTMLElement ? document.activeElement : null));
  function onDone(result: DialogResult) {
    resolve(result);
    returnFocus?.focus();
  }
  const [value, setValue] = useState(input?.defaultValue ?? "");
  const error = input?.validate?.(value) ?? null;

  const defaultButton = buttons.find((b) => b.default) ?? buttons.find((b) => b.variant === "primary");
  const cancelButton = buttons.find((b) => b.cancel) ?? buttons.find((b) => b.id === "cancel");

  function choose(button: DialogButton) {
    // Only the confirming choice is held back by a validation error — Cancel
    // or Don't Save must always work.
    if (error !== null && button === defaultButton) return;
    onDone({ button: button.id, value });
  }

  function handleKeyDown(e: ReactKeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      e.stopPropagation();
      onDone({ button: cancelButton?.id ?? "cancel", value });
    } else if (e.key === "Enter" && defaultButton && e.target instanceof HTMLInputElement) {
      // A focused button handles Enter itself (native click) — only the text
      // input needs Enter forwarded to the default button.
      e.preventDefault();
      choose(defaultButton);
    }
  }

  return (
    <ViewportPortal>
      <div className="sp-dialog-backdrop" onPointerDown={(e) => e.stopPropagation()}>
        <div className="sp-dialog" role="dialog" aria-modal="true" aria-label={title} onKeyDown={handleKeyDown}>
          <div className="sp-dialog-title">{title}</div>
          {message && <div className="sp-dialog-message">{message}</div>}
          {input && (
            <label className="sp-dialog-field">
              {input.label && <span className="sp-dialog-field-label">{input.label}</span>}
              <TextField
                autoFocus
                value={value}
                placeholder={input.placeholder}
                onFocus={(e) => e.target.select()}
                onChange={(e) => setValue(e.target.value)}
              />
              {error && <span className="sp-dialog-error">{error}</span>}
            </label>
          )}
          <div className="sp-dialog-buttons">
            {buttons.map((button) => (
              <Button
                key={button.id}
                // With no text input, the default button takes focus so Enter
                // (natively) and Tab navigation work straight away.
                autoFocus={!input && button === defaultButton}
                variant={button.variant ?? "subtle"}
                disabled={error !== null && button === defaultButton}
                onClick={() => choose(button)}
              >
                {button.label}
              </Button>
            ))}
          </div>
        </div>
      </div>
    </ViewportPortal>
  );
}
