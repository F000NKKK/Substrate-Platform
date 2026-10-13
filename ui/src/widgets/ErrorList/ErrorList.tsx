import { useMemo, useState } from "react";
import type { Diagnostic, DiagnosticSeverity } from "../../infra/diagnostics";
import { Icon } from "../../infra/icons";
import { tn, useT } from "../../infra/i18n";
import { useStoredState } from "../../infra/state";
import { ContextMenu, useContextMenu, type ContextMenuItem } from "../ContextMenu";
import { DataGrid, type DataGridColumn } from "../DataGrid";
import { SelectField } from "../Form";
import { SearchField } from "../SearchField";
import { ToolbarSeparator, ToolbarSpacer } from "../Toolbar";
import {
  DEFAULT_ERROR_LIST_FILTER,
  diagnosticsToTsv,
  filterDiagnostics,
  severityRank,
  type ErrorListFilter,
  type ErrorListOrigins,
  type ErrorListScope,
} from "./errorListFilter";
import "./ErrorList.css";

export interface ErrorListProps {
  diagnostics: readonly Diagnostic[];
  /** Double-click or Enter on a row with a file — open it at the line. */
  onNavigate: (diagnostic: Diagnostic) => void;
  /** The focused editor's file, for the "Current Document" scope. */
  activeFile?: string | null;
  /** Every open editor's file, for "Open Documents". */
  openFiles?: readonly string[];
  /** File column text; default the file name. The tooltip is always the full path. */
  formatPath?: (file: string) => string;
  /** Opens a `codeHref` (the Code column's link, "Show Error Help"); without it codes are plain text. */
  onOpenHelp?: (href: string, diagnostic: Diagnostic) => void;
  /** Prefix for the remembered filters and columns (the current `StateStore`, and the grid's own layout). Default `"errorList"`. */
  persistKey?: string;
}

const SEVERITY_ICON: Record<DiagnosticSeverity, string> = { error: "error", warning: "warning", message: "info" };
/** VS hides Column and Tool until asked for (right-click a header). */
const HIDDEN_BY_DEFAULT = new Set(["column", "tool"]);
const SEVERITIES: readonly DiagnosticSeverity[] = ["error", "warning", "message"];

const baseName = (path: string) => path.slice(Math.max(path.lastIndexOf("/"), path.lastIndexOf("\\")) + 1);

function isFilter(value: unknown): value is ErrorListFilter {
  const f = value as ErrorListFilter | null;
  return !!f && typeof f === "object" && typeof f.scope === "string" && typeof f.origins === "string" && !!f.shown && typeof f.shown === "object";
}

interface Row {
  d: Diagnostic;
  id: string;
}

/**
 * Visual Studio's Error List: a scope dropdown (Entire Solution / Open
 * Documents / Current Document), Errors / Warnings / Messages toggles with
 * their counts, a Build / live-analysis source filter and a search box over
 * a sortable grid — severity, code, description, project, file, line.
 * Double-click or Enter goes to the location. Updates as `diagnostics`
 * changes without taking focus; the filters persist.
 */
export function ErrorList({ diagnostics, onNavigate, activeFile, openFiles, formatPath = baseName, onOpenHelp, persistKey = "errorList" }: ErrorListProps) {
  const t = useT();
  const [filter, setFilter] = useStoredState<ErrorListFilter>(`${persistKey}.filter`, DEFAULT_ERROR_LIST_FILTER, isFilter);
  const [search, setSearch] = useState("");
  const [selectedIds, setSelectedIds] = useState<Set<string>>(new Set());
  const menu = useContextMenu<Row>();

  const { rows, counts } = useMemo(() => {
    const filtered = filterDiagnostics(diagnostics, filter, search, { activeFile, openFiles });
    return { rows: filtered.rows.map((i): Row => ({ d: diagnostics[i], id: String(i) })), counts: filtered.counts };
  }, [diagnostics, filter, search, activeFile, openFiles]);

  const severityLabel = (s: DiagnosticSeverity) => (s === "error" ? t("Error") : s === "warning" ? t("Warning") : t("Message"));

  const columns = useMemo<DataGridColumn<Row>[]>(
    () => [
      {
        key: "severity",
        header: "",
        width: 30,
        minWidth: 30,
        align: "center",
        sortable: true,
        hideable: false,
        lockGroup: true,
        sortValue: (r) => severityRank(r.d.severity),
        render: (r) => (
          <span className={`sp-error-list-severity sp-error-list-severity--${r.d.severity}`} title={severityLabel(r.d.severity)}>
            <Icon name={SEVERITY_ICON[r.d.severity]} />
          </span>
        ),
      },
      {
        key: "code",
        header: t("Code"),
        width: 110,
        sortable: true,
        sortValue: (r) => r.d.code ?? "",
        filterValue: (r) => r.d.code ?? "",
        render: (r) =>
          r.d.code && r.d.codeHref && onOpenHelp ? (
            <a
              className="sp-error-list-code"
              href={r.d.codeHref}
              title={r.d.codeHref}
              onClick={(e) => {
                e.preventDefault();
                onOpenHelp(r.d.codeHref!, r.d);
              }}
              onDoubleClick={(e) => e.stopPropagation()}
            >
              {r.d.code}
            </a>
          ) : (
            r.d.code
          ),
      },
      {
        key: "message",
        header: t("Description"),
        width: 520,
        sortable: true,
        hideable: false,
        sortValue: (r) => r.d.message,
        render: (r) => <span title={r.d.message}>{r.d.message}</span>,
      },
      { key: "project", header: t("Project"), width: 130, sortable: true, sortValue: (r) => r.d.project ?? "" },
      {
        key: "file",
        header: t("File"),
        width: 170,
        sortable: true,
        sortValue: (r) => (r.d.file ? formatPath(r.d.file) : ""),
        render: (r) => r.d.file && <span title={r.d.file}>{formatPath(r.d.file)}</span>,
      },
      { key: "line", header: t("Line"), width: 60, align: "right", sortable: true, sortValue: (r) => r.d.line ?? 0, render: (r) => r.d.line },
      { key: "column", header: t("Column"), width: 60, align: "right", sortable: true, sortValue: (r) => r.d.column ?? 0, render: (r) => r.d.column },
      { key: "tool", header: t("Tool"), width: 90, sortable: true, sortValue: (r) => r.d.tool ?? "" },
    ],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [t, formatPath, onOpenHelp]
  );

  const update = (patch: Partial<ErrorListFilter>) => setFilter((f) => ({ ...f, ...patch }));
  const toggleLabel = (s: DiagnosticSeverity, n: number) =>
    s === "error" ? tn(n, "{count} Error", "{count} Errors") : s === "warning" ? tn(n, "{count} Warning", "{count} Warnings") : tn(n, "{count} Message", "{count} Messages");

  function navigate(d: Diagnostic) {
    if (d.file) onNavigate(d);
  }

  function copy(items: readonly Diagnostic[]) {
    const header = [t("Severity"), t("Code"), t("Description"), t("Project"), t("File"), t("Line")];
    void navigator.clipboard?.writeText(diagnosticsToTsv(items, header, severityLabel));
  }

  const menuItems = (row: Row): ContextMenuItem[] => {
    const selected = rows.filter((r) => selectedIds.has(r.id)).map((r) => r.d);
    const items = selected.length > 0 ? selected : [row.d];
    const help = row.d.codeHref;
    const fixes = (row.d.fixes ?? []).map((fix): ContextMenuItem => ({ label: fix.label, onSelect: fix.apply }));
    return [
      ...fixes,
      { label: t("Copy"), shortcut: "Ctrl+C", separatorBefore: fixes.length > 0, onSelect: () => copy(items) },
      { label: t("Copy Description"), onSelect: () => void navigator.clipboard?.writeText(items.map((d) => d.message).join("\n")) },
      { label: t("Go To Location"), separatorBefore: true, disabled: !row.d.file, onSelect: () => navigate(row.d) },
      ...(help && onOpenHelp ? [{ label: t("Show Error Help"), onSelect: () => onOpenHelp(help, row.d) }] : []),
    ];
  };

  return (
    <div className="sp-error-list">
      <div className="sp-toolbar sp-error-list-toolbar">
        <SelectField
          aria-label={t("Scope")}
          value={filter.scope}
          onChange={(scope) => update({ scope: scope as ErrorListScope })}
          options={[
            { value: "solution", label: t("Entire Solution") },
            { value: "open", label: t("Open Documents") },
            { value: "current", label: t("Current Document") },
          ]}
        />
        <ToolbarSeparator />
        {SEVERITIES.map((s) => (
          <button
            key={s}
            type="button"
            className="sp-error-list-toggle"
            aria-pressed={filter.shown[s]}
            onClick={() => update({ shown: { ...filter.shown, [s]: !filter.shown[s] } })}
          >
            <span className={`sp-error-list-severity sp-error-list-severity--${s}`}>
              <Icon name={SEVERITY_ICON[s]} />
            </span>
            {toggleLabel(s, counts[s])}
          </button>
        ))}
        <ToolbarSeparator />
        <SelectField
          aria-label={t("Show items from")}
          value={filter.origins}
          onChange={(origins) => update({ origins: origins as ErrorListOrigins })}
          options={[
            { value: "all", label: t("Build + Live Analysis") },
            { value: "build", label: t("Build Only") },
            { value: "live", label: t("Live Analysis Only") },
          ]}
        />
        <ToolbarSpacer />
        <SearchField className="sp-error-list-search" value={search} onValueChange={setSearch} placeholder={t("Search Error List")} aria-label={t("Search Error List")} />
      </div>
      <DataGrid<Row>
        className="sp-error-list-grid"
        columns={columns}
        rows={rows}
        getRowId={(r) => r.id}
        selectable
        showSelectColumn={false}
        selectedIds={selectedIds}
        onSelectionChange={setSelectedIds}
        filterRow={false}
        showGroupPanel={false}
        defaultHiddenColumns={HIDDEN_BY_DEFAULT}
        persistKey={`${persistKey}.grid`}
        onRowActivate={(r) => navigate(r.d)}
        onRowContextMenu={(r, e) => menu.openAtPoint(r, e)}
        onGridKeyDown={(e) => {
          if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "c" && selectedIds.size > 0) {
            e.preventDefault();
            copy(rows.filter((r) => selectedIds.has(r.id)).map((r) => r.d));
          }
        }}
        rowClassName={(r) => (r.d.file ? undefined : "sp-error-list-row--nofile")}
        emptyState={diagnostics.length === 0 ? t("No problems found.") : t("No items match the current filters.")}
      />
      {menu.target && <ContextMenu target={menu.target} items={menuItems(menu.target.node)} onClose={menu.close} />}
    </div>
  );
}
