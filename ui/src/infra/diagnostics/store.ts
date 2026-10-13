import { useSyncExternalStore } from "react";
import type { Diagnostic, DiagnosticOrigin } from "./types";

/**
 * Every diagnostic the app knows about, in named sets — one per producer
 * and scope (`"build"`, `"live:/path/to/file"`), each replaced as a whole
 * when that producer runs again. Module-level like the Output buffers, so
 * nothing is lost while the Error List isn't mounted.
 */
const sets = new Map<string, readonly Diagnostic[]>();
const listeners = new Set<() => void>();
let snapshot: readonly Diagnostic[] = [];

function changed() {
  snapshot = [...sets.values()].flat();
  listeners.forEach((notify) => notify());
}

/** Replaces the set `key` (empty `items` removes it). */
export function setDiagnostics(key: string, items: readonly Diagnostic[]): void {
  if (items.length === 0) {
    if (!sets.delete(key)) return;
  } else {
    sets.set(key, items);
  }
  changed();
}

/** Appends to the set `key` — for a producer streaming diagnostics as it finds them. */
export function addDiagnostics(key: string, items: readonly Diagnostic[]): void {
  if (items.length === 0) return;
  sets.set(key, [...(sets.get(key) ?? []), ...items]);
  changed();
}

/** Drops every set whose key matches, or every set of one origin. */
export function clearDiagnostics(match: string | ((key: string) => boolean) | { origin: DiagnosticOrigin }): void {
  let any = false;
  for (const [key, items] of sets) {
    const hit =
      typeof match === "string" ? key === match : typeof match === "function" ? match(key) : items.some((d) => d.origin === match.origin);
    if (hit) any = sets.delete(key) || any;
  }
  if (any) changed();
}

/** All diagnostics, in set insertion order — the same array until something changes. */
export function getDiagnostics(): readonly Diagnostic[] {
  return snapshot;
}

export function subscribeDiagnostics(notify: () => void): () => void {
  listeners.add(notify);
  return () => {
    listeners.delete(notify);
  };
}

/** All diagnostics, re-rendering when any set changes. */
export function useDiagnostics(): readonly Diagnostic[] {
  return useSyncExternalStore(subscribeDiagnostics, getDiagnostics);
}
