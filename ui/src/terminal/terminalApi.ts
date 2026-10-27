import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { events, substrate } from "../infra/host/host.generated";
import { invokeCommand } from "../infra/host/invokeCommand";

/** A shell "New Terminal" offers — the host's `ShellInfo`. `label` is English (an i18n key). */
export interface ShellInfo {
  id: string;
  label: string;
  /** The program to start. */
  command: string;
  /** What the shell clears its screen with (a command it runs, so its line editor stays in sync). */
  clearCommand: string;
}

/** What to start: a terminal's id, its size, its shell and folder (none: the host's defaults). */
export interface SpawnSpec {
  id: string;
  cols: number;
  rows: number;
  shell?: string;
  cwd?: string;
}

/** The host plugin's terminal commands (feature `terminal`). */
export const terminalApi = {
  shells: () => invokeCommand<ShellInfo[]>(substrate.terminal.terminalShells),
  spawn: (spec: SpawnSpec) => invokeCommand<void>(substrate.terminal.terminalSpawn, { spec }),
  write: (id: string, data: string) => invokeCommand<void>(substrate.terminal.terminalWrite, { id, data }),
  resize: (id: string, cols: number, rows: number) => invokeCommand<void>(substrate.terminal.terminalResize, { id, cols, rows }),
  kill: (id: string) => invokeCommand<void>(substrate.terminal.terminalKill, { id }),
  onOutput: (handler: (id: string, data: string) => void): Promise<UnlistenFn> =>
    listen<{ id: string; data: string }>(events.terminalOutput, (e) => handler(e.payload.id, e.payload.data)),
  onExit: (handler: (id: string) => void): Promise<UnlistenFn> => listen<{ id: string }>(events.terminalExit, (e) => handler(e.payload.id)),
};
