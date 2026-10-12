import { useMemo } from "react";
import { DataGrid, type DataGridColumn } from "../DataGrid";
import type { EditorProps } from "./EditorBase";

interface PropertyRow {
  id: string;
  key: string;
  value: string;
  /** First physical line (0-indexed) this entry occupies. */
  start: number;
  /** One past its last physical line — more than `start + 1` when the value uses `\` line continuations. */
  end: number;
  /** Leading whitespace before the key, kept verbatim on rewrite. */
  indent: string;
  /** Everything between key and value (`=`, ` = `, `: `, or plain whitespace), kept verbatim on rewrite. */
  separator: string;
}

interface ParsedProperties {
  lines: string[];
  eol: string;
  rows: PropertyRow[];
}

/** A line continues onto the next one when it ends in an odd number of backslashes (an even number is just escaped backslashes). */
function continues(line: string): boolean {
  const trailing = line.match(/\\+$/)?.[0].length ?? 0;
  return trailing % 2 === 1;
}

/**
 * Splits one logical entry into key/separator/value per the `.properties`
 * rules: the key ends at the first unescaped `=`, `:` or whitespace; the
 * separator swallows surrounding whitespace plus at most one `=`/`:`.
 */
function splitEntry(text: string): { key: string; separator: string; value: string } {
  let i = 0;
  while (i < text.length) {
    const ch = text[i];
    if (ch === "\\") {
      i += 2;
      continue;
    }
    if (ch === "=" || ch === ":" || ch === " " || ch === "\t" || ch === "\f") break;
    i++;
  }
  const key = text.slice(0, i);
  const sep = text.slice(i).match(/^[ \t\f]*[=:]?[ \t\f]*/)?.[0] ?? "";
  return { key, separator: sep, value: text.slice(i + sep.length) };
}

/**
 * Parses key/value entries into rows while keeping every physical line
 * (comments, blank lines, ordering, line endings) — each row remembers which
 * lines it came from, so an edit rewrites only those lines and the rest of
 * the file round-trips byte-for-byte.
 */
function parseProperties(content: string): ParsedProperties {
  const eol = content.includes("\r\n") ? "\r\n" : "\n";
  const lines = content.split(/\r?\n/);
  const rows: PropertyRow[] = [];
  let i = 0;
  while (i < lines.length) {
    const line = lines[i];
    const trimmed = line.trimStart();
    if (!trimmed || trimmed.startsWith("#") || trimmed.startsWith("!")) {
      i++;
      continue;
    }
    const start = i;
    let logical = trimmed;
    while (continues(lines[i]) && i + 1 < lines.length) {
      logical = logical.slice(0, -1) + lines[i + 1].trimStart();
      i++;
    }
    i++;
    const { key, separator, value } = splitEntry(logical);
    rows.push({ id: `${start}`, key, value, start, end: i, indent: line.slice(0, line.length - trimmed.length), separator: separator || "=" });
  }
  return { lines, eol, rows };
}

const columns: DataGridColumn<PropertyRow>[] = [
  { key: "key", header: "Key", width: 220, sortable: true, editable: true, sortValue: (r) => r.key },
  { key: "value", header: "Value", width: 260, editable: true },
];

/**
 * A sibling branch off `EditorBase`, like `ImageEditor` — not text-based
 * editing, but a grid over parsed key/value rows, reusing the platform's
 * `DataGrid` rather than a bespoke table. Lossless: comments, blank lines
 * and entry order survive an edit untouched (only the edited entry's own
 * lines are rewritten).
 */
export function PropertiesGridEditor({ content, onChange, readOnly }: EditorProps) {
  const parsed = useMemo(() => parseProperties(content), [content]);

  function commitEdit(row: PropertyRow, key: string, value: string) {
    if (readOnly) return;
    const next = { ...row, [key]: value };
    const rewritten = `${row.indent}${next.key}${row.separator}${next.value}`;
    const lines = [...parsed.lines.slice(0, row.start), rewritten, ...parsed.lines.slice(row.end)];
    onChange(lines.join(parsed.eol));
  }

  return (
    <div style={{ height: "100%", boxSizing: "border-box" }}>
      <DataGrid columns={columns} rows={parsed.rows} getRowId={(r) => r.id} onCellEditCommit={commitEdit} filterRow />
    </div>
  );
}
