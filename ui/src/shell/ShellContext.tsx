import { createContext, useContext } from "react";

export interface ShellToolWindow {
  id: string;
  title: string;
}

/**
 * What a product can do to the running `PlatformShell` from inside it — its
 * menu, toolbar, or any panel (e.g. a View menu toggling tool windows, or a
 * file tab bringing itself to the front). Read it with `useShell()`.
 */
export interface ShellController {
  /** Every tool window the product declared, in declaration order (left, right, then bottom). */
  toolWindows: readonly ShellToolWindow[];
  isToolWindowVisible: (id: string) => boolean;
  /** Pins a hidden tool window open on its edge, activates its tab if it's docked in the center, or reopens a closed transient one where it last was. */
  showToolWindow: (id: string) => void;
  /** Collapses a tool window back to its edge's strip — or, for a transient one (`PanelDef.floatSize`), closes it. */
  hideToolWindow: (id: string) => void;
  /** Takes a tool window off screen entirely, strip tab included, like its × — `showToolWindow` (or View) brings it back. */
  closeToolWindow: (id: string) => void;
  /** Whether the tool window is currently floating free of every dock. */
  isToolWindowFloating: (id: string) => boolean;
  /** The center dock's active tab — `main`'s id when nothing else is. */
  activeCenterId: string;
  /** Makes a center panel (e.g. an open file's tab) the active one. */
  activateCenterPanel: (id: string) => void;
}

export const ShellContext = createContext<ShellController | null>(null);

/** The enclosing `PlatformShell`'s controller — only available to components rendered inside it (its `menu`, `statusBar`, and every panel). */
export function useShell(): ShellController {
  const ctx = useContext(ShellContext);
  if (!ctx) throw new Error("useShell() must be used inside a <PlatformShell>");
  return ctx;
}
