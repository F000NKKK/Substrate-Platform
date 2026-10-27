import type { InspectorError } from "./inspectorError";
import { fail, ok, type Result } from "./result";

export type JsonObject = { readonly [key: string]: unknown };
export type Read<T> = Result<T, InspectorError>;

/** How a JSON value is named in an error. */
export function describeJson(found: unknown): string {
  if (found === null) return "null";
  return Array.isArray(found) ? "array" : typeof found;
}

export function child(at: string, key: string | number): string {
  return `${at}/${String(key).replaceAll("~", "~0").replaceAll("/", "~1")}`;
}

function malformed(at: string, expected: string, found: unknown): InspectorError {
  return { kind: "malformed", at, expected, found: describeJson(found) };
}

export function readObject(at: string, json: unknown): Read<JsonObject> {
  const isObject = typeof json === "object" && json !== null && !Array.isArray(json);
  return isObject ? ok(json as JsonObject) : fail(malformed(at, "an object", json));
}

export function readArray(at: string, json: unknown): Read<readonly unknown[]> {
  return Array.isArray(json) ? ok(json) : fail(malformed(at, "an array", json));
}

export function readString(at: string, json: unknown): Read<string> {
  return typeof json === "string" ? ok(json) : fail(malformed(at, "a string", json));
}

export function readNumber(at: string, json: unknown): Read<number> {
  return typeof json === "number" ? ok(json) : fail(malformed(at, "a number", json));
}

export function readBoolean(at: string, json: unknown): Read<boolean> {
  return typeof json === "boolean" ? ok(json) : fail(malformed(at, "a boolean", json));
}

/** A string, or nothing (absent or `null`). */
export function readOptionalString(at: string, json: unknown): Read<string | null> {
  return json === undefined || json === null ? ok(null) : readString(at, json);
}
