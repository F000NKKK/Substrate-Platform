/** Visual Studio's three Error List levels. */
export type DiagnosticSeverity = "error" | "warning" | "message";

/**
 * Where a diagnostic came from, like the Error List's "Build + IntelliSense"
 * filter: `build` from the last build's output, `live` from analysis that
 * runs while editing (on save, on type).
 */
export type DiagnosticOrigin = "build" | "live";

/** One entry in the Error List. Positions are 1-based, as compilers print them. */
export interface Diagnostic {
  severity: DiagnosticSeverity;
  message: string;
  /** Error/lint code (`E0425`, `unused_variables`). */
  code?: string;
  /** Documentation for `code` — the Code column links to it. */
  codeHref?: string;
  /** Absolute path, when the diagnostic points into a file. */
  file?: string;
  line?: number;
  column?: number;
  /** End of the span (exclusive column); default: to the end of the word at `column`. */
  endLine?: number;
  endColumn?: number;
  /** Project (crate, module, package) the diagnostic belongs to. */
  project?: string;
  /** What reported it (`rustc`, `clippy`, a format checker…). */
  tool?: string;
  origin: DiagnosticOrigin;
  /** Quick fixes — offered in the Error List's menu and on the editor's squiggle. */
  fixes?: readonly DiagnosticFix[];
}

/** One quick action that resolves (or works around) a diagnostic. */
export interface DiagnosticFix {
  label: string;
  apply: () => void;
}
