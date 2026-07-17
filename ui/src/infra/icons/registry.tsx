import type { ComponentType } from "react";
import type { IconProps } from "./Icon";
import * as glyphs from "./glyphs";

export type IconComponent = ComponentType<IconProps>;

/** The platform's built-in glyph set, keyed by semantic name rather than component identity — this is what `<Icon name="..." />` resolves against by default. */
const defaultIcons: Record<string, IconComponent> = {
  window: glyphs.IconWindow,
  grid: glyphs.IconGrid,
  folder: glyphs.IconFolder,
  folderOpen: glyphs.IconFolderOpen,
  file: glyphs.IconFile,
  chevronRight: glyphs.IconChevronRight,
  sliders: glyphs.IconSliders,
  list: glyphs.IconList,
  terminal: glyphs.IconTerminal,
  float: glyphs.IconFloat,
  settings: glyphs.IconSettings,
  pin: glyphs.IconPin,
  pinOff: glyphs.IconPinOff,
  close: glyphs.IconClose,
  palette: glyphs.IconPalette,
  minimize: glyphs.IconMinimize,
  maximize: glyphs.IconMaximize,
  restore: glyphs.IconRestore,
  plus: glyphs.IconPlus,
};

const globalOverrides: Record<string, IconComponent> = {};

/**
 * Registers or replaces one icon in the platform-wide registry — the way a
 * product swaps a built-in glyph, or adds its own domain-specific one, for
 * every `<Icon name="...">` in the app without forking this package's source.
 * For overrides scoped to one subtree instead of the whole app, use
 * `IconProvider` rather than this global registration.
 */
export function registerIcon(name: string, component: IconComponent): void {
  globalOverrides[name] = component;
}

/** Bulk form of `registerIcon`. */
export function registerIcons(icons: Record<string, IconComponent>): void {
  Object.assign(globalOverrides, icons);
}

/** Resolves a name through global overrides, then the built-in set. Returns undefined for an unknown name — `IconProvider`'s `<Icon>` also checks its own scoped overrides first. */
export function getRegisteredIcon(name: string): IconComponent | undefined {
  return globalOverrides[name] ?? defaultIcons[name];
}

/** The built-in names available before any overrides — useful for a product that wants to enumerate or validate against the default set. */
export function getDefaultIconNames(): string[] {
  return Object.keys(defaultIcons);
}
