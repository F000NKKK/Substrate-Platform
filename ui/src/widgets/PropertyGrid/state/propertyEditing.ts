import { notify } from "../../Notifications";
import { t } from "../../../infra/i18n";
import { PROPERTY_KIND, type PropertyDescriptor, type PropertyValue } from "./propertyTypes";
import { applyEdit, draftOf, isEditable, kindOf, propertyKey, sameValue } from "./propertyValues";
import type { PropertyGridState } from "./usePropertyGridState";

/** Kinds edited by an always-rendered control (a check box, a dropdown, number fields) rather than a text box. */
const IN_PLACE: ReadonlySet<string> = new Set([PROPERTY_KIND.bool, PROPERTY_KIND.enum, PROPERTY_KIND.vector]);

/** Whether `p` is edited in place (no text box to open). */
export function editedInPlace(p: PropertyDescriptor): boolean {
  return IN_PLACE.has(kindOf(p));
}

/** A number typed into a box, or why it isn't one. */
export function parseNumber(text: string): number | { error: string } {
  const n = Number(text.trim());
  return text.trim() === "" || Number.isNaN(n) ? { error: t("Not a valid number.") } : n;
}

/** Editing the grid's values: opening a text box, committing, cancelling, stepping a choice. */
export function propertyEditing(grid: PropertyGridState) {
  const { busyId, setBusyId, setCursor, setEditing, setError, sessionRef, rowsRef, focusRows } = grid;

  function beginEdit(p: PropertyDescriptor) {
    setCursor(propertyKey(p));
    if (!isEditable(p) || busyId === p.id || kindOf(p) === PROPERTY_KIND.custom) return;
    if (editedInPlace(p)) {
      // Always-rendered dropdowns and fields — focusing one is "editing" it.
      const row = `[data-property-id="${CSS.escape(p.id)}"]`;
      rowsRef.current?.querySelector<HTMLElement>(`${row} select, ${row} input`)?.focus();
      return;
    }
    setEditing({ id: p.id, draft: draftOf(p), session: ++sessionRef.current });
  }

  function endSession() {
    sessionRef.current++;
    setEditing(null);
    setError(null);
  }

  function cancelEdit() {
    endSession();
    focusRows();
  }

  /** Commits `value` — resolves the rejection's message, or `null` once accepted (shown by the caller: inline, or `notify` on blur). */
  async function commit(p: PropertyDescriptor, value: PropertyValue): Promise<string | null> {
    if (sameValue(value, p.value)) return null;
    setBusyId(p.id);
    const message = await applyEdit(p, value);
    setBusyId(null);
    return message;
  }

  async function commitText(p: PropertyDescriptor, draft: string, viaBlur: boolean) {
    const session = sessionRef.current;
    const parsed = kindOf(p) === PROPERTY_KIND.number ? parseNumber(draft) : draft;
    const message = typeof parsed === "object" ? parsed.error : await commit(p, parsed);
    if (session !== sessionRef.current) return;
    if (!message) {
      endSession();
      if (!viaBlur) focusRows();
    } else if (viaBlur) {
      // The text box is going away — report it where it stays visible.
      endSession();
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
    if (kind === PROPERTY_KIND.bool) void commitChoice(p, !p.value);
    else if (kind === PROPERTY_KIND.enum && p.options?.length) {
      const next = p.options[(p.options.indexOf(String(p.value)) + 1) % p.options.length];
      void commitChoice(p, next);
    }
  }

  return { beginEdit, cancelEdit, commitText, commitChoice, cycle };
}

export type PropertyEditing = ReturnType<typeof propertyEditing>;
