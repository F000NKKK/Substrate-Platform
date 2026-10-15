import { useMemo, useRef, useState, type KeyboardEvent, type PointerEvent as ReactPointerEvent, type ReactNode } from "react";
import { Icon } from "../../infra/icons";
import { useStoredState } from "../../infra/state";
import { IconButton } from "../IconButton";
import { TextField } from "../TextField";
import { notify } from "../Notifications";
import { t, useT } from "../../infra/i18n";
import "./PropertyGrid.css";

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
  /** A `"vector"`'s component labels — default `X`, `Y`, `Z`, `W` (coloured as the axes). */
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

type ViewMode = "categorized" | "alphabetical";

/** A row of the grid as laid out for the current view mode and search — what keyboard navigation steps through. */
type GridItem = { type: "category"; key: string; name: string; collapsed: boolean } | { type: "property"; key: string; property: PropertyDescriptor };

const DEFAULT_CATEGORY = "Misc";
const MIN_SPLIT = 0.15;
const MAX_SPLIT = 0.85;
const MIN_DESCRIPTION = 36;
const MAX_DESCRIPTION = 320;

function kindOf(p: PropertyDescriptor): PropertyKind {
  if (p.kind) return p.kind;
  if (p.render) return "custom";
  if (Array.isArray(p.value)) return "vector";
  if (typeof p.value === "boolean") return "bool";
  if (typeof p.value === "number") return "number";
  return p.options ? "enum" : "string";
}

function isEditable(p: PropertyDescriptor): boolean {
  return !p.readOnly && !!p.onCommit;
}

function displayText(p: PropertyDescriptor): string {
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

function draftOf(p: PropertyDescriptor): string {
  return p.value === null ? "" : String(p.value);
}

const byName = (a: PropertyDescriptor, b: PropertyDescriptor) => a.name.localeCompare(b.name);

const AXES = ["X", "Y", "Z", "W"];

function sameValue(a: PropertyValue | null, b: PropertyValue | null): boolean {
  if (Array.isArray(a) && Array.isArray(b)) return a.length === b.length && a.every((v, i) => v === b[i]);
  return a === b;
}

/** A number as a field shows it: no float noise (0.30000000000000004 → 0.3). */
function shortNumber(n: number): string {
  return String(Math.round(n * 10000) / 10000);
}

/** Runs `validate` then `onCommit` — resolves the rejection message, or `null` once applied. */
async function applyEdit(p: PropertyDescriptor, value: PropertyValue): Promise<string | null> {
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
export function PropertyGrid({
  objectName,
  objectType,
  objectDescription,
  objectKey,
  properties,
  emptyLabel,
  persistKey = "properties",
  className,
  order = "sorted",
  compact = false,
}: PropertyGridProps) {
  const t = useT();
  const [mode, setMode] = useStoredState<ViewMode>(`${persistKey}.view`, "categorized", (v) => v === "categorized" || v === "alphabetical");
  const [split, setSplit] = useStoredState<number>(`${persistKey}.split`, 0.4, (v) => typeof v === "number" && v >= MIN_SPLIT && v <= MAX_SPLIT);
  const [collapsedList, setCollapsedList] = useStoredState<string[]>(`${persistKey}.collapsed`, [], Array.isArray);
  const [descriptionVisible, setDescriptionVisible] = useStoredState<boolean>(`${persistKey}.description`, true, (v) => typeof v === "boolean");
  const [descriptionHeight, setDescriptionHeight] = useStoredState<number>(`${persistKey}.descriptionHeight`, 64, (v) => typeof v === "number");
  const [query, setQuery] = useState("");
  const [cursor, setCursor] = useState<string | null>(null);
  const [editing, setEditing] = useState<{ id: string; draft: string; session: number } | null>(null);
  const [error, setError] = useState<{ id: string; message: string } | null>(null);
  const [busyId, setBusyId] = useState<string | null>(null);
  const rowsRef = useRef<HTMLDivElement>(null);
  // Which text-box session is live — a blur arriving after Enter/Esc
  // already ended it (or after the object changed) must not commit again.
  const sessionRef = useRef(0);

  // A different object: drop the old one's edit and error (adjusting state
  // during render, as `useStoredState` does, so the new object never
  // renders a frame with the old one's text box).
  const shownKey = objectKey ?? objectName ?? "";
  const [lastKey, setLastKey] = useState(shownKey);
  if (lastKey !== shownKey) {
    setLastKey(shownKey);
    setEditing(null);
    setError(null);
    sessionRef.current++;
  }

  const collapsed = useMemo(() => new Set(collapsedList), [collapsedList]);
  const searching = query.trim() !== "";

  const items = useMemo((): GridItem[] => {
    const needle = query.trim().toLowerCase();
    const shown = properties.filter((p) => !needle || p.name.toLowerCase().includes(needle));
    if (mode === "alphabetical") return [...shown].sort(byName).map((p) => ({ type: "property", key: `p:${p.id}`, property: p }));
    const groups = new Map<string, PropertyDescriptor[]>();
    for (const p of shown) {
      const category = p.category ?? DEFAULT_CATEGORY;
      groups.set(category, [...(groups.get(category) ?? []), p]);
    }
    const out: GridItem[] = [];
    const names = [...groups.keys()];
    if (order === "sorted") names.sort((a, b) => a.localeCompare(b));
    for (const name of names) {
      // While searching every match shows, collapsed or not.
      const isCollapsed = !searching && collapsed.has(name);
      out.push({ type: "category", key: `c:${name}`, name, collapsed: isCollapsed });
      const members = groups.get(name)!;
      if (order === "sorted") members.sort(byName);
      if (!isCollapsed) for (const p of members) out.push({ type: "property", key: `p:${p.id}`, property: p });
    }
    return out;
  }, [properties, mode, query, searching, collapsed, order]);

  const current = items.find((item) => item.key === cursor) ?? null;

  function toggleCategory(name: string) {
    setCollapsedList((prev) => (prev.includes(name) ? prev.filter((c) => c !== name) : [...prev, name]));
  }

  function focusRows() {
    rowsRef.current?.focus();
  }

  function beginEdit(p: PropertyDescriptor) {
    setCursor(`p:${p.id}`);
    if (!isEditable(p) || busyId === p.id) return;
    const kind = kindOf(p);
    if (kind === "custom") return;
    if (kind === "bool" || kind === "enum" || kind === "vector") {
      // Always-rendered dropdowns and fields — focusing one is "editing" it.
      rowsRef.current?.querySelector<HTMLElement>(`[data-property-id="${CSS.escape(p.id)}"] select, [data-property-id="${CSS.escape(p.id)}"] input`)?.focus();
      return;
    }
    setEditing({ id: p.id, draft: draftOf(p), session: ++sessionRef.current });
  }

  function cancelEdit() {
    sessionRef.current++;
    setEditing(null);
    setError(null);
    focusRows();
  }

  /** Commits `value` — resolves whether it was accepted (a rejection's message is shown by the caller's choice: inline, or `notify` on blur). */
  async function commit(p: PropertyDescriptor, value: PropertyValue): Promise<string | null> {
    if (sameValue(value, p.value)) return null;
    setBusyId(p.id);
    const message = await applyEdit(p, value);
    setBusyId(null);
    return message;
  }

  function parseDraft(p: PropertyDescriptor, draft: string): PropertyValue | { error: string } {
    if (kindOf(p) !== "number") return draft;
    const n = Number(draft.trim());
    return draft.trim() === "" || Number.isNaN(n) ? { error: t("Not a valid number.") } : n;
  }

  async function commitText(p: PropertyDescriptor, draft: string, viaBlur: boolean) {
    const session = sessionRef.current;
    const parsed = parseDraft(p, draft);
    const message = typeof parsed === "object" && !Array.isArray(parsed) ? (parsed as { error: string }).error : await commit(p, parsed as PropertyValue);
    if (session !== sessionRef.current) return;
    if (!message) {
      sessionRef.current++;
      setEditing(null);
      setError(null);
      if (!viaBlur) focusRows();
    } else if (viaBlur) {
      // The text box is going away — report it where it stays visible.
      sessionRef.current++;
      setEditing(null);
      setError(null);
      notify({ level: "error", message: `${p.name}: ${message}` });
    } else {
      setError({ id: p.id, message });
    }
  }

  async function commitChoice(p: PropertyDescriptor, value: PropertyValue) {
    const message = await commit(p, value);
    setError(message ? { id: p.id, message } : null);
  }

  /** Double-click on a boolean/enum value steps to the next choice, as in Visual Studio. */
  function cycle(p: PropertyDescriptor) {
    if (!isEditable(p) || busyId === p.id) return;
    const kind = kindOf(p);
    if (kind === "bool") void commitChoice(p, !p.value);
    else if (kind === "enum" && p.options?.length) {
      const next = p.options[(p.options.indexOf(String(p.value)) + 1) % p.options.length];
      void commitChoice(p, next);
    }
  }

  function handleKeyDown(e: KeyboardEvent<HTMLDivElement>) {
    // Keys typed into a text box or dropdown are theirs.
    if (e.target !== e.currentTarget) return;
    const index = current ? items.indexOf(current) : -1;
    const move = (to: number) => {
      const target = items[Math.max(0, Math.min(items.length - 1, to))];
      if (target) {
        setCursor(target.key);
        rowsRef.current?.querySelector(`[data-key="${CSS.escape(target.key)}"]`)?.scrollIntoView({ block: "nearest" });
      }
    };
    switch (e.key) {
      case "ArrowDown":
        move(index + 1);
        break;
      case "ArrowUp":
        move(index < 0 ? 0 : index - 1);
        break;
      case "Home":
        move(0);
        break;
      case "End":
        move(items.length - 1);
        break;
      case "ArrowLeft":
      case "ArrowRight":
        if (current?.type !== "category" || searching) return;
        if (current.collapsed === (e.key === "ArrowLeft")) return;
        toggleCategory(current.name);
        break;
      case "Enter":
      case "F2":
        if (current?.type === "category") toggleCategory(current.name);
        else if (current) beginEdit(current.property);
        break;
      default:
        return;
    }
    e.preventDefault();
  }

  /** Drags the name/value splitter — the split is a fraction of the grid's width, so it survives resizing the tool window. */
  function startSplitDrag(e: ReactPointerEvent<HTMLSpanElement>) {
    const rows = rowsRef.current;
    if (!rows || e.button !== 0) return;
    e.preventDefault();
    e.stopPropagation();
    const target = e.currentTarget;
    target.setPointerCapture(e.pointerId);
    const left = rows.getBoundingClientRect().left;
    const width = rows.clientWidth;
    const onMove = (ev: PointerEvent) => setSplit(Math.max(MIN_SPLIT, Math.min(MAX_SPLIT, (ev.clientX - left) / width)));
    const onUp = () => {
      target.removeEventListener("pointermove", onMove);
      target.removeEventListener("pointerup", onUp);
      target.removeEventListener("pointercancel", onUp);
    };
    target.addEventListener("pointermove", onMove);
    target.addEventListener("pointerup", onUp);
    target.addEventListener("pointercancel", onUp);
  }

  /**
   * Drags a number (or one component of a vector) by its label, as Blender
   * does: a step per few pixels, Shift for a tenth of it; committed once,
   * on release.
   */
  function startScrub(e: ReactPointerEvent<HTMLElement>, p: PropertyDescriptor, component: number | null) {
    if (e.button !== 0 || !isEditable(p) || busyId === p.id) return;
    const base = component === null ? (typeof p.value === "number" ? p.value : null) : Array.isArray(p.value) ? p.value[component] : null;
    if (base === null || base === undefined) return;
    e.preventDefault();
    const target = e.currentTarget;
    target.setPointerCapture(e.pointerId);
    const startX = e.clientX;
    const step = p.step ?? 1;
    let latest = base;
    const valueAt = (n: number): PropertyValue => {
      if (component === null) return n;
      const v = [...(p.value as readonly number[])];
      v[component] = n;
      return v;
    };
    const onMove = (ev: PointerEvent) => {
      const steps = Math.round((ev.clientX - startX) / 4);
      latest = Math.round((base + steps * step * (ev.shiftKey ? 0.1 : 1)) * 10000) / 10000;
      const out = rowsRef.current?.querySelector<HTMLElement>(`[data-property-id="${CSS.escape(p.id)}"] [data-scrub="${component ?? "value"}"]`);
      if (out) {
        if (out instanceof HTMLInputElement) out.value = shortNumber(latest);
        else out.textContent = shortNumber(latest);
      }
    };
    const onUp = () => {
      target.removeEventListener("pointermove", onMove);
      target.removeEventListener("pointerup", onUp);
      target.removeEventListener("pointercancel", onUp);
      if (latest !== base) void commitChoice(p, valueAt(latest));
    };
    target.addEventListener("pointermove", onMove);
    target.addEventListener("pointerup", onUp);
    target.addEventListener("pointercancel", onUp);
  }

  /** A vector's fields: each component its own box, committed on Enter or leaving it. */
  function renderVector(p: PropertyDescriptor) {
    const v = Array.isArray(p.value) ? (p.value as readonly number[]) : [];
    const labels = p.labels ?? AXES;
    const editable = isEditable(p);
    const commitAt = (i: number, text: string) => {
      const n = Number(text.trim());
      if (text.trim() === "" || Number.isNaN(n)) {
        setError({ id: p.id, message: t("Not a valid number.") });
        return;
      }
      const next = [...v];
      next[i] = n;
      void commitChoice(p, next);
    };
    return (
      <span className="sp-propgrid-vector">
        {v.map((n, i) => (
          <label key={i} className="sp-propgrid-vector-part" data-axis={i}>
            <span className="sp-propgrid-vector-label" onPointerDown={(e) => startScrub(e, p, i)}>
              {labels[i] ?? ""}
            </span>
            <input
              key={`${p.id}:${n}`}
              className="sp-propgrid-vector-input"
              data-scrub={i}
              defaultValue={shortNumber(n)}
              disabled={!editable || busyId === p.id}
              spellCheck={false}
              inputMode="decimal"
              onFocus={(e) => {
                setCursor(`p:${p.id}`);
                e.currentTarget.select();
              }}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  e.preventDefault();
                  commitAt(i, e.currentTarget.value);
                } else if (e.key === "Escape") {
                  e.preventDefault();
                  e.currentTarget.value = shortNumber(n);
                  focusRows();
                }
              }}
              onBlur={(e) => {
                if (e.currentTarget.value !== shortNumber(n)) commitAt(i, e.currentTarget.value);
              }}
            />
          </label>
        ))}
      </span>
    );
  }

  /** Drags the description pane's top edge — taller upward. */
  function startDescriptionDrag(e: ReactPointerEvent<HTMLDivElement>) {
    if (e.button !== 0) return;
    e.preventDefault();
    const target = e.currentTarget;
    target.setPointerCapture(e.pointerId);
    const startY = e.clientY;
    const startHeight = descriptionHeight;
    const onMove = (ev: PointerEvent) => setDescriptionHeight(Math.max(MIN_DESCRIPTION, Math.min(MAX_DESCRIPTION, startHeight - (ev.clientY - startY))));
    const onUp = () => {
      target.removeEventListener("pointermove", onMove);
      target.removeEventListener("pointerup", onUp);
      target.removeEventListener("pointercancel", onUp);
    };
    target.addEventListener("pointermove", onMove);
    target.addEventListener("pointerup", onUp);
    target.addEventListener("pointercancel", onUp);
  }

  function renderValue(p: PropertyDescriptor) {
    const kind = kindOf(p);
    const editable = isEditable(p);
    const busy = busyId === p.id;
    if (kind === "vector") return renderVector(p);
    if (editable && (kind === "bool" || kind === "enum")) {
      const choices = kind === "bool" ? ["true", "false"] : [...(p.options ?? [])];
      const value = draftOf(p);
      // A value outside the declared options still shows as itself.
      if (!choices.includes(value)) choices.unshift(value);
      return (
        <select
          className="sp-propgrid-select"
          value={value}
          disabled={busy}
          onChange={(e) => void commitChoice(p, kind === "bool" ? e.target.value === "true" : e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Escape") {
              e.preventDefault();
              focusRows();
            }
          }}
          onFocus={() => setCursor(`p:${p.id}`)}
        >
          {choices.map((c) => (
            <option key={c} value={c}>
              {kind === "bool" ? (c === "true" ? t("True") : t("False")) : (p.optionLabel?.(c) ?? c)}
            </option>
          ))}
        </select>
      );
    }
    if (editing?.id === p.id) {
      const invalid = error?.id === p.id;
      const listId = p.suggestions?.length ? `sp-propgrid-suggest-${p.id}` : undefined;
      return (
        <>
        {listId && (
          <datalist id={listId}>
            {p.suggestions!.map((s) => (
              <option key={s} value={s} />
            ))}
          </datalist>
        )}
        {kind === "text" ? (
          <textarea
            className="sp-propgrid-editor is-text"
            data-invalid={invalid || undefined}
            aria-invalid={invalid}
            autoFocus
            spellCheck={false}
            rows={Math.min(10, Math.max(3, editing.draft.split("\n").length + 1))}
            value={editing.draft}
            readOnly={busy}
            onFocus={(e) => e.currentTarget.select()}
            onChange={(e) => {
              setEditing({ ...editing, draft: e.target.value });
              if (invalid) setError(null);
            }}
            onKeyDown={(e) => {
              // Enter is a new line here; Ctrl+Enter applies, as in a commit box.
              if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
                e.preventDefault();
                if (!busy) void commitText(p, editing.draft, false);
              } else if (e.key === "Escape") {
                e.preventDefault();
                cancelEdit();
              }
            }}
            onBlur={() => {
              if (!busy && editing.session === sessionRef.current) void commitText(p, editing.draft, true);
            }}
          />
        ) : (
        <input
          className="sp-propgrid-editor"
          list={listId}
          data-invalid={invalid || undefined}
          aria-invalid={invalid}
          autoFocus
          spellCheck={false}
          value={editing.draft}
          // Not `disabled`: that would drop focus, and a rejected edit keeps typing.
          readOnly={busy}
          onFocus={(e) => e.currentTarget.select()}
          onChange={(e) => {
            setEditing({ ...editing, draft: e.target.value });
            if (invalid) setError(null);
          }}
          onKeyDown={(e) => {
            if (e.key === "Enter") {
              e.preventDefault();
              if (!busy) void commitText(p, editing.draft, false);
            } else if (e.key === "Escape") {
              e.preventDefault();
              cancelEdit();
            }
          }}
          onBlur={() => {
            // Not while an Enter commit is in flight, nor once the session
            // is over (the box unmounting after a commit can blur it).
            if (!busy && editing.session === sessionRef.current) void commitText(p, editing.draft, true);
          }}
        />
        )}
        </>
      );
    }
    return (
      <span className="sp-propgrid-value-text">
        {p.swatch && <span className="sp-propgrid-swatch" style={{ boxShadow: `inset 0 0 0 12px ${p.swatch}` }} />}
        {p.image && <img className="sp-propgrid-image" src={p.image} alt="" />}
        {kind === "number" ? <span data-scrub="value">{displayText(p)}</span> : displayText(p)}
      </span>
    );
  }

  const selectedProperty = current?.type === "property" ? current.property : null;
  const toolbarDisabled = objectName === undefined;

  return (
    <div className={["sp-propgrid", compact && "is-compact", className].filter(Boolean).join(" ")}>
      {(!compact || objectName !== undefined) && (
      <div className="sp-propgrid-object" title={objectName ? [`${objectName}  ${objectType ?? ""}`, objectDescription].filter(Boolean).join("\n") : undefined}>
        {objectName !== undefined && (
          <>
            <span className="sp-propgrid-object-name">{objectName}</span>
            {objectType && <span className="sp-propgrid-object-type">{objectType}</span>}
          </>
        )}
      </div>
      )}
      {!compact && (
      <div className="sp-propgrid-toolbar">
        <IconButton
          title={t("Categorized")}
          aria-pressed={mode === "categorized"}
          data-active={mode === "categorized" || undefined}
          disabled={toolbarDisabled}
          onClick={() => setMode("categorized")}
        >
          <Icon name="categorized" />
        </IconButton>
        <IconButton
          title={t("Alphabetical")}
          aria-pressed={mode === "alphabetical"}
          data-active={mode === "alphabetical" || undefined}
          disabled={toolbarDisabled}
          onClick={() => setMode("alphabetical")}
        >
          <Icon name="alphabetical" />
        </IconButton>
        <span className="sp-propgrid-toolbar-separator" />
        <TextField
          className="sp-propgrid-search"
          placeholder={t("Search Properties")}
          value={query}
          disabled={toolbarDisabled}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Escape") setQuery("");
          }}
        />
        <IconButton
          title={descriptionVisible ? t("Hide Description") : t("Show Description")}
          aria-pressed={descriptionVisible}
          data-active={descriptionVisible || undefined}
          onClick={() => setDescriptionVisible((v) => !v)}
        >
          <Icon name="info" />
        </IconButton>
      </div>
      )}
      <div className="sp-propgrid-body" style={{ ["--sp-propgrid-split" as string]: `${split * 100}%` }}>
        {objectName === undefined ? (
          <div className="sp-propgrid-empty">{emptyLabel ?? t("No object selected.")}</div>
        ) : (
          <div
            className="sp-propgrid-rows"
            ref={rowsRef}
            role="grid"
            aria-label={t("{object} properties", { object: objectName })}
            tabIndex={0}
            data-mode={mode}
            onKeyDown={handleKeyDown}
          >
            {items.map((item) => {
              const active = item.key === cursor;
              if (item.type === "category") {
                return (
                  <div
                    key={item.key}
                    data-key={item.key}
                    className="sp-propgrid-category"
                    role="row"
                    aria-expanded={!item.collapsed}
                    aria-selected={active}
                    data-selected={active || undefined}
                    onClick={() => setCursor(item.key)}
                    onDoubleClick={() => !searching && toggleCategory(item.name)}
                  >
                    <span
                      className="sp-propgrid-chevron"
                      data-expanded={!item.collapsed || undefined}
                      onClick={(e) => {
                        e.stopPropagation();
                        setCursor(item.key);
                        if (!searching) toggleCategory(item.name);
                      }}
                    >
                      <Icon name="chevronRight" size={12} />
                    </span>
                    {item.name === DEFAULT_CATEGORY ? t("Misc") : item.name}
                  </div>
                );
              }
              const p = item.property;
              const editable = isEditable(p);
              const rowError = error?.id === p.id ? error.message : null;
              if (kindOf(p) === "custom") {
                return (
                  <div
                    key={item.key}
                    data-key={item.key}
                    data-property-id={p.id}
                    className="sp-propgrid-custom"
                    role="row"
                    aria-selected={active}
                    data-selected={active || undefined}
                    onClick={() => setCursor(item.key)}
                  >
                    {p.name && <div className="sp-propgrid-custom-name">{p.name}</div>}
                    {p.render?.()}
                  </div>
                );
              }
              return (
                <div key={item.key} className="sp-propgrid-entry">
                  <div
                    data-key={item.key}
                    data-property-id={p.id}
                    className="sp-propgrid-row"
                    data-multiline={(editing?.id === p.id && kindOf(p) === "text") || undefined}
                    role="row"
                    aria-selected={active}
                    data-selected={active || undefined}
                    data-readonly={!editable || undefined}
                    onClick={() => setCursor(item.key)}
                  >
                    <div
                      className="sp-propgrid-name"
                      role="gridcell"
                      title={p.name}
                      data-scrubbable={(editable && kindOf(p) === "number") || undefined}
                      onPointerDown={(e) => kindOf(p) === "number" && startScrub(e, p, null)}
                      onDoubleClick={() => (kindOf(p) === "bool" || kindOf(p) === "enum" ? cycle(p) : beginEdit(p))}
                    >
                      <span className="sp-propgrid-name-text">{p.name}</span>
                      {/* The column splitter rides on every row's name/value boundary. */}
                      <span
                        className="sp-propgrid-splitter"
                        onPointerDown={startSplitDrag}
                        onClick={(e) => e.stopPropagation()}
                        onDoubleClick={(e) => e.stopPropagation()}
                      />
                    </div>
                    <div
                      className="sp-propgrid-value"
                      role="gridcell"
                      title={editing?.id === p.id ? undefined : displayText(p)}
                      onClick={() => {
                        if (editable && editing?.id !== p.id && kindOf(p) !== "bool" && kindOf(p) !== "enum" && kindOf(p) !== "vector") beginEdit(p);
                      }}
                      onDoubleClick={() => cycle(p)}
                    >
                      {renderValue(p)}
                      {p.edit && (
                        <button
                          type="button"
                          className="sp-propgrid-ellipsis"
                          title={t("Edit…")}
                          disabled={p.readOnly}
                          onClick={(e) => {
                            e.stopPropagation();
                            p.edit!();
                          }}
                        >
                          …
                        </button>
                      )}
                    </div>
                  </div>
                  {rowError && (
                    <div className="sp-propgrid-error" role="alert">
                      {rowError}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        )}
      </div>
      {descriptionVisible && (
        <>
          <div className="sp-propgrid-description-sizer" onPointerDown={startDescriptionDrag} />
          <div className="sp-propgrid-description" style={{ height: descriptionHeight }}>
            {!current && objectName && objectDescription && (
              <>
                <div className="sp-propgrid-description-name">{objectName}</div>
                <div className="sp-propgrid-description-text">{objectDescription}</div>
              </>
            )}
            {current && (
              <>
                <div className="sp-propgrid-description-name">{current.type === "category" ? (current.name === DEFAULT_CATEGORY ? t("Misc") : current.name) : current.property.name}</div>
                {selectedProperty?.description && <div className="sp-propgrid-description-text">{selectedProperty.description}</div>}
              </>
            )}
          </div>
        </>
      )}
    </div>
  );
}
