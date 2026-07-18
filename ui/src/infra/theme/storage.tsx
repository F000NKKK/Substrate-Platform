import type { HslColor } from "../color";
import type { AccentColor, EditorColorScheme, EditorThemeId } from "./types";
import { editorColorKeys } from "./presets";

const ACCENT_KEY = "substrate-platform:accent";
const EDITOR_THEME_KEY = "substrate-platform:editor-theme";
const EDITOR_COLORS_KEY = "substrate-platform:editor-colors";
const CUSTOM_PROFILES_KEY = "substrate-platform:editor-custom-profiles";

function isHslColor(value: unknown): value is HslColor {
  const v = value as Record<string, unknown> | null;
  return typeof v?.h === "number" && typeof v?.s === "number" && typeof v?.l === "number";
}

export function isEditorColorScheme(value: unknown): value is EditorColorScheme {
  const v = value as Record<string, unknown> | null;
  return !!v && editorColorKeys.every((key) => isHslColor(v[key]));
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
    return localStorage.getItem(EDITOR_THEME_KEY);
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
    return isEditorColorScheme(parsed) ? parsed : null;
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

export interface StoredCustomProfile {
  id: string;
  name: string;
  colors: EditorColorScheme;
}

/** Every user-created/imported profile, in creation order — anything malformed is dropped rather than discarding the whole list. */
export function loadCustomProfiles(): StoredCustomProfile[] {
  try {
    const raw = localStorage.getItem(CUSTOM_PROFILES_KEY);
    if (!raw) return [];
    const parsed = JSON.parse(raw);
    if (!Array.isArray(parsed)) return [];
    return parsed.filter(
      (p): p is StoredCustomProfile => typeof p?.id === "string" && typeof p?.name === "string" && isEditorColorScheme(p?.colors)
    );
  } catch {
    return [];
  }
}

export function storeCustomProfiles(profiles: StoredCustomProfile[]): void {
  try {
    localStorage.setItem(CUSTOM_PROFILES_KEY, JSON.stringify(profiles));
  } catch {
    // storage unavailable — custom profiles just won't persist
  }
}
