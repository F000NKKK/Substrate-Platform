import type { DataPath } from "./dataPath";
import type { DataValue, DataValueKind } from "./dataValue";

/** What the row shows as its value; the component turns it into text once. */
export type RowDisplay =
  | { readonly kind: "scalar"; readonly value: DataValue }
  | { readonly kind: "variant"; readonly name: string }
  | { readonly kind: "list"; readonly length: number }
  | { readonly kind: "map"; readonly length: number }
  | { readonly kind: "struct"; readonly name: string }
  | { readonly kind: "none" }
  | { readonly kind: "default" }
  | { readonly kind: "missing" };

/** Why a value does not fit its shape. */
export type RowProblem =
  | { readonly kind: "mismatch"; readonly expected: string; readonly found: DataValueKind }
  | { readonly kind: "unknownVariant"; readonly name: string }
  | { readonly kind: "unknownDefinition"; readonly name: string }
  | { readonly kind: "missing" };

/** How a scalar row is edited in place. */
export type RowEditor =
  | { readonly kind: "boolean" }
  | { readonly kind: "integer"; readonly min: number; readonly max: number }
  | { readonly kind: "number"; readonly range: readonly [number, number] | null }
  | { readonly kind: "text" }
  /** An enum's variant; `tag` is the key a tagged enum keeps its variant under, `null` for a plain enum (the value is the name). */
  | { readonly kind: "variant"; readonly options: readonly string[]; readonly tag: string | null };

/** A problem the host reports at a path (already in the user's language). */
export interface InspectorDiagnostic {
  readonly path: DataPath;
  readonly message: string;
}

export interface InspectorRow {
  /** Stable across rebuilds: `pathId(path)`. */
  readonly id: string;
  readonly path: DataPath;
  readonly name: string;
  /** The shape in short: `f32`, `[f32]`, `Transform`, `u8?`. */
  readonly typeLabel: string;
  readonly doc: string | null;
  readonly display: RowDisplay;
  readonly editor: RowEditor | null;
  /** The current value of an editable row. */
  readonly value: DataValue | undefined;
  readonly problem: RowProblem | null;
  readonly messages: readonly string[];
  /** Diagnostics on this row and under it. */
  readonly issueCount: number;
  /** `null` for a leaf; a branch, even an empty one, is expandable. */
  readonly children: readonly InspectorRow[] | null;
}
