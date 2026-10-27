// The protocol's shapes as the editor and the Error List take them. Only
// what's read here is typed: a server's JSON is otherwise passed along.

import type { Completion } from "@codemirror/autocomplete";
import type { Text } from "@codemirror/state";
import type { Diagnostic, DiagnosticSeverity } from "../infra/diagnostics";
import type { FileOpenRequest } from "../documents/fileOpenBus";
import { pathFromUri } from "./uri";

/** A zero-based line and UTF-16 column — what CodeMirror's offsets count too. */
export interface LspPosition {
  line: number;
  character: number;
}

export interface LspRange {
  start: LspPosition;
  end: LspPosition;
}

export interface LspLocation {
  uri: string;
  range: LspRange;
}

export interface LspLocationLink {
  targetUri: string;
  targetRange: LspRange;
  targetSelectionRange?: LspRange;
}

export interface LspMarkupContent {
  kind: "markdown" | "plaintext";
  value: string;
}

/** A string, `{ language, value }`, or markup — what hover and documentation hold. */
export type LspMarkup = string | LspMarkupContent | { language: string; value: string };

export interface LspDiagnostic {
  range: LspRange;
  severity?: number;
  code?: string | number;
  codeDescription?: { href: string };
  source?: string;
  message: string;
}

export interface LspCompletionItem {
  label: string;
  kind?: number;
  detail?: string;
  documentation?: LspMarkup;
  insertText?: string;
  textEdit?: { newText: string; range?: LspRange; insert?: LspRange };
}

export type LspCompletionResult = LspCompletionItem[] | { isIncomplete?: boolean; items: LspCompletionItem[] } | null;

export interface LspHover {
  contents: LspMarkup | LspMarkup[];
  range?: LspRange;
}

/** The offset of `position` in `doc`, clamped into it. */
export function toOffset(doc: Text, position: LspPosition): number {
  const line = doc.line(Math.min(Math.max(0, position.line), doc.lines - 1) + 1);
  return line.from + Math.min(Math.max(0, position.character), line.length);
}

export function toPosition(doc: Text, offset: number): LspPosition {
  const line = doc.lineAt(offset);
  return { line: line.number - 1, character: offset - line.from };
}

const SEVERITY: Record<number, DiagnosticSeverity> = { 1: "error", 2: "warning", 3: "message", 4: "message" };

/** A `publishDiagnostics` list for `path` as Error List entries (1-based, end column exclusive). */
export function toDiagnostics(path: string, items: readonly LspDiagnostic[], tool?: string): Diagnostic[] {
  return items.map((d) => ({
    severity: SEVERITY[d.severity ?? 1] ?? "error",
    message: d.message,
    code: d.code === undefined ? undefined : String(d.code),
    codeHref: d.codeDescription?.href,
    file: path,
    line: d.range.start.line + 1,
    column: d.range.start.character + 1,
    endLine: d.range.end.line + 1,
    endColumn: d.range.end.character + 1,
    tool: d.source ?? tool,
    origin: "live",
  }));
}

/** The protocol's `CompletionItemKind`s as CodeMirror's completion types (its icons). */
const COMPLETION_TYPE: Record<number, string> = {
  1: "text",
  2: "method",
  3: "function",
  4: "function",
  5: "property",
  6: "variable",
  7: "class",
  8: "interface",
  9: "namespace",
  10: "property",
  12: "constant",
  13: "enum",
  14: "keyword",
  15: "text",
  20: "enum",
  21: "constant",
  22: "class",
  23: "keyword",
  25: "type",
};

/** The text of hover contents or documentation — Markdown left as the server wrote it. */
export function markupText(markup: LspMarkup | LspMarkup[] | undefined): string {
  if (markup === undefined) return "";
  if (Array.isArray(markup)) return markup.map(markupText).filter(Boolean).join("\n\n");
  if (typeof markup === "string") return markup;
  if ("kind" in markup) return markup.value;
  return "```" + markup.language + "\n" + markup.value + "\n```";
}

/** A completion list as CodeMirror's options; `from` is where the typed word starts. */
export function toCompletions(doc: Text, result: LspCompletionResult, from: number): { from: number; options: Completion[] } {
  const items = result === null ? [] : Array.isArray(result) ? result : result.items;
  let start = from;
  const options = items.map((item) => {
    const edit = item.textEdit;
    const range = edit?.range ?? edit?.insert;
    if (range) start = Math.min(start, toOffset(doc, range.start));
    const info = markupText(item.documentation);
    return {
      label: item.label,
      type: item.kind === undefined ? undefined : COMPLETION_TYPE[item.kind],
      detail: item.detail,
      info: info || undefined,
      apply: edit?.newText ?? item.insertText ?? item.label,
    };
  });
  return { from: start, options };
}

/** Where a `definition` answer points — its first target — as an "open this file" request. */
export function toOpenRequest(result: LspLocation | LspLocation[] | LspLocationLink[] | null): FileOpenRequest | undefined {
  const first = Array.isArray(result) ? result[0] : result;
  if (!first) return undefined;
  const uri = "targetUri" in first ? first.targetUri : first.uri;
  const range = "targetUri" in first ? (first.targetSelectionRange ?? first.targetRange) : first.range;
  const path = pathFromUri(uri);
  if (path === undefined) return undefined;
  return { path, line: range.start.line + 1, column: range.start.character + 1 };
}
