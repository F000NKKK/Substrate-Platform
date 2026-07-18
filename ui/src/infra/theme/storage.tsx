import type { HslColor } from "../color";
import type { AccentColor, EditorThemeId } from "./types";

const ACCENT_KEY = "substrate-platform:accent";
const EDITOR_THEME_KEY = "substrate-platform:editor-theme";
const SELECTION_COLOR_KEY = "substrate-platform:selection-color";

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

export function loadStoredSelectionColor(): HslColor | null {
  try {
    const raw = localStorage.getItem(SELECTION_COLOR_KEY);
    if (!raw) return null;
    const parsed = JSON.parse(raw);
    return isHslColor(parsed) ? parsed : null;
  } catch {
    return null;
  }
}

export function storeSelectionColor(color: HslColor): void {
  try {
    localStorage.setItem(SELECTION_COLOR_KEY, JSON.stringify(color));
  } catch {
    // storage unavailable — setting just won't persist
  }
}
