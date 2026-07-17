import { useEffect, useRef, useState } from "react";
import { Terminal as XTerm } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Tab } from "../Tab";
import { IconButton } from "../IconButton";
import { Icon } from "../../infra/icons";
import "@xterm/xterm/css/xterm.css";
import "./TerminalPanel.css";

export interface TerminalPanelProps {
  /** Names of the Tauri commands backing each terminal — defaults match substrate-platform's own pty module. `spawn`/`write`/`resize`/`kill` all take a string `id` addressing one session. */
  commands?: { spawn: string; write: string; resize: string; kill: string };
  /** Tauri event carrying `{ id, data }` PTY output chunks. */
  outputEvent?: string;
  /** Tauri event carrying `{ id }` when a shell exits. */
  exitEvent?: string;
}

const DEFAULT_COMMANDS = { spawn: "pty_spawn", write: "pty_write", resize: "pty_resize", kill: "pty_kill" };

interface TerminalTab {
  id: string;
  label: string;
}

/**
 * A VS-Code-style integrated terminal panel: a tab strip of independent
 * PTY-backed shells (each an xterm bound to one `pty_*` session id), a "+" to
 * open more, and per-tab close. Inactive terminals stay mounted (scrollback and
 * their shell keep running), so switching tabs is instant. Product-agnostic —
 * any Substrate-Platform product gets it by dropping it into a PlatformShell
 * tool window and exposing the matching `pty_*` Tauri commands.
 */
export function TerminalPanel({ commands = DEFAULT_COMMANDS, outputEvent = "pty-output", exitEvent = "pty-exit" }: TerminalPanelProps) {
  const [tabs, setTabs] = useState<TerminalTab[]>([]);
  const [activeId, setActiveId] = useState<string | null>(null);
  const counterRef = useRef(0);

  function addTerminal() {
    counterRef.current += 1;
    const n = counterRef.current;
    const id = `term-${n}-${Date.now()}`;
    setTabs((prev) => [...prev, { id, label: `Terminal ${n}` }]);
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

  // Open the first terminal on mount.
  useEffect(() => {
    addTerminal();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className="sp-terminal-manager">
      <div className="sp-terminal-tabs">
        {tabs.map((t) => (
          <Tab
            key={t.id}
            orientation="horizontal-dock"
            active={t.id === activeId}
            onClick={() => setActiveId(t.id)}
            onRequestClose={() => closeTerminal(t.id)}
          >
            {t.label}
          </Tab>
        ))}
        <IconButton size={22} title="New terminal" onClick={addTerminal}>
          <Icon name="plus" size={16} />
        </IconButton>
      </div>
      <div className="sp-terminal-views">
        {tabs.map((t) => (
          <TerminalInstance
            key={t.id}
            id={t.id}
            active={t.id === activeId}
            commands={commands}
            outputEvent={outputEvent}
            exitEvent={exitEvent}
            onExit={() => closeTerminal(t.id)}
          />
        ))}
      </div>
    </div>
  );
}

interface TerminalInstanceProps {
  id: string;
  active: boolean;
  commands: { spawn: string; write: string; resize: string; kill: string };
  outputEvent: string;
  exitEvent: string;
  onExit: () => void;
}

function TerminalInstance({ id, active, commands, outputEvent, exitEvent, onExit }: TerminalInstanceProps) {
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

    invoke(commands.spawn, { id, cols: term.cols, rows: term.rows })
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
