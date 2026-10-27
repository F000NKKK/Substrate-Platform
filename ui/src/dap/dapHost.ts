import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { events, substrate } from "../infra/host/host.generated";
import { invokeCommand, type HostCommand } from "../infra/host/invokeCommand";

/** A debug adapter as the host knows it: its id and the languages it debugs. */
export interface AdapterInfo {
  id: string;
  /** Language ids, as the language servers' list names them. */
  languages: string[];
}

/**
 * What an adapter said unasked — the host's `Event`, `{ event, body }`.
 * An event the host doesn't know, or one whose body doesn't fit, is `other`
 * with the name and body as they came.
 */
export type DapEvent =
  | { event: "initialized" }
  | {
      event: "stopped";
      body: {
        reason: string;
        threadId?: number;
        allThreadsStopped?: boolean;
        description?: string;
        text?: string;
        hitBreakpointIds?: number[];
      };
    }
  | { event: "continued"; body: { threadId: number; allThreadsContinued?: boolean } }
  | { event: "output"; body: { category: string; output: string; variablesReference?: number } }
  | { event: "terminated"; body: { restart?: unknown } }
  | { event: "exited"; body: { exitCode: number } }
  | { event: "breakpoint"; body: { reason: string; breakpoint: unknown } }
  | { event: "thread"; body: { reason: string; threadId: number } }
  | { event: "other"; body: { name: string; body: unknown } };

/** An adapter's event, with the session it belongs to. */
export interface DapEventMessage {
  root: string;
  adapter: string;
  event: DapEvent;
}

/** A line of an adapter's own log (its stderr). */
export interface DapOutput {
  root: string;
  adapter: string;
  line: string;
}

/**
 * The host side of debug adapters — one session per workspace root and
 * adapter. Arguments and results are the protocol's JSON.
 */
export interface DapHost {
  adapters(): Promise<AdapterInfo[]>;
  /** Starts and initializes the adapter (a running one is reused): its capabilities. */
  start(root: string, adapter: string): Promise<unknown>;
  /** `command` with `arguments` to the adapter: the response's body. */
  request<T>(root: string, adapter: string, command: string, args: unknown): Promise<T>;
  stop(root: string, adapter: string): Promise<void>;
  onEvent(handler: (message: DapEventMessage) => void): Promise<UnlistenFn>;
  onOutput(handler: (output: DapOutput) => void): Promise<UnlistenFn>;
}

/** The commands and events a host serves debug adapters with — Substrate's plugin's by default. */
export interface DapHostNames {
  commands: {
    dapAdapters: HostCommand;
    dapStart: HostCommand;
    dapRequest: HostCommand;
    dapStop: HostCommand;
  };
  events: { event: string; output: string };
}

const SUBSTRATE_NAMES: DapHostNames = {
  commands: substrate.dap,
  events: { event: events.dapEvent, output: events.dapOutput },
};

/** Debug adapters over a Tauri host (the plugin's feature `dap`, or a product's own commands of the same shape). */
export function tauriDapHost(names: DapHostNames = SUBSTRATE_NAMES): DapHost {
  const { commands } = names;
  return {
    adapters: () => invokeCommand<AdapterInfo[]>(commands.dapAdapters),
    start: (root, adapter) => invokeCommand<unknown>(commands.dapStart, { root, adapter }),
    request: <T>(root: string, adapter: string, command: string, args: unknown) =>
      invokeCommand<T>(commands.dapRequest, { root, adapter, command, arguments: args }),
    stop: (root, adapter) => invokeCommand<void>(commands.dapStop, { root, adapter }),
    onEvent: (handler) => listen<DapEventMessage>(names.events.event, (e) => handler(e.payload)),
    onOutput: (handler) => listen<DapOutput>(names.events.output, (e) => handler(e.payload)),
  };
}

/** The first adapter that debugs `language`. */
export function adapterFor(adapters: readonly AdapterInfo[], language: string): AdapterInfo | undefined {
  return adapters.find((a) => a.languages.includes(language));
}
