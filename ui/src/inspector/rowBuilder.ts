import type { DataPath } from "./dataPath";
import { pathId } from "./dataPath";
import { isMap, valueKind, type DataValue } from "./dataValue";
import type { InspectorDiagnostic, InspectorRow, RowDisplay, RowEditor, RowProblem } from "./inspectorRow";
import type { Definition, FieldShape, ShapeNode, VariantShape } from "./shapeTypes";
import { typeLabel } from "./typeLabel";

/** Where a row sits and what its parent knows about it. */
export interface RowSite {
  readonly path: DataPath;
  readonly name: string;
  readonly doc: string | null;
  readonly range: readonly [number, number] | null;
  readonly required: boolean;
}

interface RowParts {
  readonly display: RowDisplay;
  readonly editor?: RowEditor;
  readonly value?: DataValue;
  readonly problem?: RowProblem;
  readonly children?: readonly InspectorRow[];
}

type ValueMap = { readonly [key: string]: DataValue };

/** Builds the rows of a value against its shape; nothing here throws, a value that does not fit becomes a row with a `problem`. */
export class RowBuilder {
  private readonly messages = new Map<string, string[]>();

  constructor(
    private readonly definitions: ReadonlyMap<string, Definition>,
    diagnostics: readonly InspectorDiagnostic[],
  ) {
    for (const { path, message } of diagnostics) {
      const id = pathId(path);
      this.messages.set(id, [...(this.messages.get(id) ?? []), message]);
    }
  }

  /** The row for `value` (`undefined` when absent) read as `shape`. */
  row(site: RowSite, shape: ShapeNode, value: DataValue | undefined): InspectorRow {
    let node = shape;
    let optional = false;
    while (node.kind === "named" || node.kind === "optional") {
      optional ||= node.kind === "optional";
      node = node.of;
    }
    const label = typeLabel(shape);
    if (value === undefined || (optional && value === null)) return this.absent(site, label, optional);
    return this.present(site, label, node, value);
  }

  private make(site: RowSite, label: string, parts: RowParts): InspectorRow {
    const id = pathId(site.path);
    const messages = this.messages.get(id) ?? [];
    const children = parts.children ?? null;
    const issueCount = messages.length + (children?.reduce((sum, child) => sum + child.issueCount, 0) ?? 0);
    return {
      id,
      path: site.path,
      name: site.name,
      typeLabel: label,
      doc: site.doc,
      display: parts.display,
      editor: parts.editor ?? null,
      value: parts.value,
      problem: parts.problem ?? null,
      messages,
      issueCount,
      children,
    };
  }

  private absent(site: RowSite, label: string, optional: boolean): InspectorRow {
    if (optional) return this.make(site, label, { display: { kind: "none" } });
    if (!site.required) return this.make(site, label, { display: { kind: "default" } });
    return this.make(site, label, { display: { kind: "missing" }, problem: { kind: "missing" } });
  }

  private present(site: RowSite, label: string, node: ShapeNode, value: DataValue): InspectorRow {
    switch (node.kind) {
      case "def":
        return this.definition(site, label, node.name, value);
      case "list":
        return this.list(site, label, node, value);
      case "map":
        return this.map(site, label, node, value);
      case "any":
        return this.generic(site, label, value, null);
      default:
        return this.scalar(site, label, node, value);
    }
  }

  private scalar(site: RowSite, label: string, node: ShapeNode, value: DataValue): InspectorRow {
    const display: RowDisplay = { kind: "scalar", value };
    const editor = scalarEditor(node, value, site);
    if (editor === undefined) return this.generic(site, label, value, { kind: "mismatch", expected: node.kind, found: valueKind(value) });
    return this.make(site, label, { display, value, ...(editor && { editor }) });
  }

  /** `value` as plain data, whatever it is, with the `problem` it caused (if any). */
  private generic(site: RowSite, label: string, value: DataValue, problem: RowProblem | null): InspectorRow {
    const kind = valueKind(value);
    const base = problem ? { problem } : {};
    if (kind === "list") {
      const items = (value as readonly DataValue[]).map((item, index) => this.generic(this.child(site, index, String(index)), "any", item, null));
      return this.make(site, label, { ...base, display: { kind: "list", length: items.length }, children: items });
    }
    if (isMap(value)) {
      const entries = Object.entries(value).map(([key, item]) => this.generic(this.child(site, key, key), "any", item, null));
      return this.make(site, label, { ...base, display: { kind: "map", length: entries.length }, children: entries });
    }
    return this.make(site, label, { ...base, display: { kind: "scalar", value } });
  }

  private child(site: RowSite, segment: string | number, name: string, field?: FieldShape): RowSite {
    return { path: [...site.path, segment], name, doc: field?.doc ?? null, range: field?.range ?? null, required: field?.required ?? true };
  }

  private list(site: RowSite, label: string, node: ShapeNode & { kind: "list" }, value: DataValue): InspectorRow {
    if (!Array.isArray(value)) return this.generic(site, label, value, { kind: "mismatch", expected: "list", found: valueKind(value) });
    const children = value.map((item, index) => this.row(this.child(site, index, String(index)), node.of, item));
    return this.make(site, label, { display: { kind: "list", length: children.length }, children });
  }

  private map(site: RowSite, label: string, node: ShapeNode & { kind: "map" }, value: DataValue): InspectorRow {
    if (!isMap(value)) return this.generic(site, label, value, { kind: "mismatch", expected: "map", found: valueKind(value) });
    const children = Object.entries(value).map(([key, item]) => this.row(this.child(site, key, key), node.of, item));
    return this.make(site, label, { display: { kind: "map", length: children.length }, children });
  }

  private fieldRows(site: RowSite, fields: readonly FieldShape[], value: ValueMap): InspectorRow[] {
    return fields.map((field) => this.row(this.child(site, field.name, field.name, field), field.shape, value[field.name]));
  }

  private definition(site: RowSite, label: string, name: string, value: DataValue): InspectorRow {
    const definition = this.definitions.get(name);
    if (!definition) return this.generic(site, label, value, { kind: "unknownDefinition", name });
    if (definition.kind === "enum") return this.enumeration(site, label, definition, value);
    if (!isMap(value)) return this.generic(site, label, value, { kind: "mismatch", expected: name, found: valueKind(value) });
    const children = this.fieldRows(site, definition.fields, value);
    return this.make(site, label, { display: { kind: "struct", name }, ...(children.length > 0 && { children }) });
  }

  private enumeration(site: RowSite, label: string, definition: Definition & { kind: "enum" }, value: DataValue): InspectorRow {
    const { tag, variants } = definition;
    const name = tag === null ? value : isMap(value) ? value[tag] : undefined;
    if (typeof name !== "string") return this.generic(site, label, value, { kind: "mismatch", expected: tag === null ? "string" : "map", found: valueKind(value) });
    const editor: RowEditor = { kind: "variant", options: switchable(variants, tag, name), tag };
    const variant = variants.find((candidate) => candidate.name === name);
    if (!variant) return this.make(site, label, { display: { kind: "variant", name }, editor, value: name, problem: { kind: "unknownVariant", name } });
    return this.make(site, label, { display: { kind: "variant", name }, editor, value: name, ...this.variantChildren(site, variant, value) });
  }

  private variantChildren(site: RowSite, variant: VariantShape, value: DataValue): { children?: readonly InspectorRow[] } {
    const children = variant.fields && isMap(value) ? this.fieldRows(site, variant.fields, value) : [];
    return children.length > 0 ? { children } : {};
  }
}

/** The editor for a scalar, `null` for one shown read-only, `undefined` when `value` is not that scalar. */
function scalarEditor(node: ShapeNode, value: DataValue, site: RowSite): RowEditor | null | undefined {
  switch (node.kind) {
    case "bool":
      return typeof value === "boolean" ? { kind: "boolean" } : undefined;
    case "int":
      return typeof value === "number" && Number.isInteger(value) ? { kind: "integer", min: node.min, max: node.max } : undefined;
    case "float":
      return typeof value === "number" ? { kind: "number", range: site.range } : undefined;
    case "string":
    case "atom":
      return typeof value === "string" ? { kind: "text" } : undefined;
    default:
      return typeof value === "string" ? null : undefined;
  }
}

/**
 * The variants an editor may switch to. A plain enum offers all; a tagged one
 * only the current variant and those with nothing required, since switching
 * keeps no other fields.
 */
function switchable(variants: readonly VariantShape[], tag: string | null, current: string): string[] {
  const allowed = (variant: VariantShape) => tag === null || variant.name === current || !(variant.fields ?? []).some((field) => field.required);
  return variants.filter(allowed).map((variant) => variant.name);
}
