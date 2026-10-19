// The grid's view modes and rows, and a property's value as shown, drafted
// and committed.

import { t } from "../../infra/i18n";
import type { PropertyDescriptor, PropertyKind, PropertyValue } from "./propertyTypes";

export type ViewMode = "categorized" | "alphabetical";

/** A row of the grid as laid out for the current view mode and search — what keyboard navigation steps through. */
export type GridItem = { type: "category"; key: string; name: string; collapsed: boolean } | { type: "property"; key: string; property: PropertyDescriptor };

export const DEFAULT_CATEGORY = "Misc";
export const MIN_SPLIT = 0.15;
export const MAX_SPLIT = 0.85;
export const MIN_DESCRIPTION = 36;
export const MAX_DESCRIPTION = 320;

export function kindOf(p: PropertyDescriptor): PropertyKind {
  if (p.kind) return p.kind;
  if (p.render) return "custom";
  if (Array.isArray(p.value)) return "vector";
  if (typeof p.value === "boolean") return "bool";
  if (typeof p.value === "number") return "number";
  return p.options ? "enum" : "string";
}

export function isEditable(p: PropertyDescriptor): boolean {
  return !p.readOnly && !!p.onCommit;
}

export function displayText(p: PropertyDescriptor): string {
  if (p.displayValue !== undefined) return p.displayValue;
  if (p.value === null) return "";
  if (Array.isArray(p.value)) return p.value.join(", ");
  if (typeof p.value === "boolean") return p.value ? t("True") : t("False");
  if (p.optionLabel && kindOf(p) === "enum") return p.optionLabel(String(p.value));
  // A text shows its first line; the rest is there once it's edited.
  if (kindOf(p) === "text") {
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

/** A number as a field shows it: no float noise (0.30000000000000004 → 0.3). */
export function shortNumber(n: number): string {
  return String(Math.round(n * 10000) / 10000);
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

/**
 * Visual Studio's Properties window body: an object selector line (bold
 * name + type), a Categorized/Alphabetical toggle and search box, a
 * two-column name/value grid with collapsible categories and a draggable
 * column splitter, and a resizable, hideable description pane for the
 * selected property. Purely presentational — what's shown comes from
 * `properties` (a product typically builds them from the global
 * `useSelection()`), and edits go back through each property's own
 * `onCommit`.
 *
 * Editing follows VS: a click on an editable value (or Enter/F2) opens a
 * text box that commits on Enter or blur and cancels on Esc; booleans and
 * enums are dropdowns that commit on pick, and double-clicking one cycles
 * its value. A rejected edit keeps the text box open with the message
 * under the row (one rejected by leaving the box is reported as a
 * notification instead, since the box is gone).
 */
