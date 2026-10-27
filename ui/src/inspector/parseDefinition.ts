import { child, readArray, readBoolean, readObject, readOptionalString, readString, type JsonObject, type Read } from "./jsonReader";
import { parseShapeNode } from "./parseShapeNode";
import { fail, ok } from "./result";
import type { Definition, FieldShape, VariantShape } from "./shapeTypes";

/** Every element of a JSON array, parsed; the first failure stops it. */
function parseEach<T>(at: string, json: unknown, parseOne: (at: string, json: unknown) => Read<T>): Read<T[]> {
  const array = readArray(at, json);
  if (!array.ok) return array;
  const parsed: T[] = [];
  for (const [index, entry] of array.value.entries()) {
    const one = parseOne(child(at, index), entry);
    if (!one.ok) return one;
    parsed.push(one.value);
  }
  return ok(parsed);
}

function parseRange(at: string, json: unknown): Read<readonly [number, number] | null> {
  if (json === undefined || json === null) return ok(null);
  const pair = readArray(at, json);
  if (!pair.ok) return pair;
  const [low, high] = pair.value;
  if (typeof low !== "number" || typeof high !== "number" || pair.value.length !== 2) {
    return fail({ kind: "malformed", at, expected: "[low, high]", found: "array" });
  }
  return ok([low, high]);
}

function parseField(at: string, json: unknown): Read<FieldShape> {
  const object = readObject(at, json);
  if (!object.ok) return object;
  const name = readString(child(at, "name"), object.value.name);
  if (!name.ok) return name;
  const doc = readOptionalString(child(at, "doc"), object.value.doc);
  if (!doc.ok) return doc;
  const shape = parseShapeNode(child(at, "shape"), object.value.shape);
  if (!shape.ok) return shape;
  const required = readBoolean(child(at, "required"), object.value.required);
  if (!required.ok) return required;
  const range = parseRange(child(at, "range"), object.value.range);
  return range.ok ? ok({ name: name.value, doc: doc.value, shape: shape.value, required: required.value, range: range.value }) : range;
}

function parseVariant(at: string, json: unknown): Read<VariantShape> {
  const object = readObject(at, json);
  if (!object.ok) return object;
  const name = readString(child(at, "name"), object.value.name);
  if (!name.ok) return name;
  const doc = readOptionalString(child(at, "doc"), object.value.doc);
  if (!doc.ok) return doc;
  if (object.value.fields === undefined || object.value.fields === null) return ok({ name: name.value, doc: doc.value, fields: null });
  const fields = parseEach(child(at, "fields"), object.value.fields, parseField);
  return fields.ok ? ok({ name: name.value, doc: doc.value, fields: fields.value }) : fields;
}

function parseEnum(at: string, object: JsonObject, doc: string | null): Read<Definition> {
  const tag = readOptionalString(child(at, "tag"), object.tag);
  if (!tag.ok) return tag;
  const variants = parseEach(child(at, "variants"), object.variants, parseVariant);
  return variants.ok ? ok({ kind: "enum", doc, tag: tag.value, variants: variants.value }) : variants;
}

/** A struct's or an enum's definition from its JSON. */
export function parseDefinition(at: string, json: unknown): Read<Definition> {
  const object = readObject(at, json);
  if (!object.ok) return object;
  const kind = readString(child(at, "kind"), object.value.kind);
  if (!kind.ok) return kind;
  const doc = readOptionalString(child(at, "doc"), object.value.doc);
  if (!doc.ok) return doc;
  if (kind.value === "enum") return parseEnum(at, object.value, doc.value);
  if (kind.value !== "struct") return fail({ kind: "unknownDefinitionKind", at: child(at, "kind"), found: kind.value });
  const fields = parseEach(child(at, "fields"), object.value.fields, parseField);
  return fields.ok ? ok({ kind: "struct", doc: doc.value, fields: fields.value }) : fields;
}
