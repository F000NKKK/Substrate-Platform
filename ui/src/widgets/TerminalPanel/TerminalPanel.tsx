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
  /** Tauri command returning `{ id, label, command }[]` for every shell actually available. Ignored if `shells` is given. */
  listShellsCommand?: string;
}

const DEFAULT_COMMANDS = { spawn: "pty_spawn", write: "pty_write", resize: "pty_resize", kill: "pty_kill" };
const FALLBACK_SHELL: TerminalShellKind = { id: "default", label: "Terminal" };

interface TerminalTab {
  id: string;
  label: string;
  shellId: string;
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
  const countersRef = useRef<Record<string, number>>({});
  const clearHandlesRef = useRef<Record<string, () => void>>({});
  const addMenu = useContextMenu<void>();
  const moreMenu = useContextMenu<void>();

  const shells = shellsProp ?? detectedShells;

  useEffect(() => {
    if (shellsProp) return;
    invoke<{ id: string; label: string; command: string }[]>(listShellsCommand)
      .then((detected) =>
        setDetectedShells(detected.length > 0 ? detected.map((s) => ({ id: s.id, label: s.label, shell: s.command })) : [FALLBACK_SHELL])
      )
      .catch(() => setDetectedShells([FALLBACK_SHELL]));
  }, [shellsProp, listShellsCommand]);

  function addTerminal(kind: TerminalShellKind) {
    const n = (countersRef.current[kind.id] ?? 0) + 1;
    countersRef.current[kind.id] = n;
    const id = `term-${kind.id}-${n}-${Date.now()}`;
    const label = n > 1 ? `${kind.label} ${n}` : kind.label;
    setTabs((prev) => [...prev, { id, label, shellId: kind.id }]);
    setActiveId(id);
  }

  function closeTerminal(id: string) {
    setTabs((prev) => {
      const idx = prev.findIndex((t) => t.id === id);
      const next = prev.filter((t) => t.id !== id);
      setActiveId((cur) => (cur !== id ? cur : next[idx]?.id ?? next[idx - 1]?.id ?? next[0]?.id ?? null));
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
              <span className="sp-terminal-sidebar-item-label">{t.label}</span>
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
  active: boolean;
  commands: { spawn: string; write: string; resize: string; kill: string };
  outputEvent: string;
  exitEvent: string;
  onExit: () => void;
  /** Hands the parent a `clear()` for this instance on mount, and `null` on unmount — backs the toolbar's "Clear Terminal" action. */
  onReady: (clear: (() => void) | null) => void;
}

function TerminalInstance({ id, shell, active, commands, outputEvent, exitEvent, onExit, onReady }: TerminalInstanceProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const fitRef = useRef<(() => void) | null>(null);
  const onExitRef = useRef(onExit);
  onExitRef.current = onExit;
  const onReadyRef = useRef(onReady);
  onReadyRef.current = onReady;

  useEffect(() => {
    if (!containerRef.current) return;

    const term = new XTerm({
      fontFamily: "var(--sp-font-mono)",
      fontSize: 13,
      theme: { background: "transparent" },
      cursorBlink: true,
    });
    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(containerRef.current);
    const fit = () => {
      try {
        fitAddon.fit();
      } catch {
        // container not laid out yet (e.g. hidden) — ignore.
      }
    };
    fitRef.current = fit;
    fit();
    onReadyRef.current(() => term.clear());

    // Clearing by typing a shell command (`clear`/`Clear-Host`/`cls`) races
    // that shell's own startup (profile scripts, MOTD, etc.) — a slow-loading
    // profile (pwsh especially) prints its banner *after* the command already
    // ran, leaving it on screen. Instead, clear the xterm buffer itself (no
    // shell involvement, so it's identical across every shell) once output
    // has gone quiet for a beat — that beat absorbs however long the shell's
    // startup output takes, regardless of shell kind or profile speed.
    const CLEAR_QUIET_MS = 200;
    let cleared = false;
    let clearTimer: number | undefined;
    function scheduleClear() {
      if (cleared) return;
      window.clearTimeout(clearTimer);
      clearTimer = window.setTimeout(() => {
        cleared = true;
        term.clear();
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
      // The user is already typing — clearing out from under them now would
      // erase what they can see, so cancel the pending auto-clear for good.
      cleared = true;
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

  return <div ref={containerRef} className="sp-terminal-view" data-active={active || undefined} />;
}
