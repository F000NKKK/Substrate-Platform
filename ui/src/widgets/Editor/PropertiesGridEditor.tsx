import { useMemo } from "react";
import { DataGrid, type DataGridColumn } from "../DataGrid";
import type { EditorProps } from "./EditorBase";

interface PropertyRow {
  id: string;
  key: string;
  value: string;
}

/**
 * Parses `key=value` (or `key: value`) lines into rows — comments and blank
 * lines are dropped rather than preserved, so round-tripping through this
 * editor is lossy for anything beyond plain key/value pairs. Good enough for
 * `.properties`/`gradle.properties`-style files, which are exactly that.
 */
function parseProperties(content: string): PropertyRow[] {
  const rows: PropertyRow[] = [];
  content.split(/\r?\n/).forEach((line, i) => {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#") || trimmed.startsWith("!")) return;
    const match = trimmed.match(/^([^=:]+)[=:](.*)$/);
    if (!match) return;
    rows.push({ id: `${i}`, key: match[1].trim(), value: match[2].trim() });
  });
  return rows;
}

function serializeProperties(rows: PropertyRow[]): string {
  return rows.map((row) => `${row.key}=${row.value}`).join("\n") + "\n";
}

const columns: DataGridColumn<PropertyRow>[] = [
  { key: "key", header: "Key", width: 220, sortable: true, editable: true, sortValue: (r) => r.key },
  { key: "value", header: "Value", width: 260, editable: true },
];

/**
 * A sibling branch off `EditorBase`, like `ImageEditor` — not text-based
 * editing, but a grid over parsed key/value rows, reusing the platform's
 * `DataGrid` rather than a bespoke table.
 */
export function PropertiesGridEditor({ content, onChange, readOnly }: EditorProps) {
  const rows = useMemo(() => parseProperties(content), [content]);

  function commitEdit(row: PropertyRow, key: string, value: string) {
    if (readOnly) return;
    const next = rows.map((r) => (r.id === row.id ? { ...r, [key]: value } : r));
    onChange(serializeProperties(next));
  }

  return (
    <div style={{ height: "100%", boxSizing: "border-box" }}>
      <DataGrid columns={columns} rows={rows} getRowId={(r) => r.id} onCellEditCommit={commitEdit} filterRow />
    </div>
  );
}
