import { useEffect, useMemo, useRef, useState, type InputHTMLAttributes, type KeyboardEvent, type Ref } from "react";
import { createPortal } from "react-dom";
import "./SuggestingInput.css";

/** The most suggestions shown at once. */
const MAX_SHOWN = 200;
/** The popup's gap below its input, pixels. */
const GAP = 2;

export interface SuggestingInputProps extends InputHTMLAttributes<HTMLInputElement> {
  ref?: Ref<HTMLInputElement>;
  /** What the field offers; those containing what's typed are shown. */
  suggestions?: readonly string[];
}

/**
 * A text input with suggestions in the platform's look (not the browser's
 * `<datalist>`, which ignores the theme and closes on its own terms): they
 * open on focus and typing, close on a click outside, a second click,
 * Escape or leaving; arrows move, Enter picks. A pick is typed into the
 * input as the user would, so a controlled input's `onChange` and an
 * uncontrolled one both see it.
 */
export function SuggestingInput({ suggestions, ref, onKeyDown, onFocus, onBlur, onMouseDown, ...props }: SuggestingInputProps) {
  const input = useRef<HTMLInputElement | null>(null);
  const popup = useRef<HTMLDivElement | null>(null);
  const [open, setOpen] = useState(false);
  const [typed, setTyped] = useState("");
  const [at, setAt] = useState(-1);
  const shown = useMemo(() => {
    const q = typed.toLowerCase();
    return (suggestions ?? []).filter((s) => s.toLowerCase().includes(q) && s !== typed).slice(0, MAX_SHOWN);
  }, [suggestions, typed]);
  const visible = open && shown.length > 0;

  // A press outside the input and the popup closes it.
  useEffect(() => {
    if (!visible) return;
    const away = (e: MouseEvent) => {
      const t = e.target as Node;
      if (!input.current?.contains(t) && !popup.current?.contains(t)) setOpen(false);
    };
    document.addEventListener("mousedown", away, true);
    return () => document.removeEventListener("mousedown", away, true);
  }, [visible]);

  const pick = (s: string) => {
    const el = input.current;
    setOpen(false);
    if (!el) return;
    // As if typed: React's onChange fires, controlled or not.
    Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value")?.set?.call(el, s);
    el.dispatchEvent(new Event("input", { bubbles: true }));
    setTyped(s);
  };

  const keys = (e: KeyboardEvent<HTMLInputElement>) => {
    if (visible && (e.key === "ArrowDown" || e.key === "ArrowUp")) {
      e.preventDefault();
      const step = e.key === "ArrowDown" ? 1 : -1;
      setAt((i) => (i + step + shown.length) % shown.length);
      return;
    }
    if (visible && e.key === "Enter" && at >= 0 && at < shown.length) {
      e.preventDefault();
      pick(shown[at]);
      return;
    }
    if (visible && e.key === "Escape") {
      e.preventDefault();
      e.stopPropagation();
      setOpen(false);
      return;
    }
    onKeyDown?.(e);
  };

  const box = visible ? input.current?.getBoundingClientRect() : undefined;
  return (
    <>
      <input
        {...props}
        ref={(el) => {
          input.current = el;
          if (typeof ref === "function") ref(el);
          else if (ref) (ref as { current: HTMLInputElement | null }).current = el;
        }}
        autoComplete="off"
        aria-autocomplete={suggestions ? "list" : undefined}
        aria-expanded={suggestions ? visible : undefined}
        onFocus={(e) => {
          setTyped(e.currentTarget.value);
          setOpen(true);
          onFocus?.(e);
        }}
        onMouseDown={(e) => {
          // A second click on a focused input toggles the list.
          if (document.activeElement === e.currentTarget) setOpen((o) => !o);
          onMouseDown?.(e);
        }}
        onInput={(e) => {
          setTyped(e.currentTarget.value);
          setAt(-1);
          setOpen(true);
        }}
        onKeyDown={keys}
        onBlur={(e) => {
          setOpen(false);
          onBlur?.(e);
        }}
      />
      {visible &&
        box &&
        createPortal(
          <div
            ref={popup}
            className="sp-suggestions"
            role="listbox"
            style={{ left: box.left, top: box.bottom + GAP, minWidth: box.width }}
            // Picking mustn't blur the input first.
            onMouseDown={(e) => e.preventDefault()}
          >
            {shown.map((s, i) => (
              <div key={s} role="option" aria-selected={i === at} className="sp-suggestion" onClick={() => pick(s)} onMouseEnter={() => setAt(i)}>
                {s}
              </div>
            ))}
          </div>,
          document.body,
        )}
    </>
  );
}
