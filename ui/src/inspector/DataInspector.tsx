import { useMemo, useState } from "react";
import { useT } from "../infra/i18n";
import { Tree, type TreeNode } from "../widgets/Tree";
import type { DataPath } from "./dataPath";
import type { DataValue } from "./dataValue";
import { buildRows } from "./buildRows";
import { defaultExpansion, withIssuesOpen } from "./expansion";
import { describeInspectorError } from "./inspectorError";
import type { InspectorDiagnostic, InspectorRow } from "./inspectorRow";
import { InspectorRowLabel } from "./InspectorRowLabel";
import { parseShapeDescription } from "./parseShapeDescription";
import type { ShapeDescription } from "./shapeTypes";
import type { EditHandler } from "./ValueEditor";
import "./inspector.css";

export interface DataInspectorProps {
  /** The shape: a `ShapeDoc` as JSON (parsed here; a bad one is shown as an error) or an already parsed description. */
  shape: unknown;
  /** The value to show, in Substrate Data's JSON form. */
  value: DataValue | undefined;
  /** Called with the path and the new value of an edited scalar. Only called when `readOnly` is `false`. */
  onEdit?: (path: DataPath, value: DataValue) => void;
  /** Default `true`: no editors, whatever `onEdit` is. */
  readOnly?: boolean;
  /** Problems to show on their rows (messages already in the user's language). */
  diagnostics?: readonly InspectorDiagnostic[];
  /** The root row's name (default `value`). */
  rootName?: string;
  className?: string;
}

function isDescription(shape: unknown): shape is ShapeDescription {
  return typeof shape === "object" && shape !== null && (shape as { definitions?: unknown }).definitions instanceof Map;
}

function toNode(row: InspectorRow, onEdit: EditHandler | null): TreeNode<InspectorRow> {
  return {
    id: row.id,
    label: <InspectorRowLabel row={row} onEdit={onEdit} />,
    text: row.name,
    data: row,
    ...(row.children && { children: row.children.map((child) => toNode(child, onEdit)) }),
  };
}

/**
 * A read-only-first property tree of a value and the shape that describes it
 * (Substrate Data's `ShapeDoc` and JSON value). It knows nothing of where the
 * data comes from: the host supplies both and decides what an edit does.
 */
export function DataInspector({ shape, value, onEdit, readOnly = true, diagnostics, rootName, className }: DataInspectorProps) {
  const t = useT();
  const parsed = useMemo(() => (isDescription(shape) ? ({ ok: true, value: shape } as const) : parseShapeDescription(shape)), [shape]);
  const root = useMemo(
    () => (parsed.ok ? buildRows(parsed.value, value, { rootName: rootName ?? t("value"), diagnostics }) : null),
    [parsed, value, rootName, diagnostics, t],
  );
  const [expanded, setExpanded] = useState<Set<string> | null>(null);
  if (!parsed.ok) return <div className="sp-inspector-error" role="alert">{describeInspectorError(parsed.error)}</div>;
  if (!root) return null;
  const open = expanded ?? new Set(withIssuesOpen(root, defaultExpansion(root)));
  const handler: EditHandler | null = !readOnly && onEdit ? onEdit : null;
  return (
    <div className={["sp-inspector", className].filter(Boolean).join(" ")} aria-label={t("Data inspector")}>
      <Tree nodes={[toNode(root, handler)]} expandedIds={open} onExpandedChange={(ids) => setExpanded(new Set(ids))} />
    </div>
  );
}
