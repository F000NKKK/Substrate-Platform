import type { AccentColor } from "./types";

const STORAGE_KEY = "substrate-platform:accent";

export function loadStoredAccent(): AccentColor | null {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return null;
    const parsed = JSON.parse(raw);
    if (typeof parsed?.h === "number" && typeof parsed?.s === "number" && typeof parsed?.l === "number") {
      return parsed;
    }
    return null;
  } catch {
    return null;
  }
}

export function storeAccent(accent: AccentColor): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(accent));
  } catch {
    // storage unavailable (private mode, etc.) — theme just won't persist
  }
}
