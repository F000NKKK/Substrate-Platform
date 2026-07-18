import type { HslColor } from "../color";

/** The theme's accent is just an HslColor — kept as an alias (not a redeclared shape) so theme and widgets never drift apart. */
export type AccentColor = HslColor;

/** A profile id — one of the built-in ids ("vs-dark", "classic") or a user-created/imported custom profile's generated id. Opaque beyond that, so custom ids can be anything unique. */
export type EditorThemeId = string;

/**
 * Every independently colorable part of the code editor — one shared value
 * per key across every open file/language (e.g. `background` is the same
 * for a `.rs` file as a `.json` file), but each key is its own setting, the
 * same way Visual Studio's "Fonts and Colors" page lets you recolor
 * "Keyword" independently of "Comment" independently of "Selected Text".
 */
export type EditorColorKey =
  | "background"
  | "foreground"
  | "selection"
  | "cursor"
  | "keyword"
  | "string"
  | "comment"
  | "number"
  | "type"
  | "function"
  | "variable"
  | "operator";

export type EditorColorScheme = Record<EditorColorKey, HslColor>;

/** One entry in the profile picker — built-in profiles ship as code constants and can be reset; custom ones are user-created/imported and only ever removed, never "reset" (there's no separate default to revert to). */
export interface EditorColorProfile {
  id: EditorThemeId;
  name: string;
  builtIn: boolean;
  colors: EditorColorScheme;
}

export interface ThemeContextValue {
  accent: AccentColor;
  setAccent: (accent: AccentColor) => void;
  presets: readonly AccentColor[];
  /** The profile currently applied — switching this loads that profile's colors wholesale. */
  editorTheme: EditorThemeId;
  setEditorTheme: (id: EditorThemeId) => void;
  /** Every profile available in the picker, built-in first, then custom ones in creation order. */
  editorProfiles: readonly EditorColorProfile[];
  /** The live, possibly individually-overridden color for every editor category, for whichever profile is currently active. */
  editorColors: EditorColorScheme;
  setEditorColor: (key: EditorColorKey, color: HslColor) => void;
  /** Reloads the *built-in* default colors for the active profile — a no-op (and hidden in the UI) for custom profiles, which have no separate "default" to revert to. */
  resetEditorColors: () => void;
  /** Saves the current live colors as a new custom profile and switches to it; returns the new profile's id. */
  createEditorProfile: (name: string) => EditorThemeId;
  /** Removes a custom profile (no-op on a built-in one); switches to "vs-dark" first if it was the active profile. */
  deleteEditorProfile: (id: EditorThemeId) => void;
  /** Serializes a profile (built-in or custom) to a shareable JSON string. */
  exportEditorProfile: (id: EditorThemeId) => string;
  /** Parses a previously-exported JSON string into a new custom profile (renamed to avoid colliding with an existing name) and switches to it; returns the new id, or `null` if the JSON was invalid. */
  importEditorProfile: (json: string) => EditorThemeId | null;
}
