// What a Properties grid shows: a property's kind, value and descriptor, and
// the grid's props.

import type { ReactNode } from "react";

export type PropertyKind = "string" | "text" | "number" | "bool" | "enum" | "vector" | "custom";
/**
 * A vector (`"vector"`) is a tuple of numbers — a position, a size, a turn;
 * a `"text"` is a string of several lines (a page's text, a description).
 */
export type PropertyValue = string | number | boolean | readonly number[];

/**
 * One row of a `PropertyGrid` — what a product's property provider builds
 * for the selected object. A property without `onCommit` (or with
 * `readOnly`) is shown greyed and can't be edited.
 */
export interface PropertyDescriptor {
  /** Unique within one object's properties — also what keeps the selected row selected when the object changes (as in Visual Studio, selecting another file keeps "Full Path" selected). */
  id: string;
  name: string;
  /** Grouping in the Categorized view — default "Misc". */
  category?: string;
  /** Shown in the description pane while the property is selected. */
  description?: string;
  /** `null` = not known (shown empty). */
  value: PropertyValue | null;
  /** Text shown instead of the raw value when not editing (e.g. `"1.2 KB (1,234 bytes)"`); editing still starts from `value`. */
  displayValue?: string;
  /** A CSS color drawn as a small swatch before the value (Visual Studio's Color properties) — edited as text. */
  swatch?: string;
  /** An image URL drawn as a small thumbnail before the value (pixelated — icons, textures). */
  image?: string;
  /** How it's edited — default inferred from `value`'s type (`"enum"` when `options` is given). */
  kind?: PropertyKind;
  /** The choices of an `"enum"` property. */
  options?: readonly string[];
  /** How an `"enum"` choice is shown (`"text"` → "Text") — default the choice itself. */
  optionLabel?: (option: string) => string;
  /** Values offered while editing a `"string"` property (an editable combo box) — any other text is still accepted. */
  suggestions?: readonly string[];
  readOnly?: boolean;
  /** Synchronous check before `onCommit` — return an error message to reject the edit. */
  validate?: (value: PropertyValue) => string | null;
  /**
   * Applies an edit. Returning (or resolving to) a string, or throwing,
   * rejects it with that message — shown under the row, the editor kept
   * open. The grid doesn't hold the new value itself: the provider
   * re-renders with an updated `value`.
   */
  onCommit?: (value: PropertyValue) => void | string | null | Promise<void | string | null>;
  /** Visual Studio's "…" button at the end of the row: opens the product's own editor for the value (a collection editor, a picker dialog). */
  edit?: () => void;
  /** A `"number"` or `"vector"`: what one step is — dragging the name left/right (or a component's label) changes it by steps, as Blender's fields do. Default 1. */
  step?: number;
  /** A `"vector"`'s component labels — default `X`, `Y`, `Z`, `W` (colored as the axes). */
  labels?: readonly string[];
  /** A `"custom"` row: what it shows across the whole width (a preview, a picker) — it isn't edited as a value. */
  render?: () => ReactNode;
}

export interface PropertyGridProps {
  /** Bold part of the object selector line (e.g. `main.rs`) — omit when nothing is selected, which shows `emptyLabel` instead. */
  objectName?: string;
  /** Rest of the object selector line (e.g. `File Properties`). */
  objectType?: string;
  /** What the object is — the selector line's tooltip, and the description pane's text while no row is selected. */
  objectDescription?: string;
  /** Identifies the shown object — an in-progress edit or error is dropped when it changes. Defaults to `objectName`. */
  objectKey?: string;
  properties: PropertyDescriptor[];
  emptyLabel?: string;
  /** Prefix for the view mode, column split, collapsed categories and description pane state remembered in the current `StateStore`. */
  persistKey?: string;
  className?: string;
  /**
   * `"sorted"` (Visual Studio's Properties window): categories and
   * properties by name. `"declared"` (an editor's inspector): as given —
   * position before rotation before scale. The Alphabetical view sorts
   * either way.
   */
  order?: "sorted" | "declared";
  /** Hides the toolbar (view mode, search) — an inspector inside an editor, short lists the author ordered. */
  compact?: boolean;
}
