import { useEffect, useRef, useState, useSyncExternalStore } from "react";
import { Terminal as XTerm } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { IconButton } from "../IconButton";
import { Icon } from "../../infra/icons";
import { eventKey } from "../../infra/keys";
import { useT } from "../../infra/i18n";
import { ContextMenu, useContextMenu } from "../ContextMenu";
import { useStateStore } from "../../infra/state";
import "@xterm/xterm/css/xterm.css";
import "./TerminalPanel.css";

export interface TerminalShellKind {
  id: string;
  label: string;
  /** Passed as-is to the backend's `pty_spawn`; omit to use its own OS default shell. */
  shell?: string;
  /** What to type into this shell to clear its screen — defaults are inferred from `id` (`Clear-Host` for pwsh/powershell, `cls` for cmd, `clear` otherwise). Must be an actual shell command, not a client-side terminal clear: the shell's own line-editor tracks cursor position itself and only resyncs when it's the one that ran the clear. */
  clearCommand?: string;
}

export interface TerminalPanelProps {
  /** Names of the Tauri commands backing each terminal — defaults match substrate-platform's own pty module. `spawn`/`write`/`resize`/`kill` all take a string `id` addressing one session; `spawn` also gets `shell` and, for "Open in Terminal" (`openTerminalIn`), `cwd`. */
  commands?: { spawn: string; write: string; resize: string; kill: string };
  /** Tauri event carrying `{ id, data }` PTY output chunks. */
  outputEvent?: string;
  /** Tauri event carrying `{ id }` when a shell exits. */
  exitEvent?: string;
  /**
   * Explicit override for the shell kinds offered by "new terminal" — when
   * omitted (the common case), the panel asks the backend via
   * `listShellsCommand` (defaults to substrate-platform's own `list_shells`,
   * which detects what's actually installed on this machine rather than
   * assuming e.g. zsh/pwsh exist).
   */
  shells?: TerminalShellKind[];
  /** Tauri command returning `{ id, label, command, clearCommand }[]` for every shell actually available. Ignored if `shells` is given. */
  listShellsCommand?: string;
}

const DEFAULT_COMMANDS = { spawn: "pty_spawn", write: "pty_write", resize: "pty_resize", kill: "pty_kill" };
const FALLBACK_SHELL: TerminalShellKind = { id: "default", label: "Terminal" };

/**
 * Terminal ids are unique per real session (Date.now()-suffixed, never
 * reused), but React.StrictMode double-invokes effects in dev — mount,
 * cleanup, mount again — for the *same* id. Each invocation owns its own
 * `cleared`/`clearTimer` closure, so without this, both the orphaned first
 * mount's delayed clear and the second mount's legitimate one can end up
 * firing against whichever session is actually live, writing the clear
 * command twice. Tracking "already auto-cleared" here, keyed by id and
 * shared across every mount, makes that a no-op past the first one to land.
 */
const autoClearedIds = new Set<string>();

function defaultClearCommand(id: string): string {
  // `\r`, not `\n` — see shell.rs's `clear_command_for` for why (PSReadLine
  // only submits on `\r`, unlike bash's readline which accepts either).
  if (id === "pwsh" || id === "powershell") return "Clear-Host\r";
  if (id === "cmd") return "cls\r";
  return "clear\r";
}

interface TerminalTab {
  id: string;
  shellId: string;
  /** This shell kind's instance number (Bash, Bash 2, Bash 3, ...) — reused from whatever's currently free, not a monotonic counter, so closing "Bash 2" means the next new Bash becomes "Bash 2" again instead of "Bash 4". */
  n: number;
  /** Set by double-clicking the sidebar label to rename — overrides the generated "Bash 2"-style label until the tab closes. */
  customLabel?: string;
  /** Terminals sharing a groupId render side by side as split panes in the main view — a plain "new terminal" starts its own group (equal to its own id); "Split Terminal" joins the active tab's group instead. */
  groupId: string;
  /** The folder it was opened in ("Open in Terminal"); unset, the backend picks (the project root). Not persisted. */
  cwd?: string;
}

/** What's remembered of each terminal across restarts (under `TERMINAL_STATE_KEY` in the current `StateStore`) — its shell kind, label and split grouping, never its contents: a restored terminal is a fresh shell session. */
interface PersistedTerminals {
  tabs: { shellId: string; n: number; customLabel?: string; group: number }[];
  /** Index into `tabs` of the focused one. */
  active: number;
}

const TERMINAL_STATE_KEY = "terminal.tabs";

// "Open in Terminal" requests — module-level, so a file explorer can ask
// before the panel is even shown; the panel opens each once it's ready.
let pendingCwds: string[] = [];
const cwdSubscribers = new Set<() => void>();

/** Opens a new terminal (of the first shell kind) in `cwd` — the product shows the Terminal panel itself. */
export function openTerminalIn(cwd: string): void {
  pendingCwds = [...pendingCwds, cwd];
  cwdSubscribers.forEach((notify) => notify());
}

function usePendingCwds(): string[] {
  return useSyncExternalStore(
    (notify) => {
      cwdSubscribers.add(notify);
      return () => cwdSubscribers.delete(notify);
    },
    () => pendingCwds
  );
}

function isPersistedTerminals(value: unknown): value is PersistedTerminals {
  const v = value as PersistedTerminals;
  return (
    !!v &&
    Array.isArray(v.tabs) &&
    typeof v.active === "number" &&
    v.tabs.every((t) => t && typeof t.shellId === "string" && typeof t.n === "number" && typeof t.group === "number")
  );
}

function toPersisted(tabs: TerminalTab[], activeId: string | null): PersistedTerminals {
  const groups = [...new Set(tabs.map((t) => t.groupId))];
  return {
    tabs: tabs.map((t) => ({ shellId: t.shellId, n: t.n, customLabel: t.customLabel, group: groups.indexOf(t.groupId) })),
    active: Math.max(0, tabs.findIndex((t) => t.id === activeId)),
  };
}

function labelFor(kind: TerminalShellKind | undefined, tab: TerminalTab): string {
  if (tab.customLabel) return tab.customLabel;
  const base = kind?.label ?? tab.shellId;
  return tab.n > 1 ? `${base} ${tab.n}` : base;
}

/**
 * A VS-Code-style integrated terminal panel: a top toolbar (new terminal,
 * kill the active one, a "more actions" menu) and a right-side sidebar
 * listing every open PTY-backed shell session (click to switch, "x" to
 * close). Inactive terminals stay mounted (scrollback and their shell keep
 * running), so switching is instant. Product-agnostic — any
 * Substrate-Platform product gets it by dropping it into a PlatformShell tool
 * window and exposing the matching `pty_*`/`list_shells` Tauri commands.
 */
export function TerminalPanel({
  commands = DEFAULT_COMMANDS,
  outputEvent = "pty-output",
  exitEvent = "pty-exit",
  shells: shellsProp,
  listShellsCommand = "list_shells",
}: TerminalPanelProps) {
  const t = useT();
  const [detectedShells, setDetectedShells] = useState<TerminalShellKind[] | null>(shellsProp ?? null);
  const [tabs, setTabs] = useState<TerminalTab[]>([]);
  const [activeId, setActiveId] = useState<string | null>(null);
  const [renamingId, setRenamingId] = useState<string | null>(null);
  const clearHandlesRef = useRef<Record<string, () => void>>({});
  const addMenu = useContextMenu<void>();
  const moreMenu = useContextMenu<void>();
  const store = useStateStore();
  // The store scope whose saved terminals have been restored — until then,
  // the tabs on screen aren't that scope's and mustn't be saved over it.
  const [restoredScope, setRestoredScope] = useState<string | null>(null);

  const shells = shellsProp ?? detectedShells;
  const activeGroupId = tabs.find((t) => t.id === activeId)?.groupId;
  // Groups in first-appearance order, each group's members in insertion order
  // — keeps split siblings adjacent in the sidebar even if unrelated tabs
  // were created in between them.
  const groupIds = [...new Set(tabs.map((t) => t.groupId))];
  const orderedTabs = groupIds.flatMap((gid) => tabs.filter((t) => t.groupId === gid));

  useEffect(() => {
    if (shellsProp) return;
    invoke<{ id: string; label: string; command: string; clearCommand: string }[]>(listShellsCommand)
      .then((detected) =>
        setDetectedShells(
          detected.length > 0
            ? detected.map((s) => ({ id: s.id, label: s.label, shell: s.command, clearCommand: s.clearCommand }))
            : [FALLBACK_SHELL]
        )
      )
      .catch(() => setDetectedShells([FALLBACK_SHELL]));
  }, [shellsProp, listShellsCommand]);

  /** `groupId` omitted starts a new split group of its own; passed, the new terminal joins that group as an additional side-by-side pane. */
  function addTerminal(kind: TerminalShellKind, groupId?: string, cwd?: string) {
    setTabs((prev) => {
      const used = new Set(prev.filter((t) => t.shellId === kind.id).map((t) => t.n));
      let n = 1;
      while (used.has(n)) n++;
      const id = `term-${kind.id}-${n}-${Date.now()}`;
      setActiveId(id);
      return [...prev, { id, shellId: kind.id, n, groupId: groupId ?? id, cwd }];
    });
  }

  /** Adds a new pane to the active terminal's split group, using the same shell kind — VS Code's "Split Terminal". */
  function splitTerminal() {
    const activeTab = tabs.find((t) => t.id === activeId);
    if (!activeTab) return;
    const kind = shells?.find((s) => s.id === activeTab.shellId) ?? shells?.[0];
    if (kind) addTerminal(kind, activeTab.groupId);
  }

  function renameTerminal(id: string, label: string) {
    const trimmed = label.trim();
    setTabs((prev) => prev.map((t) => (t.id === id ? { ...t, customLabel: trimmed || undefined } : t)));
  }

  function closeTerminal(id: string) {
    setTabs((prev) => {
      const closing = prev.find((t) => t.id === id);
      const idx = prev.findIndex((t) => t.id === id);
      const next = prev.filter((t) => t.id !== id);
      setActiveId((cur) => {
        if (cur !== id) return cur;
        // Prefer another pane in the same split group, so closing one pane
        // keeps you looking at its siblings instead of jumping elsewhere.
        const sibling = closing && next.find((t) => t.groupId === closing.groupId);
        return sibling?.id ?? next[idx]?.id ?? next[idx - 1]?.id ?? next[0]?.id ?? null;
      });
      return next;
    });
  }

  // As soon as the shell list is known (and again whenever the state store
  // switches scope — another project opened), reopen the terminals saved
  // there, as fresh sessions; with none saved, keep what's open, or open a
  // first terminal of the first detected/configured shell kind.
  useEffect(() => {
    if (!shells) return;
    const saved = store.get(TERMINAL_STATE_KEY);
    const known = isPersistedTerminals(saved) ? saved.tabs.filter((t) => shells.some((s) => s.id === t.shellId)) : [];
    if (known.length > 0) {
      const stamp = Date.now();
      const ids = known.map((t, i) => `term-${t.shellId}-${t.n}-${stamp}-${i}`);
      const groupIds = known.map((t, i) => ids[known.findIndex((o) => o.group === t.group)] ?? ids[i]);
      setTabs(known.map((t, i) => ({ id: ids[i], shellId: t.shellId, n: t.n, customLabel: t.customLabel, groupId: groupIds[i] })));
      setActiveId(ids[Math.min((saved as PersistedTerminals).active, ids.length - 1)] ?? null);
    } else if (tabs.length === 0 && shells[0] && pendingCwds.length === 0) addTerminal(shells[0]);
    setRestoredScope(store.scope);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [shells, store]);

  const cwds = usePendingCwds();
  useEffect(() => {
    if (!shells?.[0] || restoredScope !== store.scope || cwds.length === 0) return;
    pendingCwds = [];
    for (const cwd of cwds) addTerminal(shells[0], undefined, cwd);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [cwds, shells, restoredScope, store]);

  useEffect(() => {
    if (restoredScope === store.scope) store.set(TERMINAL_STATE_KEY, toPersisted(tabs, activeId));
  }, [tabs, activeId, restoredScope, store]);

  return (
    <div className="sp-terminal-manager">
      <div className="sp-terminal-toolbar">
        <div className="sp-terminal-toolbar-anchor">
          <IconButton
            title={t("New Terminal")}
            disabled={!shells}
            onClick={() => (shells && shells.length > 1 ? addMenu.openAtAnchor() : shells?.[0] && addTerminal(shells[0]))}
          >
            <Icon name="plus" size={14} />
          </IconButton>
          <ContextMenu
            target={addMenu.target ? { mode: "anchor" } : null}
            items={(shells ?? []).map((kind) => ({ label: kind.label, onSelect: () => addTerminal(kind) }))}
            onClose={addMenu.close}
          />
        </div>
        <IconButton title={t("Split Terminal")} disabled={!activeId} onClick={splitTerminal}>
          <Icon name="split" size={14} />
        </IconButton>
        <IconButton title={t("Kill Terminal")} disabled={!activeId} onClick={() => activeId && closeTerminal(activeId)}>
          <Icon name="trash" size={14} />
        </IconButton>
        <div className="sp-terminal-toolbar-anchor">
          <IconButton title={t("More Actions")} onClick={() => moreMenu.openAtAnchor()}>
            <Icon name="more" size={14} />
          </IconButton>
          <ContextMenu
            target={moreMenu.target ? { mode: "anchor" } : null}
            items={[
              {
                label: t("Clear Terminal"),
                shortcut: "Ctrl+L",
                disabled: !activeId,
                onSelect: () => activeId && clearHandlesRef.current[activeId]?.(),
              },
            ]}
            onClose={moreMenu.close}
          />
        </div>
      </div>
      <div className="sp-terminal-body">
        <div className="sp-terminal-views">
          {groupIds.map((groupId) => (
            <div key={groupId} className="sp-terminal-group" data-active={groupId === activeGroupId || undefined}>
              {tabs
                .filter((t) => t.groupId === groupId)
                .map((t) => {
                  const kind = shells?.find((s) => s.id === t.shellId);
                  return (
                    <div
                      key={t.id}
                      className="sp-terminal-pane"
                      data-focused={t.id === activeId || undefined}
                      onClick={() => setActiveId(t.id)}
                    >
                      <TerminalInstance
                        id={t.id}
                        shell={kind?.shell}
                        cwd={t.cwd}
                        clearCommand={kind?.clearCommand ?? defaultClearCommand(t.shellId)}
                        active={groupId === activeGroupId}
                        focused={t.id === activeId}
                        commands={commands}
                        outputEvent={outputEvent}
                        exitEvent={exitEvent}
                        onExit={() => closeTerminal(t.id)}
                        onReady={(clear) => {
                          if (clear) clearHandlesRef.current[t.id] = clear;
                          else delete clearHandlesRef.current[t.id];
                        }}
                      />
                    </div>
                  );
                })}
            </div>
          ))}
        </div>
        <div className="sp-terminal-sidebar">
          {orderedTabs.map((tab, i) => (
            <div
              key={tab.id}
              className="sp-terminal-sidebar-item"
              data-active={tab.id === activeId || undefined}
              data-split={(i > 0 && orderedTabs[i - 1].groupId === tab.groupId) || undefined}
              onClick={() => setActiveId(tab.id)}
            >
              <Icon name="terminal" size={14} />
              {renamingId === tab.id ? (
                <input
                  className="sp-terminal-sidebar-item-rename"
                  autoFocus
                  defaultValue={labelFor(shells?.find((s) => s.id === tab.shellId), tab)}
                  onClick={(e) => e.stopPropagation()}
                  onBlur={(e) => {
                    renameTerminal(tab.id, e.currentTarget.value);
                    setRenamingId(null);
                  }}
                  onKeyDown={(e) => {
                    e.stopPropagation();
                    if (e.key === "Enter") e.currentTarget.blur();
                    else if (e.key === "Escape") setRenamingId(null);
                  }}
                />
              ) : (
                <span
                  className="sp-terminal-sidebar-item-label"
                  onDoubleClick={(e) => {
                    e.stopPropagation();
                    setRenamingId(tab.id);
                  }}
                >
                  {labelFor(shells?.find((s) => s.id === tab.shellId), tab)}
                </span>
              )}
              <IconButton
                size="sm"
                title={t("Close Terminal")}
                className="sp-terminal-sidebar-item-close"
                onClick={(e) => {
                  e.stopPropagation();
                  closeTerminal(tab.id);
                }}
              >
                <Icon name="close" size={14} />
              </IconButton>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

interface TerminalInstanceProps {
  id: string;
  shell?: string;
  cwd?: string;
  clearCommand: string;
  /** This pane's split group is the one currently shown (drives `fit()` — xterm can't measure while hidden). Several panes can be `active` at once (every pane in a shown group); at most one is `focused`. */
  active: boolean;
  /** This exact terminal is the one that should receive keyboard input right now — distinct from `active` since a split group can show several panes at once, only one of which is focused. */
  focused: boolean;
  commands: { spawn: string; write: string; resize: string; kill: string };
  outputEvent: string;
  exitEvent: string;
  onExit: () => void;
  /** Hands the parent a `clear()` for this instance on mount, and `null` on unmount — backs the toolbar's "Clear Terminal" action. */
  onReady: (clear: (() => void) | null) => void;
}

function TerminalInstance({ id, shell, cwd, clearCommand, active, focused, commands, outputEvent, exitEvent, onExit, onReady }: TerminalInstanceProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const fitRef = useRef<(() => void) | null>(null);
  const termRef = useRef<XTerm | null>(null);
  const onExitRef = useRef(onExit);
  onExitRef.current = onExit;
  const onReadyRef = useRef(onReady);
  onReadyRef.current = onReady;
  const menu = useContextMenu<void>();
  const t = useT();

  useEffect(() => {
    if (!containerRef.current) return;

    const term = new XTerm({
      fontFamily: "var(--sp-font-mono)",
      fontSize: 13,
      theme: { background: "transparent" },
      cursorBlink: true,
    });
    termRef.current = term;
    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(containerRef.current);

    // Ctrl+Shift+C/V for copy/paste — xterm's own defaults for these
    // (Ctrl+C/V) are reserved for SIGINT and shell-native paste, so the
    // clipboard shortcuts live one modifier over, same as VS Code's terminal.
    term.attachCustomKeyEventHandler((e) => {
      if (e.type !== "keydown" || !e.ctrlKey || !e.shiftKey) return true;
      const key = eventKey(e);
      if (key === "c") {
        const selection = term.getSelection();
        if (selection) navigator.clipboard.writeText(selection).catch(() => {});
        return false;
      }
      if (key === "v") {
        navigator.clipboard
          .readText()
          .then((text) => invoke(commands.write, { id, data: text }))
          .catch(() => {});
        return false;
      }
      return true;
    });
    const fit = () => {
      try {
        fitAddon.fit();
      } catch {
        // container not laid out yet (e.g. hidden) — ignore.
      }
    };
    fitRef.current = fit;
    fit();

    // A client-side-only clear (xterm's own `term.clear()`) desyncs from the
    // shell's own line-editor: readline/PSReadLine track the cursor position
    // themselves, and only resync that tracking when *they're* the one that
    // ran the clear. Clearing just the display leaves that tracking stale, so
    // the shell's next redraw computes cursor moves from the old position and
    // everything typed afterward renders rows-below/columns-right of where it
    // should. So "clear" here always means "make the shell run its own clear
    // command" — never touch the xterm buffer directly.
    const runClear = () => invoke(commands.write, { id, data: clearCommand }).catch(() => {});
    onReadyRef.current(runClear);

    // Firing that clear command immediately after spawn races the shell's own
    // startup (profile scripts, MOTD, etc.) — a slow-loading profile (pwsh
    // especially) prints its banner *after* the command already ran, leaving
    // it on screen. Waiting for a quiet beat in the output stream absorbs
    // however long that startup takes, regardless of shell kind or profile
    // speed, before the clear actually runs.
    const CLEAR_QUIET_MS = 200;
    let cleared = false;
    let clearTimer: number | undefined;
    function scheduleClear() {
      if (cleared || autoClearedIds.has(id)) return;
      window.clearTimeout(clearTimer);
      clearTimer = window.setTimeout(() => {
        cleared = true;
        // Re-check at fire time, not just at schedule time — a sibling mount
        // for this same id (StrictMode's double-invoke) could have won the
        // race and already fired in between.
        if (autoClearedIds.has(id)) return;
        autoClearedIds.add(id);
        runClear();
      }, CLEAR_QUIET_MS);
    }

    let disposed = false;
    const unlistenOutput = listen<{ id: string; data: string }>(outputEvent, (event) => {
      if (disposed || event.payload.id !== id) return;
      term.write(event.payload.data);
      scheduleClear();
    });
    const unlistenExit = listen<{ id: string }>(exitEvent, (event) => {
      if (!disposed && event.payload.id === id) onExitRef.current();
    });

    invoke(commands.spawn, { id, cols: term.cols, rows: term.rows, shell, cwd })
      .then(scheduleClear)
      .catch((err) => term.writeln(`\r\n[failed to start shell: ${err}]`));

    const onData = term.onData((data) => {
      // The user is already typing — running the auto-clear out from under
      // them now would erase what they can see, so cancel it for good (and
      // for any sibling mount of this same id still holding a pending timer).
      cleared = true;
      autoClearedIds.add(id);
      window.clearTimeout(clearTimer);
      invoke(commands.write, { id, data }).catch(() => {});
    });

    const resizeObserver = new ResizeObserver(() => {
      fit();
      invoke(commands.resize, { id, cols: term.cols, rows: term.rows }).catch(() => {});
    });
    resizeObserver.observe(containerRef.current);

    return () => {
      disposed = true;
      fitRef.current = null;
      termRef.current = null;
      onReadyRef.current(null);
      window.clearTimeout(clearTimer);
      resizeObserver.disconnect();
      onData.dispose();
      unlistenOutput.then((un) => un());
      unlistenExit.then((un) => un());
      invoke(commands.kill, { id }).catch(() => {});
      term.dispose();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [id]);

  // xterm can't measure while hidden — refit when this pane's group becomes shown.
  useEffect(() => {
    if (active) fitRef.current?.();
  }, [active]);

  // Switching which terminal/pane is focused (sidebar click, pane click, a
  // group becoming active) needs to move actual keyboard focus along with
  // it — otherwise the visible/active terminal changes but typing still
  // lands wherever focus previously was, looking like the switch "didn't
  // take".
  useEffect(() => {
    if (focused) termRef.current?.focus();
  }, [focused]);

  return (
    <div
      ref={containerRef}
      className="sp-terminal-view"
      // Ctrl+<letter> belongs to the shell (Ctrl+W, Ctrl+S, ...), not the IDE.
      data-sp-native-keys=""
      data-active={active || undefined}
      data-focused={focused || undefined}
      onContextMenu={(e) => menu.openAtPoint(undefined, e)}
    >
      <ContextMenu
        target={menu.target && menu.target.mode === "viewport" ? { mode: "viewport", x: menu.target.x, y: menu.target.y } : null}
        items={[
          {
            label: t("Copy"),
            shortcut: "Ctrl+Shift+C",
            disabled: !termRef.current?.hasSelection(),
            onSelect: () => {
              const selection = termRef.current?.getSelection();
              if (selection) navigator.clipboard.writeText(selection).catch(() => {});
            },
          },
          {
            label: t("Paste"),
            shortcut: "Ctrl+Shift+V",
            onSelect: () => {
              navigator.clipboard
                .readText()
                .then((text) => invoke(commands.write, { id, data: text }))
                .catch(() => {});
            },
          },
        ]}
        onClose={menu.close}
      />
    </div>
  );
}
