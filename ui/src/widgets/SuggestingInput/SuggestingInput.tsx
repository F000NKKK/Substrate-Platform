import { useEffect, useMemo, useRef, useState, type InputHTMLAttributes, type KeyboardEvent, type Ref } from "react";
import { createPortal } from "react-dom";
import { useViewportPosition } from "../../infra/portal";
import "./SuggestingInput.css";

/** The most suggestions shown at once (a game's items and a mod's, all of them). */
const MAX_SHOWN = 2000;
/** The popup's gap below its input, pixels. */
const GAP = 2;

export interface SuggestingInputProps extends InputHTMLAttributes<HTMLInputElement> {
  ref?: Ref<HTMLInputElement>;
  /** What the field offers: all of them until something's typed, then those containing it. */
  suggestions?: readonly string[];
}

/**
 * A text input with suggestions in the platform's look (not the browser's
 * `<datalist>`, which ignores the theme and closes on its own terms): they
 * open on focus (all of them, the current value marked, as Visual
 * Studio's lists) and narrow as one types; close on a click outside, a second click,
 * Escape or leaving; arrows move, Enter picks. A pick is typed into the
 * input as the user would, so a controlled input's `onChange` and an
 * uncontrolled one both see it.
 */
export function SuggestingInput({ suggestions, ref, onKeyDown, onFocus, onBlur, onMouseDown, ...props }: SuggestingInputProps) {
  const input = useRef<HTMLInputElement | null>(null);
  const [open, setOpen] = useState(false);
  // What's been typed since the field was entered; none: show every suggestion.
  const [typed, setTyped] = useState<string | null>(null);
  const [at, setAt] = useState(-1);
  const shown = useMemo(() => {
    const all = suggestions ?? [];
    if (typed === null) return all.slice(0, MAX_SHOWN);
    const q = typed.toLowerCase();
    return all.filter((s) => s.toLowerCase().includes(q) && s !== typed).slice(0, MAX_SHOWN);
  }, [suggestions, typed]);
  const visible = open && shown.length > 0;
  // Under the input, kept inside the window.
  const box = visible ? input.current?.getBoundingClientRect() : undefined;
  const { ref: popup, style: placed } = useViewportPosition(box ? { x: box.left, y: box.bottom + GAP } : null);

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

  // The marked one in view (the current value on entering, the arrows' after).
  useEffect(() => {
    if (visible && at >= 0) popup.current?.children[at]?.scrollIntoView({ block: "nearest" });
  }, [visible, at]);

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
          setTyped(null);
          setAt((suggestions ?? []).slice(0, MAX_SHOWN).indexOf(e.currentTarget.value));
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
            style={{ ...placed, minWidth: box.width }}
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
