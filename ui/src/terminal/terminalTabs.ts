import { useSyncExternalStore } from "react";
import type { ShellInfo } from "./terminalApi";

/** One terminal in the panel. */
export interface TerminalTab {
  id: string;
  shellId: string;
  /** This shell's instance number (Bash, Bash 2…) — the lowest free one, so closing "Bash 2" makes the next Bash "Bash 2" again. */
  n: number;
  /** A name the user gave it (double-click the sidebar label), until it closes. */
  customLabel?: string;
  /** Terminals sharing a group show side by side; a new terminal starts its own (its id), Split Terminal joins the active one's. */
  groupId: string;
  /** The folder it was opened in ("Open in Terminal"); unset, the host picks (the project). Not remembered. */
  cwd?: string;
}

/** What's remembered of the terminals across restarts (under `STATE_KEY`): shells, names, splits — never contents. */
export interface PersistedTerminals {
  tabs: { shellId: string; n: number; customLabel?: string; group: number }[];
  /** Index into `tabs` of the focused one. */
  active: number;
}

export const STATE_KEY = "terminal.tabs";

/** A new terminal's id: unique per session, never reused. */
export function terminalId(shellId: string, n: number, salt = ""): string {
  return `term-${shellId}-${n}-${Date.now()}${salt}`;
}

export function isPersistedTerminals(value: unknown): value is PersistedTerminals {
  const v = value as PersistedTerminals;
  return (
    !!v &&
    Array.isArray(v.tabs) &&
    typeof v.active === "number" &&
    v.tabs.every((t) => t && typeof t.shellId === "string" && typeof t.n === "number" && typeof t.group === "number")
  );
}

export function toPersisted(tabs: TerminalTab[], activeId: string | null): PersistedTerminals {
  const groups = [...new Set(tabs.map((t) => t.groupId))];
  return {
    tabs: tabs.map((t) => ({ shellId: t.shellId, n: t.n, customLabel: t.customLabel, group: groups.indexOf(t.groupId) })),
    active: Math.max(0, tabs.findIndex((t) => t.id === activeId)),
  };
}

/** The remembered terminals as fresh tabs, split as they were; `known`: shells installed now. */
export function restoredTabs(saved: PersistedTerminals, known: (shellId: string) => boolean): { tabs: TerminalTab[]; active: string | null } {
  const kept = saved.tabs.filter((t) => known(t.shellId));
  const ids = kept.map((t, i) => terminalId(t.shellId, t.n, `-${i}`));
  const groupIds = kept.map((t, i) => ids[kept.findIndex((o) => o.group === t.group)] ?? ids[i]);
  return {
    tabs: kept.map((t, i) => ({ id: ids[i], shellId: t.shellId, n: t.n, customLabel: t.customLabel, groupId: groupIds[i] })),
    active: ids[Math.min(saved.active, ids.length - 1)] ?? null,
  };
}

/** The tab's name: the user's, else the shell's label (translated by `t`) and its number past the first. */
export function labelFor(shell: ShellInfo | undefined, tab: TerminalTab, t: (text: string) => string): string {
  if (tab.customLabel) return tab.customLabel;
  const base = shell ? t(shell.label) : tab.shellId;
  return tab.n > 1 ? `${base} ${tab.n}` : base;
}

// "Open in Terminal" requests — module-level, so a file explorer can ask
// before the panel is even shown; the panel opens each once it's ready.
let pendingCwds: string[] = [];
const cwdSubscribers = new Set<() => void>();

/** Opens a new terminal (of the first shell) in `cwd` — the product shows the Terminal window itself. */
export function openTerminalIn(cwd: string): void {
  pendingCwds = [...pendingCwds, cwd];
  cwdSubscribers.forEach((notify) => notify());
}

/** Takes the waiting "Open in Terminal" folders. */
export function takePendingCwds(): string[] {
  const taken = pendingCwds;
  pendingCwds = [];
  return taken;
}

export function usePendingCwds(): string[] {
  return useSyncExternalStore(
    (notify) => {
      cwdSubscribers.add(notify);
      return () => cwdSubscribers.delete(notify);
    },
    () => pendingCwds,
  );
}
