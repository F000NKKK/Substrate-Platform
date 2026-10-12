import { useEffect, useRef } from "react";

/**
 * Keyboard shortcuts that don't depend on the keyboard layout: Ctrl+S is
 * the key physically labelled S whether the active layout types "s", "ы"
 * or "σ" there, the way Visual Studio and VS Code behave — nobody has to
 * switch to English to save.
 *
 * Shortcuts are written the way menus show them — `"Ctrl+Shift+S"`,
 * `"Shift+Alt+F"`, `"F5"`, and chords as space-separated steps,
 * `"Ctrl+K V"`. Matching is case-insensitive; `Ctrl` also matches ⌘ on
 * macOS (`Cmd`/`Meta`/`Mod` are accepted as spellings of it).
 */

// The US layout's character for each physical key — what a shortcut's
// letter, digit or punctuation means when the layout types something else.
const US_KEYS: Record<string, string> = {
  Minus: "-",
  Equal: "=",
  BracketLeft: "[",
  BracketRight: "]",
  Backslash: "\\",
  Semicolon: ";",
  Quote: "'",
  Comma: ",",
  Period: ".",
  Slash: "/",
  Backquote: "`",
  Space: "space",
};

const MODIFIER_KEYS = new Set(["Control", "Shift", "Alt", "Meta", "AltGraph", "OS", "Hyper", "Super"]);

const MODIFIER_NAMES: Record<string, string> = {
  ctrl: "ctrl",
  control: "ctrl",
  cmd: "ctrl",
  command: "ctrl",
  meta: "ctrl",
  mod: "ctrl",
  alt: "alt",
  option: "alt",
  shift: "shift",
};

const KEY_ALIASES: Record<string, string> = {
  esc: "escape",
  del: "delete",
  ins: "insert",
  return: "enter",
  up: "arrowup",
  down: "arrowdown",
  left: "arrowleft",
  right: "arrowright",
  " ": "space",
  spacebar: "space",
};

/**
 * The layout-independent name of the key `e` pressed, lowercase: `"s"`,
 * `"7"`, `"/"`, `"f4"`, `"enter"`, `"arrowup"`. Latin letters and digits the
 * layout itself types are taken as is (so AZERTY's or Dvorak's own letters
 * still count); anything else falls back to what the physical key types on
 * a US layout.
 */
export function eventKey(e: Pick<KeyboardEvent, "key" | "code">): string {
  const key = e.key ?? "";
  if (/^[a-z0-9]$/i.test(key)) return key.toLowerCase();
  const letter = /^Key([A-Z])$/.exec(e.code ?? "");
  if (letter) return letter[1].toLowerCase();
  const digit = /^(?:Digit|Numpad)([0-9])$/.exec(e.code ?? "");
  if (digit) return digit[1];
  if (e.code && e.code in US_KEYS) return US_KEYS[e.code];
  const lower = key.toLowerCase();
  return KEY_ALIASES[lower] ?? lower;
}

/** Whether `e` is a lone modifier press (Ctrl, Shift, ...) — never a shortcut by itself. */
export function isModifierOnly(e: Pick<KeyboardEvent, "key">): boolean {
  return MODIFIER_KEYS.has(e.key);
}

/** `e` as one normalized shortcut step: `"ctrl+shift+s"`. */
export function eventShortcut(e: KeyboardEvent): string {
  const parts: string[] = [];
  if (e.ctrlKey || e.metaKey) parts.push("ctrl");
  if (e.altKey) parts.push("alt");
  if (e.shiftKey) parts.push("shift");
  parts.push(eventKey(e));
  return parts.join("+");
}

function normalizeStep(step: string): string {
  // `+` is a separator except as the key itself ("Ctrl++", "Ctrl+Shift++").
  const tokens = step.trim().split(/\+(?!$)/).map((t) => t.trim().toLowerCase());
  const key = tokens.pop() ?? "";
  const mods = new Set(tokens.map((t) => MODIFIER_NAMES[t] ?? t));
  const parts = ["ctrl", "alt", "shift"].filter((m) => mods.has(m));
  parts.push(KEY_ALIASES[key] ?? key);
  return parts.join("+");
}

/** A shortcut's steps, normalized: `"Ctrl+K V"` → `["ctrl+k", "v"]`. */
export function parseShortcut(shortcut: string): string[] {
  return shortcut.trim().split(/\s+/).filter(Boolean).map(normalizeStep);
}

/** Whether `e` is the (single-step) `shortcut`, in any keyboard layout. */
export function matchesShortcut(e: KeyboardEvent, shortcut: string): boolean {
  const steps = parseShortcut(shortcut);
  return steps.length === 1 && steps[0] === eventShortcut(e);
}

// -- Global keybindings ------------------------------------------------------

export interface Keybinding {
  /** Names the command, so a user keymap (`setKeymapOverrides`) can rebind it and menus can show its shortcut. */
  id?: string;
  /** Default shortcut(s), e.g. `"Ctrl+S"` or `["Ctrl+F4", "Ctrl+W"]`; chords as `"Ctrl+K V"`. */
  keys: string | string[];
  /** Runs the command; returning `false` declines (the key goes on to whatever else handles it). */
  run: (e: KeyboardEvent) => void | boolean;
  /** Only while this holds, checked on every press. */
  when?: () => boolean;
}

interface Registered {
  binding: Keybinding;
  steps: string[][];
}

// Module-level, like the selection service: bindings come from unrelated
// components (editor actions, the debug session, a single tab) with one
// window listener between them. Later registrations win ties, so a binding
// added by the focused part of the UI overrides a shell-wide one.
const registered: Registered[] = [];
let overrides: Record<string, string[]> = {};
let pending: string | null = null;
let pendingAt = 0;
let listening = false;

const CHORD_TIMEOUT_MS = 2000;

function stepsOf(binding: Keybinding): string[][] {
  const keys = (binding.id && overrides[binding.id]) || binding.keys;
  return (Array.isArray(keys) ? keys : [keys]).map(parseShortcut).filter((s) => s.length > 0);
}

/**
 * Inside an element marked `data-sp-native-keys` (a terminal),
 * Ctrl+<character> steps go to it rather than to keybindings — the shell
 * owns Ctrl+W and Ctrl+K, the terminal Ctrl+Shift+C/V — while function
 * keys (F5, Ctrl+F4, ...) still reach the IDE, as in VS Code's terminal.
 */
function inNativeKeys(e: KeyboardEvent): boolean {
  return e.target instanceof Element && !!e.target.closest("[data-sp-native-keys]");
}

function onKeyDown(e: KeyboardEvent) {
  if (e.defaultPrevented || e.isComposing || isModifierOnly(e)) return;
  const step = eventShortcut(e);
  const prefix = pending && Date.now() - pendingAt < CHORD_TIMEOUT_MS ? pending : null;
  pending = null;
  const active = registered.filter((r) => !r.binding.when || r.binding.when());
  const run = (wanted: string[]) => {
    for (let i = active.length - 1; i >= 0; i--) {
      const r = active[i];
      if (!r.steps.some((s) => s.length === wanted.length && s.every((x, j) => x === wanted[j]))) continue;
      if (r.binding.run(e) === false) continue;
      e.preventDefault();
      e.stopPropagation();
      return true;
    }
    return false;
  };
  if (inNativeKeys(e) && /^ctrl\+(alt\+)?(shift\+)?[^+]$/.test(step)) return;
  if (prefix) {
    if (run([prefix, step])) return;
    // An unbound second step falls through as an ordinary key press.
  }
  // A chord's first step isn't consumed: the focused editor may have chords
  // of its own starting the same way (Ctrl+K Ctrl+C), and sees it too.
  if (active.some((r) => r.steps.some((s) => s.length > 1 && s[0] === step))) {
    pending = step;
    pendingAt = Date.now();
    return;
  }
  run([step]);
}

/** Adds a window-wide keybinding (capture phase, ahead of editors and inputs); returns its removal. */
export function registerKeybinding(binding: Keybinding): () => void {
  const entry: Registered = { binding, steps: stepsOf(binding) };
  registered.push(entry);
  if (!listening) {
    window.addEventListener("keydown", onKeyDown, true);
    listening = true;
  }
  return () => {
    const index = registered.indexOf(entry);
    if (index >= 0) registered.splice(index, 1);
  };
}

/**
 * `registerKeybinding` for a component's lifetime. `run` and `when` are
 * read live, so they can close over fresh state without re-registering;
 * `enabled: false` drops the binding entirely.
 */
export function useKeybinding(
  keys: string | string[],
  run: (e: KeyboardEvent) => void | boolean,
  options: { id?: string; when?: () => boolean; enabled?: boolean } = {},
): void {
  const runRef = useRef(run);
  runRef.current = run;
  const whenRef = useRef(options.when);
  whenRef.current = options.when;
  const keysKey = Array.isArray(keys) ? keys.join("\n") : keys;
  const enabled = options.enabled ?? true;
  useEffect(() => {
    if (!enabled) return;
    return registerKeybinding({
      id: options.id,
      keys: keysKey.split("\n"),
      run: (e) => runRef.current(e),
      when: () => whenRef.current?.() ?? true,
    });
  }, [keysKey, enabled, options.id]);
}

/** A user keymap: command id → its shortcuts, replacing the defaults (an empty list unbinds it). */
export function setKeymapOverrides(map: Record<string, string[]>): void {
  overrides = { ...map };
  for (const r of registered) r.steps = stepsOf(r.binding);
}

/** The shortcut a command id is bound to right now, as written (for menus) — the user's override, else its default. */
export function keybindingLabel(id: string): string | undefined {
  if (overrides[id]) return overrides[id][0];
  const r = registered.find((r) => r.binding.id === id);
  if (!r) return undefined;
  const keys = r.binding.keys;
  return Array.isArray(keys) ? keys[0] : keys;
}

// -- Text fields -------------------------------------------------------------

/**
 * Plain `<input>`s and `<textarea>`s get their Ctrl+A/C/X/V/Z/Y from the
 * webview, which matches them by the character typed — so under another
 * layout (Ctrl+Ф, Ctrl+С, ...) they're done here instead. Code editors
 * (contenteditable, not inputs) and terminals handle their own keys.
 */
function onTextFieldKeyDown(e: KeyboardEvent) {
  if (e.defaultPrevented || !(e.ctrlKey || e.metaKey) || e.altKey) return;
  const field = e.target;
  if (!(field instanceof HTMLInputElement || field instanceof HTMLTextAreaElement)) return;
  if (field.closest("[data-sp-native-keys]")) return;
  const latin = eventKey(e);
  if (latin.length !== 1 || latin === e.key.toLowerCase()) return;
  const shift = e.shiftKey;
  let done = true;
  if (latin === "a" && !shift) field.select();
  else if (latin === "c" && !shift) document.execCommand("copy");
  else if (latin === "x" && !shift) document.execCommand("cut");
  else if (latin === "v" && !shift) {
    navigator.clipboard
      .readText()
      .then((text) => {
        field.focus();
        document.execCommand("insertText", false, text);
      })
      .catch(() => {});
  } else if (latin === "z") document.execCommand(shift ? "redo" : "undo");
  else if (latin === "y" && !shift) document.execCommand("redo");
  else done = false;
  if (done) e.preventDefault();
}

if (typeof window !== "undefined") window.addEventListener("keydown", onTextFieldKeyDown);
