import type { DebugFrame, DebugScope, DebugVariable } from "./debugState";

// The adapter's JSON read tolerantly: what it leaves out has a default, what
// isn't the shape asked for is dropped.

interface Json {
  [key: string]: unknown;
}

const asObjects = (value: unknown): Json[] => (Array.isArray(value) ? value.filter((v): v is Json => typeof v === "object" && v !== null) : []);
const num = (value: unknown, fallback: number) => (typeof value === "number" ? value : fallback);
const text = (value: unknown, fallback = "") => (typeof value === "string" ? value : fallback);

/** A `stackTrace` response's frames. */
export function toFrames(body: unknown): DebugFrame[] {
  return asObjects((body as Json | null)?.stackFrames).map((f) => {
    const path = (f.source as Json | undefined)?.path;
    return { id: num(f.id, 0), name: text(f.name), path: typeof path === "string" ? path : undefined, line: num(f.line, 0), column: num(f.column, 0) };
  });
}

/** A `scopes` response's scopes. */
export function toScopes(body: unknown): DebugScope[] {
  return asObjects((body as Json | null)?.scopes).map((s) => ({
    name: text(s.name),
    variablesReference: num(s.variablesReference, 0),
    expensive: s.expensive === true,
  }));
}

/** A `variables` response's variables. */
export function toVariables(body: unknown): DebugVariable[] {
  return asObjects((body as Json | null)?.variables).map((v) => ({
    name: text(v.name),
    value: text(v.value),
    type: typeof v.type === "string" ? v.type : undefined,
    variablesReference: num(v.variablesReference, 0),
  }));
}

/** The first thread of a `threads` response. */
export function firstThread(body: unknown): number | undefined {
  const id = asObjects((body as Json | null)?.threads)[0]?.id;
  return typeof id === "number" ? id : undefined;
}
