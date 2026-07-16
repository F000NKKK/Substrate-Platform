import { createContext, useCallback, useMemo, useState, type ReactNode } from "react";
import { tokens } from "../tokens";
import type { AccentColor, ThemeContextValue } from "./types";
import { accentPresets, defaultAccent } from "./presets";
import { loadStoredAccent, storeAccent } from "./storage";

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

/** Owns the live accent color, mirrors it onto `--sp-accent-*`, and persists edits across sessions. */
export function ThemeProvider({ children, initialAccent }: ThemeProviderProps) {
  const [accent, setAccentState] = useState<AccentColor>(() => {
    const stored = loadStoredAccent();
    const initial = stored ?? initialAccent ?? defaultAccent;
    applyAccent(initial);
    return initial;
  });

  const setAccent = useCallback((next: AccentColor) => {
    applyAccent(next);
    storeAccent(next);
    setAccentState(next);
  }, []);

  const value = useMemo<ThemeContextValue>(
    () => ({ accent, setAccent, presets: accentPresets }),
    [accent, setAccent]
  );

  return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>;
}
