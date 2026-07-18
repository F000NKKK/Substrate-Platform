import type { HslColor } from "../color";
import type { AccentColor, EditorColorScheme, EditorThemeId } from "./types";
import { editorColorKeys } from "./presets";

const ACCENT_KEY = "substrate-platform:accent";
const EDITOR_THEME_KEY = "substrate-platform:editor-theme";
const EDITOR_COLORS_KEY = "substrate-platform:editor-colors";

function isHslColor(value: unknown): value is HslColor {
  const v = value as Record<string, unknown> | null;
  return typeof v?.h === "number" && typeof v?.s === "number" && typeof v?.l === "number";
}

export function loadStoredAccent(): AccentColor | null {
  try {
    const raw = localStorage.getItem(ACCENT_KEY);
    if (!raw) return null;
    const parsed = JSON.parse(raw);
    return isHslColor(parsed) ? parsed : null;
  } catch {
    return null;
  }
}

export function storeAccent(accent: AccentColor): void {
  try {
    localStorage.setItem(ACCENT_KEY, JSON.stringify(accent));
  } catch {
    // storage unavailable (private mode, etc.) — theme just won't persist
  }
}

export function loadStoredEditorTheme(): EditorThemeId | null {
  try {
    const raw = localStorage.getItem(EDITOR_THEME_KEY);
    return raw === "vs-dark" || raw === "classic" ? raw : null;
  } catch {
    return null;
  }
}

export function storeEditorTheme(id: EditorThemeId): void {
  try {
    localStorage.setItem(EDITOR_THEME_KEY, id);
  } catch {
    // storage unavailable — setting just won't persist
  }
}

/** A full scheme every one of `editorColorKeys` is present and valid — anything less (a corrupt entry, an older/incompatible shape) is discarded rather than partially trusted. */
export function loadStoredEditorColors(): EditorColorScheme | null {
  try {
    const raw = localStorage.getItem(EDITOR_COLORS_KEY);
    if (!raw) return null;
    const parsed = JSON.parse(raw);
    if (editorColorKeys.every((key) => isHslColor(parsed?.[key]))) {
      return parsed;
    }
    return null;
  } catch {
    return null;
  }
}

export function storeEditorColors(colors: EditorColorScheme): void {
  try {
    localStorage.setItem(EDITOR_COLORS_KEY, JSON.stringify(colors));
  } catch {
    // storage unavailable — setting just won't persist
  }
}
