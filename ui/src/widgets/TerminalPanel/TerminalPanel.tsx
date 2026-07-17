import { useEffect, useRef, useState } from "react";
import { Terminal as XTerm } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { IconButton } from "../IconButton";
import { Icon } from "../../infra/icons";
import { ContextMenu, useContextMenu } from "../ContextMenu";
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
  /** Names of the Tauri commands backing each terminal — defaults match substrate-platform's own pty module. `spawn`/`write`/`resize`/`kill` all take a string `id` addressing one session. */
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
  const [detectedShells, setDetectedShells] = useState<TerminalShellKind[] | null>(shellsProp ?? null);
  const [tabs, setTabs] = useState<TerminalTab[]>([]);
  const [activeId, setActiveId] = useState<string | null>(null);
  const [renamingId, setRenamingId] = useState<string | null>(null);
  const clearHandlesRef = useRef<Record<string, () => void>>({});
  const addMenu = useContextMenu<void>();
  const moreMenu = useContextMenu<void>();

  const shells = shellsProp ?? detectedShells;

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
  function addTerminal(kind: TerminalShellKind, groupId?: string) {
    setTabs((prev) => {
      const used = new Set(prev.filter((t) => t.shellId === kind.id).map((t) => t.n));
      let n = 1;
      while (used.has(n)) n++;
      const id = `term-${kind.id}-${n}-${Date.now()}`;
      setActiveId(id);
      return [...prev, { id, shellId: kind.id, n, groupId: groupId ?? id }];
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

  // Open a first terminal (the first detected/configured shell kind) as soon
  // as the shell list is known.
  useEffect(() => {
    if (shells?.[0] && tabs.length === 0) addTerminal(shells[0]);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [shells]);

  return (
    <div className="sp-terminal-manager">
      <div className="sp-terminal-toolbar">
        <div className="sp-terminal-toolbar-anchor">
          <IconButton
            size={22}
            title="New terminal"
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
        <IconButton size={22} title="Split terminal" disabled={!activeId} onClick={splitTerminal}>
          <Icon name="split" size={14} />
        </IconButton>
        <IconButton size={22} title="Kill terminal" disabled={!activeId} onClick={() => activeId && closeTerminal(activeId)}>
          <Icon name="trash" size={14} />
        </IconButton>
        <div className="sp-terminal-toolbar-anchor">
          <IconButton size={22} title="More actions" onClick={() => moreMenu.openAtAnchor()}>
            <Icon name="more" size={14} />
          </IconButton>
          <ContextMenu
            target={moreMenu.target ? { mode: "anchor" } : null}
            items={[
              {
                label: "Clear Terminal",
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
          {tabs.map((t) => {
            const kind = shells?.find((s) => s.id === t.shellId);
            return (
              <TerminalInstance
                key={t.id}
                id={t.id}
                shell={kind?.shell}
                clearCommand={kind?.clearCommand ?? defaultClearCommand(t.shellId)}
                active={t.id === activeId}
                commands={commands}
                outputEvent={outputEvent}
                exitEvent={exitEvent}
                onExit={() => closeTerminal(t.id)}
                onReady={(clear) => {
                  if (clear) clearHandlesRef.current[t.id] = clear;
                  else delete clearHandlesRef.current[t.id];
                }}
              />
            );
          })}
        </div>
        <div className="sp-terminal-sidebar">
          {tabs.map((t) => (
            <div
              key={t.id}
              className="sp-terminal-sidebar-item"
              data-active={t.id === activeId || undefined}
              onClick={() => setActiveId(t.id)}
            >
              <Icon name="terminal" size={14} />
              {renamingId === t.id ? (
                <input
                  className="sp-terminal-sidebar-item-rename"
                  autoFocus
                  defaultValue={labelFor(shells?.find((s) => s.id === t.shellId), t)}
                  onClick={(e) => e.stopPropagation()}
                  onBlur={(e) => {
                    renameTerminal(t.id, e.currentTarget.value);
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
                    setRenamingId(t.id);
                  }}
                >
                  {labelFor(shells?.find((s) => s.id === t.shellId), t)}
                </span>
              )}
              <IconButton
                size={22}
                title="Close terminal"
                className="sp-terminal-sidebar-item-close"
                onClick={(e) => {
                  e.stopPropagation();
                  closeTerminal(t.id);
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
  clearCommand: string;
  active: boolean;
  commands: { spawn: string; write: string; resize: string; kill: string };
  outputEvent: string;
  exitEvent: string;
  onExit: () => void;
  /** Hands the parent a `clear()` for this instance on mount, and `null` on unmount — backs the toolbar's "Clear Terminal" action. */
  onReady: (clear: (() => void) | null) => void;
}

function TerminalInstance({ id, shell, clearCommand, active, commands, outputEvent, exitEvent, onExit, onReady }: TerminalInstanceProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const fitRef = useRef<(() => void) | null>(null);
  const termRef = useRef<XTerm | null>(null);
  const onExitRef = useRef(onExit);
  onExitRef.current = onExit;
  const onReadyRef = useRef(onReady);
  onReadyRef.current = onReady;
  const menu = useContextMenu<void>();

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
      const key = e.key.toLowerCase();
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

    invoke(commands.spawn, { id, cols: term.cols, rows: term.rows, shell })
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

  // xterm can't measure while hidden — refit when this tab becomes active.
  useEffect(() => {
    if (active) fitRef.current?.();
  }, [active]);

  return (
    <div
      ref={containerRef}
      className="sp-terminal-view"
      data-active={active || undefined}
      onContextMenu={(e) => menu.openAtPoint(undefined, e)}
    >
      <ContextMenu
        target={menu.target && menu.target.mode === "viewport" ? { mode: "viewport", x: menu.target.x, y: menu.target.y } : null}
        items={[
          {
            label: "Copy",
            shortcut: "Ctrl+Shift+C",
            disabled: !termRef.current?.hasSelection(),
            onSelect: () => {
              const selection = termRef.current?.getSelection();
              if (selection) navigator.clipboard.writeText(selection).catch(() => {});
            },
          },
          {
            label: "Paste",
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
