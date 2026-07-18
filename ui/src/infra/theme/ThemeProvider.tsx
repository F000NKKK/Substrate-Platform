import { createContext, useCallback, useMemo, useState, type ReactNode } from "react";
import type { HslColor } from "../color";
import { tokens } from "../tokens";
import type { AccentColor, EditorColorKey, EditorColorScheme, EditorThemeId, ThemeContextValue } from "./types";
import { accentPresets, colorSchemeFor, defaultAccent, defaultEditorTheme, editorThemePresets } from "./presets";
import { loadStoredAccent, loadStoredEditorColors, loadStoredEditorTheme, storeAccent, storeEditorColors, storeEditorTheme } from "./storage";

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
 * Owns every live theming choice — the UI's accent color, and the code
 * editor's full per-category color scheme (background/foreground/selection/
 * cursor plus one entry per syntax token kind) — mirrors the accent onto CSS
 * custom properties, hands the editor scheme to consumers directly, and
 * persists edits across sessions. The editor scheme and the UI accent are
 * deliberately independent: a code editor's palette and a brand accent serve
 * different purposes, the same way Visual Studio's own "Fonts and Colors"
 * page has nothing to do with its window-chrome theme.
 */
export function ThemeProvider({ children, initialAccent }: ThemeProviderProps) {
  const [accent, setAccentState] = useState<AccentColor>(() => {
    const stored = loadStoredAccent();
    const initial = stored ?? initialAccent ?? defaultAccent;
    applyAccent(initial);
    return initial;
  });
  const [editorTheme, setEditorThemeState] = useState<EditorThemeId>(() => loadStoredEditorTheme() ?? defaultEditorTheme);
  const [editorColors, setEditorColorsState] = useState<EditorColorScheme>(
    () => loadStoredEditorColors() ?? colorSchemeFor(editorTheme)
  );

  const setAccent = useCallback((next: AccentColor) => {
    applyAccent(next);
    storeAccent(next);
    setAccentState(next);
  }, []);

  const setEditorTheme = useCallback((next: EditorThemeId) => {
    storeEditorTheme(next);
    setEditorThemeState(next);
    // Switching profiles resets every category to that profile's defaults —
    // individual keys can still be re-overridden afterward via setEditorColor.
    const scheme = colorSchemeFor(next);
    storeEditorColors(scheme);
    setEditorColorsState(scheme);
  }, []);

  const setEditorColor = useCallback((key: EditorColorKey, color: HslColor) => {
    setEditorColorsState((prev) => {
      const next = { ...prev, [key]: color };
      storeEditorColors(next);
      return next;
    });
  }, []);

  const resetEditorColors = useCallback(() => {
    const scheme = colorSchemeFor(editorTheme);
    storeEditorColors(scheme);
    setEditorColorsState(scheme);
  }, [editorTheme]);

  const value = useMemo<ThemeContextValue>(
    () => ({
      accent,
      setAccent,
      presets: accentPresets,
      editorTheme,
      setEditorTheme,
      editorThemePresets,
      editorColors,
      setEditorColor,
      resetEditorColors,
    }),
    [accent, setAccent, editorTheme, setEditorTheme, editorColors, setEditorColor, resetEditorColors]
  );

  return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>;
}
