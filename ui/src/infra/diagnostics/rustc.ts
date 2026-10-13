import type { Diagnostic, DiagnosticOrigin, DiagnosticSeverity } from "./types";

export interface RustcParserOptions {
  /** Default `build`. */
  origin?: DiagnosticOrigin;
  /** Project column for every diagnostic; default: the crate cargo last said it was compiling. */
  project?: string;
  /** Turns a path as printed (relative to the build's directory) into the one to open. */
  resolvePath?: (path: string) => string;
}

export interface RustcParser {
  /** One line of output; returns the diagnostics it completed. */
  push(line: string): Diagnostic[];
  /** End of output — returns the one still open, if any. */
  flush(): Diagnostic[];
}

const HEADER = /^(error|warning)(?:\[([A-Za-z0-9_:]+)\])?: (.*)$/;
const LOCATION = /^\s*--> (.+):(\d+):(\d+)$/;
const LINT_LEVEL = /#\[(?:warn|deny|forbid)\(([A-Za-z0-9_:]+)\)\]/;
const COMPILING = /^\s*(?:Compiling|Checking) (\S+) v/;
/** Cargo's and rustc's closing tallies — not diagnostics of their own. */
const SUMMARY = [
  /^error: could not compile /,
  /^error: aborting due to /,
  /^warning: build failed, waiting for other jobs/,
  /^warning: `[^`]+` \([^)]*\) generated \d+ warnings?/,
];
const isSummary = (message: string) => SUMMARY.some((re) => re.test(message));

/** Documentation for a rustc error code or a clippy lint. */
export function rustcCodeHref(code: string): string | undefined {
  if (/^E\d{4}$/.test(code)) return `https://doc.rust-lang.org/error_codes/${code}.html`;
  if (code.startsWith("clippy::")) return `https://rust-lang.github.io/rust-clippy/master/index.html#${code.slice("clippy::".length)}`;
  return undefined;
}

function severityOf(level: string): DiagnosticSeverity {
  if (level.startsWith("error")) return "error";
  if (level === "warning") return "warning";
  return "message";
}

function withCode(d: Diagnostic, code: string | undefined): Diagnostic {
  if (!code) return d;
  return { ...d, code, codeHref: rustcCodeHref(code), tool: code.startsWith("clippy::") ? "clippy" : d.tool };
}

interface JsonSpan {
  file_name: string;
  line_start: number;
  column_start: number;
  is_primary: boolean;
}
interface JsonMessage {
  message: string;
  level: string;
  code?: { code: string } | null;
  spans?: JsonSpan[];
}

/**
 * Reads rustc diagnostics out of a build's output, in either of cargo's
 * formats: the human one (`error[E0425]: …` then ` --> src/lib.rs:3:5`)
 * and `--message-format=json` lines. Everything else passes through.
 */
export function createRustcParser(options: RustcParserOptions = {}): RustcParser {
  const origin = options.origin ?? "build";
  const resolve = options.resolvePath ?? ((p: string) => p);
  let crate: string | undefined;
  let open: Diagnostic | null = null;
  let located = false;

  const project = () => options.project ?? crate;
  function close(): Diagnostic[] {
    const done = open;
    open = null;
    located = false;
    return done ? [done] : [];
  }

  function fromJson(line: string): Diagnostic[] | null {
    let value: unknown;
    try {
      value = JSON.parse(line);
    } catch {
      return null;
    }
    if (!value || typeof value !== "object") return null;
    const record = value as Record<string, unknown>;
    let message: JsonMessage;
    let target: string | undefined;
    if (record.reason === "compiler-message") {
      message = record.message as JsonMessage;
      target = (record.target as { name?: string } | undefined)?.name;
    } else if (record.$message_type === "diagnostic") {
      message = record as unknown as JsonMessage;
    } else {
      return [];
    }
    if (message.level === "failure-note" || (!message.spans?.length && isSummary(`${message.level}: ${message.message}`))) return [];
    const span = message.spans?.find((s) => s.is_primary) ?? message.spans?.[0];
    const d: Diagnostic = {
      severity: severityOf(message.level),
      message: message.message,
      file: span ? resolve(span.file_name) : undefined,
      line: span?.line_start,
      column: span?.column_start,
      project: options.project ?? target,
      tool: "rustc",
      origin,
    };
    return [withCode(d, message.code?.code)];
  }

  return {
    push(raw) {
      const line = raw.replace(/\r$/, "");
      if (line.startsWith("{")) {
        const parsed = fromJson(line);
        if (parsed) return [...close(), ...parsed];
      }
      const compiling = COMPILING.exec(line);
      if (compiling) {
        crate = compiling[1];
        return close();
      }
      const header = HEADER.exec(line);
      if (header) {
        const done = close();
        const [, level, code, message] = header;
        if (isSummary(line)) return done;
        open = withCode({ severity: severityOf(level), message, project: project(), tool: "rustc", origin }, code);
        return done;
      }
      if (!open) return [];
      if (line.trim() === "") return close();
      const location = !located && LOCATION.exec(line);
      if (location) {
        located = true;
        open = { ...open, file: resolve(location[1]), line: Number(location[2]), column: Number(location[3]) };
        return [];
      }
      const lint = !open.code && LINT_LEVEL.exec(line);
      if (lint) open = withCode(open, lint[1]);
      return [];
    },
    flush: close,
  };
}
