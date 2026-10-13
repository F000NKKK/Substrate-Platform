import { useSyncExternalStore, type ReactNode } from "react";
import { parseShortcut } from "../keys";

/**
 * UI localization. Strings are keyed by their English text, gettext-style:
 * `t("Save All")` returns the active locale's translation, or the English
 * itself when there isn't one — so untranslated text still reads fine and
 * nothing needs a separate id. `{name}` placeholders are filled from
 * `params`: `t("Delete profile \"{name}\"?", { name })`.
 *
 * Count-dependent text goes through `tn(count, "{count} file", "{count}
 * files")`: the catalog entry is keyed by the singular English and maps
 * to the target language's plural forms (`Intl.PluralRules` categories,
 * `{ one, few, many, other }`); `{count}` is filled automatically.
 *
 * One English word that needs two translations (the Build menu and its
 * Build command) is told apart by a context, gettext's `pgettext`:
 * `pt("menu", "Build")`, catalogued under `contextKey("menu", "Build")`.
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

/** A translation's plural forms by `Intl.PluralRules` category; `other` is the fallback. */
export type PluralForms = Partial<Record<Intl.LDMLPluralRule, string>> & { other: string };

/** A locale's catalog: English source text → translation (plural forms for `tn` entries). */
export type Messages = Record<string, string | PluralForms>;

const catalogs = new Map<string, Map<string, string | PluralForms>>();
const locales = new Map<string, LocaleInfo>([[SOURCE_LOCALE.id, SOURCE_LOCALE]]);
const subscribers = new Set<() => void>();
let explicit = storedLocale();
let current = explicit ?? systemLocale();

function storedLocale(): string | null {
  try {
    return localStorage.getItem(STORAGE_KEY) || null;
  } catch {
    // storage unavailable — fall back to the system language
    return null;
  }
}

function systemLocale(): string {
  return typeof navigator !== "undefined" && navigator.language ? navigator.language : SOURCE_LOCALE.id;
}

function notify() {
  subscribers.forEach((fn) => fn());
}

if (typeof window !== "undefined") {
  window.addEventListener("storage", (e) => {
    if (e.key !== STORAGE_KEY) return;
    explicit = e.newValue || null;
    const next = explicit ?? systemLocale();
    if (next !== current) {
      current = next;
      notify();
    }
  });
}

/** The catalog for `locale`, falling back from a region to its language (`"ru-RU"` → `"ru"`). */
function catalogFor(locale: string): Map<string, string | PluralForms> | undefined {
  return catalogs.get(locale) ?? catalogs.get(locale.split("-")[0]);
}

/** Adds translations (English text → translated text) for `locale`; `name` lists it in the language picker. */
export function registerTranslations(locale: LocaleInfo | string, messages: Messages): void {
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

/**
 * The registered locale the UI is actually shown in — the active tag, or
 * its language (`"ru-RU"` → `"ru"`), else English. What a picker selects.
 */
export function resolvedLocale(): string {
  if (locales.has(current)) return current;
  const language = current.split("-")[0];
  return locales.has(language) ? language : SOURCE_LOCALE.id;
}

/**
 * Switches the UI language everywhere (every window) and remembers it;
 * `null` forgets the choice and follows the system language again.
 */
export function setLocale(locale: string | null): void {
  explicit = locale;
  current = locale ?? systemLocale();
  try {
    if (locale) localStorage.setItem(STORAGE_KEY, locale);
    else localStorage.removeItem(STORAGE_KEY);
  } catch {
    // storage unavailable — the choice just won't persist
  }
  notify();
}

/** Whether the UI language follows the system's (nothing chosen with `setLocale`). */
export function localeFollowsSystem(): boolean {
  return explicit === null;
}

function fill(text: string, params?: Record<string, string | number>): string {
  if (!params) return text;
  return text.replace(/\{(\w+)\}/g, (match, name: string) => (name in params ? String(params[name]) : match));
}

const pluralRules = new Map<string, Intl.PluralRules>();

function pluralCategory(locale: string, count: number): Intl.LDMLPluralRule {
  let rules = pluralRules.get(locale);
  if (!rules) {
    try {
      rules = new Intl.PluralRules(locale);
    } catch {
      rules = new Intl.PluralRules(SOURCE_LOCALE.id);
    }
    pluralRules.set(locale, rules);
  }
  return rules.select(count);
}

/** `text` in the active locale, `{placeholders}` filled from `params`. */
export function t(text: string, params?: Record<string, string | number>): string {
  const entry = catalogFor(current)?.get(text);
  const translated = entry === undefined ? text : typeof entry === "string" ? entry : entry.other;
  return fill(translated, params);
}

/** The catalog key of `text` in `context` — see `pt`. */
export function contextKey(context: string, text: string): string {
  return `${context}\u0004${text}`;
}

/** `t` for `text` in a `context` (`pt("menu", "Build")`): the context's translation, else the plain one, else the English. */
export function pt(context: string, text: string, params?: Record<string, string | number>): string {
  const entry = catalogFor(current)?.get(contextKey(context, text));
  if (entry === undefined) return t(text, params);
  return fill(typeof entry === "string" ? entry : entry.other, params);
}

/**
 * Count-dependent text: `tn(n, "{count} file", "{count} files")`. The
 * catalog entry (keyed by `one`) gives the locale's plural forms; without
 * one, English rules pick `one` or `other`. `{count}` is `count`.
 */
export function tn(count: number, one: string, other: string, params?: Record<string, string | number>): string {
  const values = { count, ...params };
  const entry = catalogFor(current)?.get(one);
  if (entry === undefined) return fill(count === 1 ? one : other, values);
  if (typeof entry === "string") return fill(entry, values);
  return fill(entry[pluralCategory(resolvedLocale(), count)] ?? entry.other, values);
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

/** `t` as an element, subscribed: `<T>Close</T>`, `<T params={{ name }}>Open {name}</T>`. */
export function T({ children, params }: { children: string; params?: Record<string, string | number> }): ReactNode {
  useLocale();
  return t(children, params);
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
