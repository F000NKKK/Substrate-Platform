import React from "react";
import { DataGrid, type DataGridColumn } from "substrate-platform-ui";

interface FileRow {
  id: string;
  name: string;
  kind: "folder" | "file";
  sizeBytes: number;
  modified: string;
}

const ROWS: FileRow[] = [
  { id: "1", name: "src", kind: "folder", sizeBytes: 0, modified: "2026-07-01" },
  { id: "2", name: "index.tsx", kind: "file", sizeBytes: 2140, modified: "2026-07-14" },
  { id: "3", name: "package.json", kind: "file", sizeBytes: 890, modified: "2026-07-10" },
  { id: "4", name: "DataGrid.tsx", kind: "file", sizeBytes: 15230, modified: "2026-07-16" },
  { id: "5", name: "DataGrid.css", kind: "file", sizeBytes: 4310, modified: "2026-07-16" },
  { id: "6", name: "README.md", kind: "file", sizeBytes: 1780, modified: "2026-06-28" },
  { id: "7", name: "tsconfig.json", kind: "file", sizeBytes: 512, modified: "2026-06-20" },
];

function formatSize(bytes: number, kind: FileRow["kind"]): string {
  if (kind === "folder") return "—";
  if (bytes < 1024) return `${bytes} B`;
  return `${(bytes / 1024).toFixed(1)} KB`;
}

const columns: DataGridColumn<FileRow>[] = [
  { key: "name", header: "Name", width: 220, sortable: true, sortValue: (r) => r.name },
  { key: "kind", header: "Type", width: 90, sortable: true, sortValue: (r) => r.kind },
  {
    key: "sizeBytes",
    header: "Size",
    width: 100,
    align: "right",
    sortable: true,
    sortValue: (r) => r.sizeBytes,
    summary: "sum",
    summaryValue: (r) => r.sizeBytes,
    formatSummary: (v) => `${(v / 1024).toFixed(1)} KB`,
    render: (r) => formatSize(r.sizeBytes, r.kind),
  },
  { key: "modified", header: "Modified", width: 120, sortable: true, sortValue: (r) => r.modified },
];

function Frame({ children }: { children: React.ReactNode }) {
  return (
    <div style={{
      background: "var(--sp-surface-0)", padding: "var(--sp-space-md)",
      borderRadius: "var(--sp-radius-md)", width: 640, height: 360,
      display: "flex", flexDirection: "column",
    }}>{children}</div>
  );
}

/** A sortable, filterable file listing with a size summary in the footer — DataGrid's baseline configuration. */
export function FileListing() {
  return (
    <Frame>
      <DataGrid columns={columns} rows={ROWS} getRowId={(r) => r.id} selectable defaultSort={[{ key: "name", direction: "asc" }]} />
    </Frame>
  );
}

/** Rows dragged into the group panel bucket by type, each group showing a count and its own size subtotal. */
export function GroupedByType() {
  return (
    <Frame>
      <DataGrid columns={columns} rows={ROWS} getRowId={(r) => r.id} defaultGroupBy={["kind"]} filterRow={false} />
    </Frame>
  );
}
