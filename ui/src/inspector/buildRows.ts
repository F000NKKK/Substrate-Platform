import type { DataValue } from "./dataValue";
import type { InspectorDiagnostic, InspectorRow } from "./inspectorRow";
import { RowBuilder } from "./rowBuilder";
import type { ShapeDescription } from "./shapeTypes";

export interface BuildRowsOptions {
  /** The root row's name (default `"value"`). */
  readonly rootName?: string;
  /** Problems the host found, by path; shown on their rows. */
  readonly diagnostics?: readonly InspectorDiagnostic[];
}

/** The row tree of `value` described by `description`: the root row and everything under it. */
export function buildRows(description: ShapeDescription, value: DataValue | undefined, options: BuildRowsOptions = {}): InspectorRow {
  const builder = new RowBuilder(description.definitions, options.diagnostics ?? []);
  const site = { path: [], name: options.rootName ?? "value", doc: null, range: null, required: true };
  return builder.row(site, description.root, value);
}
