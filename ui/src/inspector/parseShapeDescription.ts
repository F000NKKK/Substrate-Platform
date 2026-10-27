import { child, readObject, type Read } from "./jsonReader";
import { parseDefinition } from "./parseDefinition";
import { parseShapeNode } from "./parseShapeNode";
import { fail, ok } from "./result";
import type { Definition, ShapeDescription, ShapeNode } from "./shapeTypes";

/** The first `def` reference in `node` that names nothing in `definitions`. */
function missingDefinition(node: ShapeNode, definitions: ReadonlyMap<string, Definition>): string | null {
  switch (node.kind) {
    case "def":
      return definitions.has(node.name) ? null : node.name;
    case "list":
    case "map":
    case "optional":
    case "named":
      return missingDefinition(node.of, definitions);
    case "ref":
      return missingDefinition(node.id, definitions);
    default:
      return null;
  }
}

function missingInDefinition(definition: Definition, definitions: ReadonlyMap<string, Definition>): string | null {
  const fields = definition.kind === "struct" ? definition.fields : definition.variants.flatMap((variant) => variant.fields ?? []);
  for (const field of fields) {
    const missing = missingDefinition(field.shape, definitions);
    if (missing !== null) return missing;
  }
  return null;
}

function unresolved(root: ShapeNode, definitions: ReadonlyMap<string, Definition>): { at: string; name: string } | null {
  const fromRoot = missingDefinition(root, definitions);
  if (fromRoot !== null) return { at: "/root", name: fromRoot };
  for (const [name, definition] of definitions) {
    const missing = missingInDefinition(definition, definitions);
    if (missing !== null) return { at: child("/defs", name), name: missing };
  }
  return null;
}

/**
 * A shape description from its JSON (`{ root, defs }`), with every `def`
 * reference checked against the definitions: a bad description is an
 * `InspectorError`, never an exception.
 */
export function parseShapeDescription(json: unknown): Read<ShapeDescription> {
  const object = readObject("", json);
  if (!object.ok) return object;
  const root = parseShapeNode("/root", object.value.root);
  if (!root.ok) return root;
  const defsObject = readObject("/defs", object.value.defs ?? {});
  if (!defsObject.ok) return defsObject;
  const definitions = new Map<string, Definition>();
  for (const [name, raw] of Object.entries(defsObject.value)) {
    const definition = parseDefinition(child("/defs", name), raw);
    if (!definition.ok) return definition;
    definitions.set(name, definition.value);
  }
  const missing = unresolved(root.value, definitions);
  if (missing) return fail({ kind: "unknownDefinition", ...missing });
  return ok({ root: root.value, definitions });
}
