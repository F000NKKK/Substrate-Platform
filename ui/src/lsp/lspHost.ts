import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { events, substrate } from "../infra/host/host.generated";
import { invokeCommand, type HostCommand } from "../infra/host/invokeCommand";

/** A language as the host knows it: its files and whether a server serves it. */
export interface LanguageInfo {
  id: string;
  /** Extensions without the dot, compared ignoring case. */
  extensions: string[];
  /** Whole file names (`Cargo.lock`), compared exactly. */
  fileNames: string[];
  hasServer: boolean;
}

/** What a server said unasked (`textDocument/publishDiagnostics`, `window/logMessage`…). */
export interface LspNotification {
  root: string;
  language: string;
  method: string;
  params: unknown;
}

/** A line of a server's own log (its stderr). */
export interface LspOutput {
  root: string;
  language: string;
  line: string;
}

/**
 * The host side of language servers — one server per workspace root and
 * language. Params and results are the protocol's JSON.
 */
export interface LspHost {
  languages(): Promise<LanguageInfo[]>;
  /** Starts the server (a running one is reused): its capabilities. */
  start(root: string, language: string): Promise<unknown>;
  request<T>(root: string, language: string, method: string, params: unknown): Promise<T>;
  notify(root: string, language: string, method: string, params: unknown): Promise<void>;
  didOpen(root: string, language: string, path: string, version: number, text: string): Promise<void>;
  didChange(root: string, language: string, path: string, version: number, text: string): Promise<void>;
  didSave(root: string, language: string, path: string): Promise<void>;
  didClose(root: string, language: string, path: string): Promise<void>;
  stop(root: string, language: string): Promise<void>;
  onNotification(handler: (notification: LspNotification) => void): Promise<UnlistenFn>;
  onOutput(handler: (output: LspOutput) => void): Promise<UnlistenFn>;
}

/** The commands and events a host serves language servers with — Substrate's plugin's by default. */
export interface LspHostNames {
  commands: {
    lspLanguages: HostCommand;
    lspStart: HostCommand;
    lspRequest: HostCommand;
    lspNotify: HostCommand;
    lspDidOpen: HostCommand;
    lspDidChange: HostCommand;
    lspDidSave: HostCommand;
    lspDidClose: HostCommand;
    lspStop: HostCommand;
  };
  events: { notification: string; output: string };
}

const SUBSTRATE_NAMES: LspHostNames = {
  commands: substrate.lsp,
  events: { notification: events.lspNotification, output: events.lspOutput },
};

/** Language servers over a Tauri host (the plugin's feature `lsp`, or a product's own commands of the same shape). */
export function tauriLspHost(names: LspHostNames = SUBSTRATE_NAMES): LspHost {
  const { commands } = names;
  return {
    languages: () => invokeCommand<LanguageInfo[]>(commands.lspLanguages),
    start: (root, language) => invokeCommand<unknown>(commands.lspStart, { root, language }),
    request: <T>(root: string, language: string, method: string, params: unknown) =>
      invokeCommand<T>(commands.lspRequest, { root, language, method, params }),
    notify: (root, language, method, params) => invokeCommand<void>(commands.lspNotify, { root, language, method, params }),
    didOpen: (root, language, path, version, text) => invokeCommand<void>(commands.lspDidOpen, { root, language, path, version, text }),
    didChange: (root, language, path, version, text) => invokeCommand<void>(commands.lspDidChange, { root, language, path, version, text }),
    didSave: (root, language, path) => invokeCommand<void>(commands.lspDidSave, { root, language, path }),
    didClose: (root, language, path) => invokeCommand<void>(commands.lspDidClose, { root, language, path }),
    stop: (root, language) => invokeCommand<void>(commands.lspStop, { root, language }),
    onNotification: (handler) => listen<LspNotification>(names.events.notification, (e) => handler(e.payload)),
    onOutput: (handler) => listen<LspOutput>(names.events.output, (e) => handler(e.payload)),
  };
}

/** The language `path` is in — a file name match before an extension. */
export function languageFor(languages: readonly LanguageInfo[], path: string): LanguageInfo | undefined {
  const name = path.split(/[\\/]/).pop() ?? path;
  const dot = name.lastIndexOf(".");
  const extension = dot > 0 ? name.slice(dot + 1).toLowerCase() : undefined;
  return (
    languages.find((l) => l.fileNames.includes(name)) ??
    (extension === undefined ? undefined : languages.find((l) => l.extensions.some((e) => e.toLowerCase() === extension)))
  );
}
