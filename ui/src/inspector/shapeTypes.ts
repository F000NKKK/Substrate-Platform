/** The description of a shape, as `substrate_data::schema::ShapeDoc` writes it in JSON. */

/** One shape, written `{ kind: "…", … }`. */
export type ShapeNode =
  | { readonly kind: "any" }
  | { readonly kind: "bool" }
  | { readonly kind: "int"; readonly min: number; readonly max: number }
  | { readonly kind: "float"; readonly bits: number }
  | { readonly kind: "string" }
  | { readonly kind: "atom" }
  | { readonly kind: "bytes" }
  | { readonly kind: "ref"; readonly of: readonly string[]; readonly id: ShapeNode }
  | { readonly kind: "named"; readonly name: string; readonly of: ShapeNode }
  | { readonly kind: "list"; readonly of: ShapeNode }
  | { readonly kind: "map"; readonly of: ShapeNode }
  | { readonly kind: "optional"; readonly of: ShapeNode }
  | { readonly kind: "def"; readonly name: string };

export type ShapeKind = ShapeNode["kind"];

export interface FieldShape {
  readonly name: string;
  readonly doc: string | null;
  readonly shape: ShapeNode;
  readonly required: boolean;
  /** `[low, high]` the value should stay within, when the field says so. */
  readonly range: readonly [number, number] | null;
}

export interface VariantShape {
  readonly name: string;
  readonly doc: string | null;
  /** The variant's fields; `null` for an enum without a tag, whose variants carry nothing. */
  readonly fields: readonly FieldShape[] | null;
}

export type Definition =
  | { readonly kind: "struct"; readonly doc: string | null; readonly fields: readonly FieldShape[] }
  | { readonly kind: "enum"; readonly doc: string | null; readonly tag: string | null; readonly variants: readonly VariantShape[] };

/** A whole shape: its root and the structs and enums it names, each defined once. */
export interface ShapeDescription {
  readonly root: ShapeNode;
  readonly definitions: ReadonlyMap<string, Definition>;
}
