import { child, readArray, readNumber, readObject, readString, type JsonObject, type Read } from "./jsonReader";
import { fail, ok } from "./result";
import type { ShapeNode } from "./shapeTypes";

const PLAIN_KINDS = ["any", "bool", "string", "atom", "bytes"] as const;

/** A shape node from its JSON (`{ kind: "list", of: … }`). */
export function parseShapeNode(at: string, json: unknown): Read<ShapeNode> {
  const object = readObject(at, json);
  if (!object.ok) return object;
  const kind = readString(child(at, "kind"), object.value.kind);
  if (!kind.ok) return kind;
  return parseByKind(at, kind.value, object.value);
}

function parseByKind(at: string, kind: string, object: JsonObject): Read<ShapeNode> {
  switch (kind) {
    case "int":
      return parseInt(at, object);
    case "float": {
      const bits = readNumber(child(at, "bits"), object.bits);
      return bits.ok ? ok({ kind, bits: bits.value }) : bits;
    }
    case "list":
    case "map":
    case "optional": {
      const of = parseShapeNode(child(at, "of"), object.of);
      return of.ok ? ok({ kind, of: of.value }) : of;
    }
    case "def": {
      const name = readString(child(at, "name"), object.name);
      return name.ok ? ok({ kind, name: name.value }) : name;
    }
    case "named":
      return parseNamed(at, object);
    case "ref":
      return parseRef(at, object);
    default:
      return (PLAIN_KINDS as readonly string[]).includes(kind) ? ok({ kind } as ShapeNode) : fail({ kind: "unknownShapeKind", at, found: kind });
  }
}

function parseInt(at: string, object: JsonObject): Read<ShapeNode> {
  const min = readNumber(child(at, "min"), object.min);
  if (!min.ok) return min;
  const max = readNumber(child(at, "max"), object.max);
  return max.ok ? ok({ kind: "int", min: min.value, max: max.value }) : max;
}

function parseNamed(at: string, object: JsonObject): Read<ShapeNode> {
  const name = readString(child(at, "name"), object.name);
  if (!name.ok) return name;
  const of = parseShapeNode(child(at, "of"), object.of);
  return of.ok ? ok({ kind: "named", name: name.value, of: of.value }) : of;
}

function parseRef(at: string, object: JsonObject): Read<ShapeNode> {
  const kinds = readArray(child(at, "of"), object.of);
  if (!kinds.ok) return kinds;
  const names: string[] = [];
  for (const [index, entry] of kinds.value.entries()) {
    const name = readString(child(child(at, "of"), index), entry);
    if (!name.ok) return name;
    names.push(name.value);
  }
  const id = parseShapeNode(child(at, "id"), object.id);
  return id.ok ? ok({ kind: "ref", of: names, id: id.value }) : id;
}
