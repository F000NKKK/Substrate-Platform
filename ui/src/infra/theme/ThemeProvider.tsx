import { createContext, useCallback, useMemo, useRef, useState, type ReactNode } from "react";
import type { HslColor } from "../color";
import { tokens } from "../tokens";
import type { AccentColor, EditorColorKey, EditorColorProfile, EditorColorScheme, EditorThemeId, ThemeContextValue } from "./types";
import { accentPresets, builtInProfiles, colorSchemeFor, defaultAccent, defaultEditorTheme, isBuiltInProfile } from "./presets";
import {
  isEditorColorScheme,
  loadCustomProfiles,
  loadStoredAccent,
  loadStoredEditorColors,
  loadStoredEditorTheme,
  storeAccent,
  storeCustomProfiles,
  storeEditorColors,
  storeEditorTheme,
  type StoredCustomProfile,
} from "./storage";

export const ThemeContext = createContext<ThemeContextValue | null>(null);

function applyAccent(accent: AccentColor): void {
  const root = document.documentElement.style;
  root.setProperty(tokens.accentH, String(accent.h));
  root.setProperty(tokens.accentS, `${accent.s}%`);
  root.setProperty(tokens.accentL, `${accent.l}%`);
}

function randomId(): string {
  return `custom-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
}

export interface ThemeProviderProps {
  children: ReactNode;
  /** Overrides the shipped default (studio purple) for products that want their own initial accent. */
  initialAccent?: AccentColor;
}

/**
 * Owns every live theming choice — the UI's accent color, and the code
 * editor's full per-category color scheme (background/foreground/selection/
 * cursor plus one entry per syntax token kind), across both built-in
 * profiles (VS Dark, Classic) and any number of user-created or imported
 * custom ones — mirrors the accent onto CSS custom properties, hands the
 * editor scheme to consumers directly, and persists edits across sessions.
 * The editor scheme and the UI accent are deliberately independent: a code
 * editor's palette and a brand accent serve different purposes, the same
 * way Visual Studio's own "Fonts and Colors" page has nothing to do with its
 * window-chrome theme.
 */
export function ThemeProvider({ children, initialAccent }: ThemeProviderProps) {
  const [accent, setAccentState] = useState<AccentColor>(() => {
    const stored = loadStoredAccent();
    const initial = stored ?? initialAccent ?? defaultAccent;
    applyAccent(initial);
    return initial;
  });

  const [customProfiles, setCustomProfilesState] = useState<StoredCustomProfile[]>(() => loadCustomProfiles());
  const customProfilesRef = useRef(customProfiles);
  customProfilesRef.current = customProfiles;

  const schemeForId = useCallback((id: EditorThemeId): EditorColorScheme => {
    return customProfilesRef.current.find((p) => p.id === id)?.colors ?? colorSchemeFor(id);
  }, []);

  const [editorTheme, setEditorThemeState] = useState<EditorThemeId>(() => loadStoredEditorTheme() ?? defaultEditorTheme);
  const [editorColors, setEditorColorsState] = useState<EditorColorScheme>(() => loadStoredEditorColors() ?? schemeForId(editorTheme));

  const setAccent = useCallback((next: AccentColor) => {
    applyAccent(next);
    storeAccent(next);
    setAccentState(next);
  }, []);

  const setEditorTheme = useCallback(
    (next: EditorThemeId) => {
      storeEditorTheme(next);
      setEditorThemeState(next);
      // Switching profiles loads that profile's colors wholesale — individual
      // keys can still be re-overridden afterward via setEditorColor.
      const scheme = schemeForId(next);
      storeEditorColors(scheme);
      setEditorColorsState(scheme);
    },
    [schemeForId]
  );

  const setEditorColor = useCallback((key: EditorColorKey, color: HslColor) => {
    setEditorColorsState((prev) => {
      const next = { ...prev, [key]: color };
      storeEditorColors(next);
      return next;
    });
  }, []);

  const resetEditorColors = useCallback(() => {
    // No-op for a custom profile — it has no separate "default" to revert
    // to (it *is* the default); the UI hides this action in that case too.
    if (!isBuiltInProfile(editorTheme)) return;
    const scheme = colorSchemeFor(editorTheme);
    storeEditorColors(scheme);
    setEditorColorsState(scheme);
  }, [editorTheme]);

  const createEditorProfile = useCallback(
    (name: string): EditorThemeId => {
      const id = randomId();
      const profile: StoredCustomProfile = { id, name, colors: editorColors };
      setCustomProfilesState((prev) => {
        const next = [...prev, profile];
        storeCustomProfiles(next);
        return next;
      });
      storeEditorTheme(id);
      setEditorThemeState(id);
      return id;
    },
    [editorColors]
  );

  const deleteEditorProfile = useCallback((id: EditorThemeId) => {
    if (isBuiltInProfile(id)) return;
    setCustomProfilesState((prev) => {
      const next = prev.filter((p) => p.id !== id);
      storeCustomProfiles(next);
      return next;
    });
    setEditorThemeState((current) => {
      if (current !== id) return current;
      const scheme = colorSchemeFor(defaultEditorTheme);
      storeEditorTheme(defaultEditorTheme);
      storeEditorColors(scheme);
      setEditorColorsState(scheme);
      return defaultEditorTheme;
    });
  }, []);

  const exportEditorProfile = useCallback(
    (id: EditorThemeId): string => {
      const builtIn = builtInProfiles.find((p) => p.id === id);
      const custom = customProfilesRef.current.find((p) => p.id === id);
      const name = builtIn?.name ?? custom?.name ?? id;
      const colors = builtIn?.colors ?? custom?.colors ?? colorSchemeFor(id);
      return JSON.stringify({ name, colors }, null, 2);
    },
    []
  );

  const importEditorProfile = useCallback((json: string): EditorThemeId | null => {
    try {
      const parsed: unknown = JSON.parse(json);
      const name = (parsed as { name?: unknown })?.name;
      const colors = (parsed as { colors?: unknown })?.colors;
      if (typeof name !== "string" || !isEditorColorScheme(colors)) return null;

      const id = randomId();
      const profile: StoredCustomProfile = { id, name, colors };
      setCustomProfilesState((prev) => {
        const next = [...prev, profile];
        storeCustomProfiles(next);
        return next;
      });
      storeEditorTheme(id);
      setEditorThemeState(id);
      storeEditorColors(colors);
      setEditorColorsState(colors);
      return id;
    } catch {
      return null;
    }
  }, []);

  const editorProfiles = useMemo<readonly EditorColorProfile[]>(
    () => [...builtInProfiles, ...customProfiles.map((p) => ({ id: p.id, name: p.name, builtIn: false, colors: p.colors }))],
    [customProfiles]
  );

  const value = useMemo<ThemeContextValue>(
    () => ({
      accent,
      setAccent,
      presets: accentPresets,
      editorTheme,
      setEditorTheme,
      editorProfiles,
      editorColors,
      setEditorColor,
      resetEditorColors,
      createEditorProfile,
      deleteEditorProfile,
      exportEditorProfile,
      importEditorProfile,
    }),
    [
      accent,
      setAccent,
      editorTheme,
      setEditorTheme,
      editorProfiles,
      editorColors,
      setEditorColor,
      resetEditorColors,
      createEditorProfile,
      deleteEditorProfile,
      exportEditorProfile,
      importEditorProfile,
    ]
  );

  return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>;
}
