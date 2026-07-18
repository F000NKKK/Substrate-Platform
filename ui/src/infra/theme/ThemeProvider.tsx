import { createContext, useCallback, useMemo, useState, type ReactNode } from "react";
import type { HslColor } from "../color";
import { tokens } from "../tokens";
import type { AccentColor, EditorThemeId, ThemeContextValue } from "./types";
import { accentPresets, defaultAccent, defaultEditorTheme, defaultSelectionColor, editorThemePresets, selectionColorPresets } from "./presets";
import {
  loadStoredAccent,
  loadStoredEditorTheme,
  loadStoredSelectionColor,
  storeAccent,
  storeEditorTheme,
  storeSelectionColor,
} from "./storage";

export const ThemeContext = createContext<ThemeContextValue | null>(null);

function applyAccent(accent: AccentColor): void {
  const root = document.documentElement.style;
  root.setProperty(tokens.accentH, String(accent.h));
  root.setProperty(tokens.accentS, `${accent.s}%`);
  root.setProperty(tokens.accentL, `${accent.l}%`);
}

export interface ThemeProviderProps {
  children: ReactNode;
  /** Overrides the shipped default (studio purple) for products that want their own initial accent. */
  initialAccent?: AccentColor;
}

/**
 * Owns every live theming choice — the UI's accent color, the code editor's
 * syntax-highlighting profile, and its selection-highlight color (kept
 * separate from `accent` on purpose: a code editor's palette and a brand
 * accent serve different purposes) — mirrors each onto CSS custom
 * properties or hands it to consumers directly, and persists edits across
 * sessions.
 */
export function ThemeProvider({ children, initialAccent }: ThemeProviderProps) {
  const [accent, setAccentState] = useState<AccentColor>(() => {
    const stored = loadStoredAccent();
    const initial = stored ?? initialAccent ?? defaultAccent;
    applyAccent(initial);
    return initial;
  });
  const [editorTheme, setEditorThemeState] = useState<EditorThemeId>(() => loadStoredEditorTheme() ?? defaultEditorTheme);
  const [selectionColor, setSelectionColorState] = useState<HslColor>(() => loadStoredSelectionColor() ?? defaultSelectionColor);

  const setAccent = useCallback((next: AccentColor) => {
    applyAccent(next);
    storeAccent(next);
    setAccentState(next);
  }, []);

  const setEditorTheme = useCallback((next: EditorThemeId) => {
    storeEditorTheme(next);
    setEditorThemeState(next);
  }, []);

  const setSelectionColor = useCallback((next: HslColor) => {
    storeSelectionColor(next);
    setSelectionColorState(next);
  }, []);

  const value = useMemo<ThemeContextValue>(
    () => ({
      accent,
      setAccent,
      presets: accentPresets,
      editorTheme,
      setEditorTheme,
      editorThemePresets,
      selectionColor,
      setSelectionColor,
      selectionColorPresets,
    }),
    [accent, setAccent, editorTheme, setEditorTheme, selectionColor, setSelectionColor]
  );

  return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>;
}
