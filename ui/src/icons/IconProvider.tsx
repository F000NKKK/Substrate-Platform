import { createContext, useContext, useMemo, type ReactNode } from "react";
import type { IconComponent } from "./registry";
import { getRegisteredIcon } from "./registry";
import type { IconProps } from "./Icon";

const IconOverridesContext = createContext<Record<string, IconComponent>>({});

export interface IconProviderProps {
  /** Icon overrides scoped to this subtree — merged over anything `registerIcon`-ed globally, which is in turn merged over the built-in set. Nest providers to layer overrides further down the tree. */
  icons?: Record<string, IconComponent>;
  children: ReactNode;
}

/** Scopes icon overrides to part of the tree — e.g. one screen wants its own "folder" glyph without changing it for the rest of the app. Most products just call `registerIcon` once at startup instead of reaching for this. */
export function IconProvider({ icons, children }: IconProviderProps) {
  const parentOverrides = useContext(IconOverridesContext);
  const merged = useMemo(() => ({ ...parentOverrides, ...icons }), [parentOverrides, icons]);
  return <IconOverridesContext.Provider value={merged}>{children}</IconOverridesContext.Provider>;
}

export interface IconElementProps extends IconProps {
  name: string;
}

/**
 * Renders a registered icon by semantic name — resolves the nearest
 * `IconProvider`'s scoped overrides, then anything globally `registerIcon`-ed,
 * then the platform's built-in set. Renders nothing for an unknown name
 * (and warns once in development) rather than throwing, since a missing
 * icon shouldn't take down the surrounding UI.
 */
export function Icon({ name, ...props }: IconElementProps) {
  const overrides = useContext(IconOverridesContext);
  const Component = overrides[name] ?? getRegisteredIcon(name);
  if (!Component) {
    console.warn(`[substrate-platform-ui] Unknown icon "${name}" — check registerIcon/IconProvider or the built-in name list.`);
    return null;
  }
  return <Component {...props} />;
}
