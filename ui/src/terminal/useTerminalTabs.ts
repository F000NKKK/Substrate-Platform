import { useEffect, useState } from "react";
import { useStateStore } from "../infra/state";
import type { ShellInfo } from "./terminalApi";
import { STATE_KEY, isPersistedTerminals, restoredTabs, takePendingCwds, terminalId, toPersisted, usePendingCwds, type TerminalTab } from "./terminalTabs";

/**
 * The Terminal window's terminals: opening, splitting, renaming and closing
 * them, "Open in Terminal" requests, and remembering them per state scope
 * (the open project) — reopened as fresh shells of `shells`.
 */
export function useTerminalTabs(shells: ShellInfo[] | null) {
  const [tabs, setTabs] = useState<TerminalTab[]>([]);
  const [activeId, setActiveId] = useState<string | null>(null);
  const store = useStateStore();
  // The scope whose terminals were restored — till then the tabs on screen
  // aren't that scope's and mustn't be saved over it.
  const [restoredScope, setRestoredScope] = useState<string | null>(null);

  const cwds = usePendingCwds();

  /** `groupId` omitted starts a split group of its own; given, joins it. */
  function addTerminal(shell: ShellInfo, groupId?: string, cwd?: string) {
    setTabs((prev) => {
      const used = new Set(prev.filter((tab) => tab.shellId === shell.id).map((tab) => tab.n));
      let n = 1;
      while (used.has(n)) n++;
      const id = terminalId(shell.id, n);
      setActiveId(id);
      return [...prev, { id, shellId: shell.id, n, groupId: groupId ?? id, cwd }];
    });
  }

  /** Split Terminal: another of the active one's shell, beside it. */
  function splitTerminal() {
    const active = tabs.find((tab) => tab.id === activeId);
    const shell = shells?.find((s) => s.id === active?.shellId) ?? shells?.[0];
    if (active && shell) addTerminal(shell, active.groupId);
  }

  function renameTerminal(id: string, label: string) {
    const trimmed = label.trim();
    setTabs((prev) => prev.map((tab) => (tab.id === id ? { ...tab, customLabel: trimmed || undefined } : tab)));
  }

  function closeTerminal(id: string) {
    setTabs((prev) => {
      const closing = prev.find((tab) => tab.id === id);
      const idx = prev.findIndex((tab) => tab.id === id);
      const next = prev.filter((tab) => tab.id !== id);
      setActiveId((cur) => {
        if (cur !== id) return cur;
        // Its split siblings first, so closing a pane keeps the others in view.
        const sibling = closing && next.find((tab) => tab.groupId === closing.groupId);
        return sibling?.id ?? next[idx]?.id ?? next[idx - 1]?.id ?? next[0]?.id ?? null;
      });
      return next;
    });
  }

  // Once the shells are known, and whenever the store's scope changes (another
  // project): reopen the terminals saved there; with none, keep what's open
  // or start one of the first shell.
  useEffect(() => {
    if (!shells) return;
    const saved = store.get(STATE_KEY);
    const restored = isPersistedTerminals(saved) ? restoredTabs(saved, (id) => shells.some((s) => s.id === id)) : null;
    if (restored && restored.tabs.length > 0) {
      setTabs(restored.tabs);
      setActiveId(restored.active);
    } else if (tabs.length === 0 && shells[0] && cwds.length === 0) addTerminal(shells[0]);
    setRestoredScope(store.scope);
    // Only on new shells or a new scope: what's open then is what it keeps.
  }, [shells, store]);

  // "Open in Terminal" — once the scope's own terminals are back.
  useEffect(() => {
    if (!shells?.[0] || restoredScope !== store.scope || cwds.length === 0) return;
    for (const cwd of takePendingCwds()) addTerminal(shells[0], undefined, cwd);
  }, [cwds, shells, restoredScope, store]);

  useEffect(() => {
    if (restoredScope === store.scope) store.set(STATE_KEY, toPersisted(tabs, activeId));
  }, [tabs, activeId, restoredScope, store]);

  return { tabs, activeId, setActiveId, addTerminal, splitTerminal, renameTerminal, closeTerminal };
}
