import { useSyncExternalStore } from "react";
import { parseShortcut } from "../keys";

/**
 * UI localization. Strings are keyed by their English text, gettext-style:
 * `t("Save All")` returns the active locale's translation, or the English
 * itself when there isn't one — so untranslated text still reads fine and
 * nothing needs a separate id. `{name}` placeholders are filled from
 * `params`: `t("Delete profile \"{name}\"?", { name })`.
 *
 * Products and the platform register catalogs per locale
 * (`registerTranslations("ru", {...})`); several registrations for one
 * locale merge, later ones winning. The chosen locale persists in
 * localStorage and follows across windows (Settings is its own window).
 */

export interface LocaleInfo {
  /** BCP 47 tag: `"en"`, `"ru"`, `"pt-BR"`. */
  id: string;
  /** The language's own name for itself: `"English"`, `"Русский"`. */
  name: string;
}

const STORAGE_KEY = "substrate.locale";
const SOURCE_LOCALE: LocaleInfo = { id: "en", name: "English" };

const catalogs = new Map<string, Map<string, string>>();
const locales = new Map<string, LocaleInfo>([[SOURCE_LOCALE.id, SOURCE_LOCALE]]);
const subscribers = new Set<() => void>();
let current = initialLocale();

function initialLocale(): string {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) return stored;
  } catch {
    // storage unavailable — fall back to the system language
  }
  return typeof navigator !== "undefined" ? navigator.language : SOURCE_LOCALE.id;
}

function notify() {
  subscribers.forEach((fn) => fn());
}

if (typeof window !== "undefined") {
  window.addEventListener("storage", (e) => {
    if (e.key === STORAGE_KEY && e.newValue && e.newValue !== current) {
      current = e.newValue;
      notify();
    }
  });
}

/** The catalog for `locale`, falling back from a region to its language (`"ru-RU"` → `"ru"`). */
function catalogFor(locale: string): Map<string, string> | undefined {
  return catalogs.get(locale) ?? catalogs.get(locale.split("-")[0]);
}

/** Adds translations (English text → translated text) for `locale`; `name` lists it in the language picker. */
export function registerTranslations(locale: LocaleInfo | string, messages: Record<string, string>): void {
  const info = typeof locale === "string" ? { id: locale, name: locales.get(locale)?.name ?? locale } : locale;
  locales.set(info.id, info);
  let catalog = catalogs.get(info.id);
  if (!catalog) catalogs.set(info.id, (catalog = new Map()));
  for (const [source, translated] of Object.entries(messages)) catalog.set(source, translated);
  notify();
}

/** The locales a picker can offer: English plus every one with registered translations. */
export function availableLocales(): LocaleInfo[] {
  return [...locales.values()];
}

/** The active locale tag (as chosen or detected, e.g. `"ru-RU"`). */
export function getLocale(): string {
  return current;
}

/** Switches the UI language everywhere (every window) and remembers it. */
export function setLocale(locale: string): void {
  current = locale;
  try {
    localStorage.setItem(STORAGE_KEY, locale);
  } catch {
    // storage unavailable — the choice just won't persist
  }
  notify();
}

/** `text` in the active locale, `{placeholders}` filled from `params`. */
export function t(text: string, params?: Record<string, string | number>): string {
  const translated = catalogFor(current)?.get(text) ?? text;
  if (!params) return translated;
  return translated.replace(/\{(\w+)\}/g, (match, name: string) => (name in params ? String(params[name]) : match));
}

let version = 0;
subscribers.add(() => version++);

/**
 * Re-renders the calling component when the locale or a catalog changes;
 * returns the active locale. Components that call `t()` while rendering
 * use this (or `useT`) so switching language updates them live.
 */
export function useLocale(): string {
  useSyncExternalStore(
    (fn) => {
      subscribers.add(fn);
      return () => subscribers.delete(fn);
    },
    () => version,
  );
  return current;
}

/** `t`, subscribed — `const t = useT();` at the top of a component. */
export function useT(): typeof t {
  useLocale();
  return t;
}

const KEY_LABELS: Record<string, string> = {
  ctrl: "Ctrl",
  alt: "Alt",
  shift: "Shift",
  escape: "Esc",
  arrowup: "Up",
  arrowdown: "Down",
  arrowleft: "Left",
  arrowright: "Right",
  pageup: "PageUp",
  pagedown: "PageDown",
  space: "Space",
  enter: "Enter",
  delete: "Del",
  insert: "Ins",
  backspace: "Backspace",
  tab: "Tab",
  pause: "Pause",
  cancel: "Break",
};

/**
 * A shortcut as menus show it in the active locale — key names go through
 * `t` (so a catalog can say "Strg" for "Ctrl"), letters upper-cased,
 * chords space-separated: `"ctrl+k v"` → `"Ctrl+K V"`.
 */
export function formatShortcut(shortcut: string): string {
  return parseShortcut(shortcut)
    .map((step) =>
      step
        .split(/\+(?!$)/)
        .map((part) => {
          const label = KEY_LABELS[part] ?? (/^f\d+$/.test(part) ? part.toUpperCase() : part.length === 1 ? part.toUpperCase() : part[0].toUpperCase() + part.slice(1));
          return t(label);
        })
        .join("+"),
    )
    .join(" ");
}
