import type { UnlistenFn } from "@tauri-apps/api/event";
import { clearDiagnostics, setDiagnostics } from "../infra/diagnostics";
import { errorText } from "../infra/i18n";
import { reportError } from "../output/reportError";
import { toDiagnostics, type LspDiagnostic } from "./convert";
import type { LspHost } from "./lspHost";
import { pathFromUri } from "./uri";

/** The Error List's set for a server's diagnostics of one file. */
function diagnosticsKey(root: string, language: string, path: string): string {
  return `lsp:${root}\u0000${language}\u0000${path}`;
}

/**
 * One language server for one workspace root, as the editors of its files
 * share it: started on first use, its diagnostics kept in the Error List
 * (`live`), its failure to start reported once.
 */
export class LspSession {
  private started?: Promise<unknown>;

  constructor(
    readonly host: LspHost,
    readonly root: string,
    readonly language: string
  ) {}

  /** Starts the server once: its capabilities. A failed start is reported and retried on the next use. */
  start(): Promise<unknown> {
    if (!this.started) {
      watchDiagnostics(this.host);
      this.started = this.host.start(this.root, this.language).catch((err) => {
        this.started = undefined;
        reportError(errorText(err));
        throw err;
      });
    }
    return this.started;
  }

  async request<T>(method: string, params: unknown): Promise<T> {
    await this.start();
    return this.host.request<T>(this.root, this.language, method, params);
  }

  async didOpen(path: string, version: number, text: string): Promise<void> {
    await this.start();
    await this.host.didOpen(this.root, this.language, path, version, text);
  }

  didChange(path: string, version: number, text: string): Promise<void> {
    return this.host.didChange(this.root, this.language, path, version, text);
  }

  didSave(path: string): Promise<void> {
    return this.host.didSave(this.root, this.language, path);
  }

  async didClose(path: string): Promise<void> {
    await this.host.didClose(this.root, this.language, path);
  }

  /** Stops the server and drops its diagnostics. */
  async stop(): Promise<void> {
    sessions.delete(sessionKey(this.host, this.root, this.language));
    this.started = undefined;
    clearDiagnostics((key) => key.startsWith(diagnosticsKey(this.root, this.language, "")));
    await this.host.stop(this.root, this.language);
  }
}

const hostIds = new WeakMap<LspHost, number>();
let nextHostId = 1;
const sessions = new Map<string, LspSession>();

function sessionKey(host: LspHost, root: string, language: string): string {
  if (!hostIds.has(host)) hostIds.set(host, nextHostId++);
  return `${hostIds.get(host)}\u0000${root}\u0000${language}`;
}

/** The session of `language`'s server for `root` — the same one for every editor asking. */
export function lspSession(host: LspHost, root: string, language: string): LspSession {
  const key = sessionKey(host, root, language);
  let session = sessions.get(key);
  if (!session) {
    session = new LspSession(host, root, language);
    sessions.set(key, session);
  }
  return session;
}

/** Stops every server — the workspace closed. */
export async function stopLspSessions(): Promise<void> {
  await Promise.all([...sessions.values()].map((s) => s.stop().catch(() => {})));
}

const watched = new WeakMap<LspHost, Promise<UnlistenFn>>();

/** Keeps every server's `publishDiagnostics` in the Error List, once per host. */
function watchDiagnostics(host: LspHost): void {
  if (watched.has(host)) return;
  watched.set(
    host,
    host.onNotification(({ root, language, method, params }) => {
      if (method !== "textDocument/publishDiagnostics") return;
      const { uri, diagnostics } = params as { uri: string; diagnostics: LspDiagnostic[] };
      const path = pathFromUri(uri);
      if (path === undefined) return;
      setDiagnostics(diagnosticsKey(root, language, path), toDiagnostics(path, diagnostics ?? [], language));
    })
  );
}
