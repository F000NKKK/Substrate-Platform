import type { Diagnostic, DiagnosticOrigin, DiagnosticSeverity } from "../../infra/diagnostics";

/** Visual Studio's scope dropdown — which files' diagnostics are listed. */
export type ErrorListScope = "solution" | "open" | "current";
/** Its "Build + IntelliSense" dropdown. */
export type ErrorListOrigins = "all" | DiagnosticOrigin;

export interface ErrorListFilter {
  scope: ErrorListScope;
  origins: ErrorListOrigins;
  /** The Errors / Warnings / Messages toggle buttons. */
  shown: Record<DiagnosticSeverity, boolean>;
}

export const DEFAULT_ERROR_LIST_FILTER: ErrorListFilter = {
  scope: "solution",
  origins: "all",
  shown: { error: true, warning: true, message: true },
};

export interface ErrorListContext {
  activeFile?: string | null;
  openFiles?: readonly string[];
}

export interface FilteredDiagnostics {
  /** Indexes into the input, in its order. */
  rows: number[];
  /** Per severity, what scope, origin and search let through — the toggle buttons' counts, independent of the toggles themselves. */
  counts: Record<DiagnosticSeverity, number>;
}

/** The text the search box matches: every visible column. */
export function diagnosticSearchText(d: Diagnostic): string {
  return [d.code, d.message, d.project, d.file, d.line].filter((v) => v !== undefined).join("\u0000").toLowerCase();
}

/** Applies the toolbar's filters, as the Error List does: scope, origin and search narrow everything; the severity toggles only hide. */
export function filterDiagnostics(
  diagnostics: readonly Diagnostic[],
  filter: ErrorListFilter,
  search: string,
  context: ErrorListContext = {}
): FilteredDiagnostics {
  const needle = search.trim().toLowerCase();
  const open = filter.scope === "open" ? new Set(context.openFiles ?? []) : null;
  const counts: Record<DiagnosticSeverity, number> = { error: 0, warning: 0, message: 0 };
  const rows: number[] = [];
  diagnostics.forEach((d, i) => {
    if (filter.origins !== "all" && d.origin !== filter.origins) return;
    if (filter.scope === "current" && (!d.file || d.file !== context.activeFile)) return;
    if (open && (!d.file || !open.has(d.file))) return;
    if (needle && !diagnosticSearchText(d).includes(needle)) return;
    counts[d.severity]++;
    if (filter.shown[d.severity]) rows.push(i);
  });
  return { rows, counts };
}

const SEVERITY_RANK: Record<DiagnosticSeverity, number> = { error: 0, warning: 1, message: 2 };
export const severityRank = (s: DiagnosticSeverity) => SEVERITY_RANK[s];

/** Tab-separated, header first — what Copy puts on the clipboard, pasteable into a spreadsheet. */
export function diagnosticsToTsv(items: readonly Diagnostic[], header: readonly string[], severityLabel: (s: DiagnosticSeverity) => string): string {
  const lines = items.map((d) =>
    [severityLabel(d.severity), d.code ?? "", d.message, d.project ?? "", d.file ?? "", d.line ?? ""].map((v) => String(v).replace(/[\t\n]/g, " ")).join("\t")
  );
  return [header.join("\t"), ...lines].join("\n");
}
