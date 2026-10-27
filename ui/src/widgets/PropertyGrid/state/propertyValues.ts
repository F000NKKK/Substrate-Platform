// The grid's view modes and rows, and a property's value as shown, drafted
// and committed.

import { t } from "../../../infra/i18n";
import { PROPERTY_KIND, PROPERTY_ORDER, type PropertyDescriptor, type PropertyGridProps, type PropertyKind, type PropertyValue } from "./propertyTypes";

/** The grid's view modes: by category, or every property by name. */
export const VIEW_MODE = { categorized: "categorized", alphabetical: "alphabetical" } as const;
export type ViewMode = (typeof VIEW_MODE)[keyof typeof VIEW_MODE];

/** A row of the grid as laid out for the current view mode and search — what keyboard navigation steps through. */
export type GridItem = { type: "category"; key: string; name: string; collapsed: boolean } | { type: "property"; key: string; property: PropertyDescriptor };

export const DEFAULT_CATEGORY = "Misc";
export const MIN_SPLIT = 0.15;
export const MAX_SPLIT = 0.85;
export const MIN_DESCRIPTION = 36;
export const MAX_DESCRIPTION = 320;
/** The name column's share of the width at first. */
export const DEFAULT_SPLIT = 0.4;
/** The description pane's height at first, in pixels. */
export const DEFAULT_DESCRIPTION = 64;
/** What a grid's remembered state is kept under unless `persistKey` says otherwise. */
export const DEFAULT_PERSIST_KEY = "properties";
/** A multi-line text box's height in lines: its text's, plus one, within these. */
export const MIN_TEXT_ROWS = 3;
export const MAX_TEXT_ROWS = 10;
/** Scrubbing a number: pixels per step, and the step's share with Shift held. */
export const SCRUB_PIXELS = 4;
export const SCRUB_FINE = 0.1;
/** The precision numbers keep (no float noise): 4 decimal places. */
const NUMBER_PRECISION = 10000;

export function kindOf(p: PropertyDescriptor): PropertyKind {
  if (p.kind) return p.kind;
  if (p.render) return PROPERTY_KIND.custom;
  if (Array.isArray(p.value)) return PROPERTY_KIND.vector;
  if (typeof p.value === "boolean") return PROPERTY_KIND.bool;
  if (typeof p.value === "number") return PROPERTY_KIND.number;
  return p.options ? PROPERTY_KIND.enum : PROPERTY_KIND.string;
}

export function isEditable(p: PropertyDescriptor): boolean {
  return !p.readOnly && !!p.onCommit;
}

export function displayText(p: PropertyDescriptor): string {
  if (p.displayValue !== undefined) return p.displayValue;
  if (p.value === null) return "";
  if (Array.isArray(p.value)) return p.value.join(", ");
  if (typeof p.value === "boolean") return p.value ? t("True") : t("False");
  if (p.optionLabel && kindOf(p) === PROPERTY_KIND.enum) return p.optionLabel(String(p.value));
  // A text shows its first line; the rest is there once it's edited.
  if (kindOf(p) === PROPERTY_KIND.text) {
    const lines = String(p.value).split("\n");
    return lines.length > 1 ? `${lines[0]} …` : lines[0];
  }
  return String(p.value);
}

export function draftOf(p: PropertyDescriptor): string {
  return p.value === null ? "" : String(p.value);
}

export const byName = (a: PropertyDescriptor, b: PropertyDescriptor) => a.name.localeCompare(b.name);

export const AXES = ["X", "Y", "Z", "W"];

export function sameValue(a: PropertyValue | null, b: PropertyValue | null): boolean {
  if (Array.isArray(a) && Array.isArray(b)) return a.length === b.length && a.every((v, i) => v === b[i]);
  return a === b;
}

/** `n` without float noise. */
export function roundNumber(n: number): number {
  return Math.round(n * NUMBER_PRECISION) / NUMBER_PRECISION;
}

/** A number as a field shows it: no float noise (0.30000000000000004 → 0.3). */
export function shortNumber(n: number): string {
  return String(roundNumber(n));
}

/** Runs `validate` then `onCommit` — resolves the rejection message, or `null` once applied. */
export async function applyEdit(p: PropertyDescriptor, value: PropertyValue): Promise<string | null> {
  const invalid = p.validate?.(value);
  if (invalid) return invalid;
  try {
    const result = await p.onCommit?.(value);
    return typeof result === "string" ? result : null;
  } catch (err) {
    return err instanceof Error ? err.message : String(err);
  }
}

/** A property row's key (its cursor position). */
export function propertyKey(p: PropertyDescriptor): string {
  return `p:${p.id}`;
}

function categoryKey(name: string): string {
  return `c:${name}`;
}

/** The rows the grid shows: `properties` matching `query`, by category (collapsed ones closed) or by name. */
export function layoutItems(properties: readonly PropertyDescriptor[], mode: ViewMode, query: string, collapsed: ReadonlySet<string>, order: PropertyGridProps["order"]): GridItem[] {
  const needle = query.trim().toLowerCase();
  const searching = needle !== "";
  const row = (property: PropertyDescriptor): GridItem => ({ type: "property", key: propertyKey(property), property });
  const shown = properties.filter((p) => !needle || p.name.toLowerCase().includes(needle));
  if (mode === VIEW_MODE.alphabetical) return [...shown].sort(byName).map(row);
  const groups = new Map<string, PropertyDescriptor[]>();
  for (const p of shown) {
    const category = p.category ?? DEFAULT_CATEGORY;
    groups.set(category, [...(groups.get(category) ?? []), p]);
  }
  const sorted = order === PROPERTY_ORDER.sorted;
  const names = [...groups.keys()];
  if (sorted) names.sort((a, b) => a.localeCompare(b));
  return names.flatMap((name): GridItem[] => {
    // While searching every match shows, collapsed or not.
    const isCollapsed = !searching && collapsed.has(name);
    const members = groups.get(name) ?? [];
    if (sorted) members.sort(byName);
    return [{ type: "category", key: categoryKey(name), name, collapsed: isCollapsed }, ...(isCollapsed ? [] : members.map(row))];
  });
}

/** A category's name as shown: the default one is "Misc", translated. */
export function categoryLabel(name: string): string {
  return name === DEFAULT_CATEGORY ? t("Misc") : name;
}
