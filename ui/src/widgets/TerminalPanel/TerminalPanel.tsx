import { useEffect, useRef, useState } from "react";
import { Terminal as XTerm } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { IconButton } from "../IconButton";
import { Icon } from "../../infra/icons";
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
  /** The shell kinds offered as individual "new terminal" buttons — VS-Code style (bash, pwsh, etc). Defaults cover the common cross-platform shells. */
  shells?: TerminalShellKind[];
}

const DEFAULT_COMMANDS = { spawn: "pty_spawn", write: "pty_write", resize: "pty_resize", kill: "pty_kill" };

const DEFAULT_SHELLS: TerminalShellKind[] = [
  { id: "bash", label: "Bash", shell: "bash" },
  { id: "zsh", label: "Zsh", shell: "zsh" },
  { id: "pwsh", label: "PowerShell", shell: "pwsh" },
];

interface TerminalTab {
  id: string;
  label: string;
  shellId: string;
}

/**
 * A VS-Code-style integrated terminal panel: a right-side sidebar listing
 * every open PTY-backed shell session (click to switch, "x" to close), with
 * one button per configured shell kind at the top of the sidebar to spawn a
 * new session of that kind. Inactive terminals stay mounted (scrollback and
 * their shell keep running), so switching is instant. Product-agnostic — any
 * Substrate-Platform product gets it by dropping it into a PlatformShell tool
 * window and exposing the matching `pty_*` Tauri commands.
 */
export function TerminalPanel({
  commands = DEFAULT_COMMANDS,
  outputEvent = "pty-output",
  exitEvent = "pty-exit",
  shells = DEFAULT_SHELLS,
}: TerminalPanelProps) {
  const [tabs, setTabs] = useState<TerminalTab[]>([]);
  const [activeId, setActiveId] = useState<string | null>(null);
  const countersRef = useRef<Record<string, number>>({});

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

  // Open a first terminal (the first configured shell kind) on mount.
  useEffect(() => {
    if (shells[0]) addTerminal(shells[0]);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className="sp-terminal-manager">
      <div className="sp-terminal-views">
        {tabs.map((t) => {
          const kind = shells.find((s) => s.id === t.shellId);
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
            />
          );
        })}
      </div>
      <div className="sp-terminal-sidebar">
        <div className="sp-terminal-sidebar-add">
          {shells.map((kind) => (
            <IconButton key={kind.id} size={22} title={`New ${kind.label} terminal`} onClick={() => addTerminal(kind)}>
              <Icon name="plus" size={14} />
            </IconButton>
          ))}
        </div>
        <div className="sp-terminal-sidebar-list">
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
                size={16}
                title="Close terminal"
                className="sp-terminal-sidebar-item-close"
                onClick={(e) => {
                  e.stopPropagation();
                  closeTerminal(t.id);
                }}
              >
                <Icon name="close" size={12} />
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
}

function TerminalInstance({ id, shell, active, commands, outputEvent, exitEvent, onExit }: TerminalInstanceProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const fitRef = useRef<(() => void) | null>(null);
  const onExitRef = useRef(onExit);
  onExitRef.current = onExit;

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

    let disposed = false;
    const unlistenOutput = listen<{ id: string; data: string }>(outputEvent, (event) => {
      if (!disposed && event.payload.id === id) term.write(event.payload.data);
    });
    const unlistenExit = listen<{ id: string }>(exitEvent, (event) => {
      if (!disposed && event.payload.id === id) onExitRef.current();
    });

    invoke(commands.spawn, { id, cols: term.cols, rows: term.rows, shell })
      .then(() => {
        // Start from a clean screen, VS-Code style.
        invoke(commands.write, { id, data: "clear\n" }).catch(() => {});
      })
      .catch((err) => term.writeln(`\r\n[failed to start shell: ${err}]`));

    const onData = term.onData((data) => {
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
