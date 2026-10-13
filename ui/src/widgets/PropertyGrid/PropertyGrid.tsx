import { useMemo, useRef, useState, type KeyboardEvent, type PointerEvent as ReactPointerEvent } from "react";
import { Icon } from "../../infra/icons";
import { useStoredState } from "../../infra/state";
import { IconButton } from "../IconButton";
import { TextField } from "../TextField";
import { notify } from "../Notifications";
import { useT } from "../../infra/i18n";
import "./PropertyGrid.css";

export type PropertyKind = "string" | "number" | "bool" | "enum";
export type PropertyValue = string | number | boolean;

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
}

export interface PropertyGridProps {
  /** Bold part of the object selector line (e.g. `main.rs`) — omit when nothing is selected, which shows `emptyLabel` instead. */
  objectName?: string;
  /** Rest of the object selector line (e.g. `File Properties`). */
  objectType?: string;
  /** Identifies the shown object — an in-progress edit or error is dropped when it changes. Defaults to `objectName`. */
  objectKey?: string;
  properties: PropertyDescriptor[];
  emptyLabel?: string;
  /** Prefix for the view mode, column split, collapsed categories and description pane state remembered in the current `StateStore`. */
  persistKey?: string;
  className?: string;
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
  if (typeof p.value === "boolean") return p.value ? "True" : "False";
  return String(p.value);
}

function draftOf(p: PropertyDescriptor): string {
  return p.value === null ? "" : String(p.value);
}

const byName = (a: PropertyDescriptor, b: PropertyDescriptor) => a.name.localeCompare(b.name);

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
  objectKey,
  properties,
  emptyLabel,
  persistKey = "properties",
  className,
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
    for (const name of [...groups.keys()].sort((a, b) => a.localeCompare(b))) {
      // While searching every match shows, collapsed or not.
      const isCollapsed = !searching && collapsed.has(name);
      out.push({ type: "category", key: `c:${name}`, name, collapsed: isCollapsed });
      if (!isCollapsed) for (const p of groups.get(name)!.sort(byName)) out.push({ type: "property", key: `p:${p.id}`, property: p });
    }
    return out;
  }, [properties, mode, query, searching, collapsed]);

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
    if (kind === "bool" || kind === "enum") {
      // Always-rendered dropdowns — focusing one is "editing" it.
      rowsRef.current?.querySelector<HTMLSelectElement>(`[data-property-id="${CSS.escape(p.id)}"] select`)?.focus();
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
    if (value === p.value) return null;
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
    const message = typeof parsed === "object" ? parsed.error : await commit(p, parsed);
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
    };
    target.addEventListener("pointermove", onMove);
    target.addEventListener("pointerup", onUp);
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
    };
    target.addEventListener("pointermove", onMove);
    target.addEventListener("pointerup", onUp);
  }

  function renderValue(p: PropertyDescriptor) {
    const kind = kindOf(p);
    const editable = isEditable(p);
    const busy = busyId === p.id;
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
              {kind === "bool" ? (c === "true" ? "True" : "False") : c}
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
        </>
      );
    }
    return (
      <span className="sp-propgrid-value-text">
        {p.swatch && <span className="sp-propgrid-swatch" style={{ boxShadow: `inset 0 0 0 12px ${p.swatch}` }} />}
        {p.image && <img className="sp-propgrid-image" src={p.image} alt="" />}
        {displayText(p)}
      </span>
    );
  }

  const selectedProperty = current?.type === "property" ? current.property : null;
  const toolbarDisabled = objectName === undefined;

  return (
    <div className={["sp-propgrid", className].filter(Boolean).join(" ")}>
      <div className="sp-propgrid-object" title={objectName ? `${objectName}  ${objectType ?? ""}` : undefined}>
        {objectName !== undefined && (
          <>
            <span className="sp-propgrid-object-name">{objectName}</span>
            {objectType && <span className="sp-propgrid-object-type">{objectType}</span>}
          </>
        )}
      </div>
      <div className="sp-propgrid-toolbar">
        <IconButton
          size={22}
          title={t("Categorized")}
          aria-pressed={mode === "categorized"}
          data-active={mode === "categorized" || undefined}
          disabled={toolbarDisabled}
          onClick={() => setMode("categorized")}
        >
          <Icon name="categorized" size={14} />
        </IconButton>
        <IconButton
          size={22}
          title={t("Alphabetical")}
          aria-pressed={mode === "alphabetical"}
          data-active={mode === "alphabetical" || undefined}
          disabled={toolbarDisabled}
          onClick={() => setMode("alphabetical")}
        >
          <Icon name="alphabetical" size={14} />
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
          size={22}
          title={descriptionVisible ? t("Hide Description") : t("Show Description")}
          aria-pressed={descriptionVisible}
          data-active={descriptionVisible || undefined}
          onClick={() => setDescriptionVisible((v) => !v)}
        >
          <Icon name="info" size={14} />
        </IconButton>
      </div>
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
              return (
                <div key={item.key} className="sp-propgrid-entry">
                  <div
                    data-key={item.key}
                    data-property-id={p.id}
                    className="sp-propgrid-row"
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
                        if (editable && editing?.id !== p.id && kindOf(p) !== "bool" && kindOf(p) !== "enum") beginEdit(p);
                      }}
                      onDoubleClick={() => cycle(p)}
                    >
                      {renderValue(p)}
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
