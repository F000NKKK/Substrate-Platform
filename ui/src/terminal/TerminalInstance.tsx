import { useEffect, useRef } from "react";
import type { Terminal as XTerm } from "@xterm/xterm";
import { useT } from "../infra/i18n";
import { ContextMenu, useContextMenu } from "../widgets/ContextMenu";
import { copySelection, pasteInto } from "./clipboard";
import { useTerminalSession } from "./useTerminalSession";

export interface TerminalInstanceProps {
  id: string;
  shell?: string;
  cwd?: string;
  clearCommand: string;
  /** Its split group is shown (xterm can only measure then); several panes can be active at once. */
  active: boolean;
  /** It gets the keyboard — at most one terminal. */
  focused: boolean;
  onExit: () => void;
  /** Hands the panel this terminal's `clear()` on mount, `null` on unmount — Clear Terminal. */
  onReady: (clear: (() => void) | null) => void;
}

/** One shell session in xterm: started on mount, killed on unmount. */
export function TerminalInstance({ id, shell, cwd, clearCommand, active, focused, onExit, onReady }: TerminalInstanceProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const fitRef = useRef<(() => void) | null>(null);
  const termRef = useRef<XTerm | null>(null);
  const onExitRef = useRef(onExit);
  onExitRef.current = onExit;
  const onReadyRef = useRef(onReady);
  onReadyRef.current = onReady;
  const menu = useContextMenu<void>();
  const t = useT();

  useTerminalSession(id, shell, cwd, clearCommand, { container: containerRef, term: termRef, fit: fitRef, onExit: onExitRef, onReady: onReadyRef });

  // xterm can't measure while hidden — fit again when its group shows.
  useEffect(() => {
    if (active) fitRef.current?.();
  }, [active]);

  // The keyboard follows the focused terminal, or typing lands where focus was.
  useEffect(() => {
    if (focused) termRef.current?.focus();
  }, [focused]);

  return (
    <div
      ref={containerRef}
      className="sp-terminal-view"
      // Ctrl+<letter> belongs to the shell (Ctrl+W, Ctrl+S…), not the IDE.
      data-sp-native-keys=""
      data-active={active || undefined}
      data-focused={focused || undefined}
      onContextMenu={(e) => menu.openAtPoint(undefined, e)}
    >
      <ContextMenu
        target={menu.target && menu.target.mode === "viewport" ? { mode: "viewport", x: menu.target.x, y: menu.target.y } : null}
        items={[
          { label: t("Copy"), shortcut: "Ctrl+Shift+C", disabled: !termRef.current?.hasSelection(), onSelect: () => copySelection(termRef.current?.getSelection()) },
          { label: t("Paste"), shortcut: "Ctrl+Shift+V", onSelect: () => pasteInto(id) },
        ]}
        onClose={menu.close}
      />
    </div>
  );
}
