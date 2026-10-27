/** A value in Substrate Data's JSON form (`substrate_data::json::to_json`). */
export type DataValue = null | boolean | number | string | readonly DataValue[] | { readonly [key: string]: DataValue };

/** What a value is, by its JSON type. */
export type DataValueKind = "null" | "boolean" | "number" | "string" | "list" | "map";

export function isDataValue(candidate: unknown): candidate is DataValue {
  if (candidate === null) return true;
  switch (typeof candidate) {
    case "boolean":
    case "number":
    case "string":
      return true;
    case "object":
      return Array.isArray(candidate) ? candidate.every(isDataValue) : Object.values(candidate).every(isDataValue);
    default:
      return false;
  }
}

export function valueKind(value: DataValue): DataValueKind {
  if (value === null) return "null";
  if (Array.isArray(value)) return "list";
  const type = typeof value;
  return type === "object" ? "map" : (type as "boolean" | "number" | "string");
}

export function isMap(value: DataValue): value is { readonly [key: string]: DataValue } {
  return valueKind(value) === "map";
}
