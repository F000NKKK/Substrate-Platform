import { useEffect, useRef } from "react";
import { Terminal as XTerm } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "@xterm/xterm/css/xterm.css";
import "./TerminalPanel.css";

export interface TerminalPanelProps {
  /** Names of the Tauri commands backing this terminal — defaults match substrate-platform's own pty module. */
  commands?: { spawn: string; write: string; resize: string };
  /** Tauri event carrying PTY output chunks. */
  outputEvent?: string;
}

const DEFAULT_COMMANDS = { spawn: "pty_spawn", write: "pty_write", resize: "pty_resize" };

/**
 * Real PTY-backed terminal panel — spawns a shell via the host's `pty_spawn`
 * Tauri command (backed by `substrate_platform::PtySession`) and streams its
 * output through a Tauri event. Product-agnostic: any Substrate-Platform
 * product gets this for free by dropping it into a PlatformShell tool window.
 */
export function TerminalPanel({ commands = DEFAULT_COMMANDS, outputEvent = "pty-output" }: TerminalPanelProps) {
  const containerRef = useRef<HTMLDivElement>(null);

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
    fitAddon.fit();

    let disposed = false;
    const unlistenPromise = listen<string>(outputEvent, (event) => {
      if (!disposed) term.write(event.payload);
    });

    invoke(commands.spawn, { cols: term.cols, rows: term.rows }).catch((err) =>
      term.writeln(`\r\n[failed to start shell: ${err}]`)
    );

    const onData = term.onData((data) => {
      invoke(commands.write, { data }).catch(() => {});
    });

    const resizeObserver = new ResizeObserver(() => {
      fitAddon.fit();
      invoke(commands.resize, { cols: term.cols, rows: term.rows }).catch(() => {});
    });
    resizeObserver.observe(containerRef.current);

    return () => {
      disposed = true;
      resizeObserver.disconnect();
      onData.dispose();
      unlistenPromise.then((unlisten) => unlisten());
      term.dispose();
    };
  }, [commands, outputEvent]);

  return <div ref={containerRef} className="sp-terminal-panel" />;
}
