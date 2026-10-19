import type { KeyboardEvent } from "react";
import { PROPERTY_KIND, type PropertyDescriptor } from "./propertyTypes";
import { kindOf, MAX_TEXT_ROWS, MIN_TEXT_ROWS } from "./propertyValues";
import type { EditSession } from "./usePropertyGridState";
import type { PropertyGridModel } from "./usePropertyGrid";

/**
 * The text box a string, number or text value is edited in: Enter applies
 * (Ctrl+Enter in a multi-line text, where Enter is a new line), Escape
 * cancels, leaving it applies.
 */
export function TextBox({ grid, p, editing }: { grid: PropertyGridModel; p: PropertyDescriptor; editing: EditSession }) {
  const { error, setError, setEditing, busyId, sessionRef, commitText, cancelEdit } = grid;
  const busy = busyId === p.id;
  const invalid = error?.id === p.id;
  const multiline = kindOf(p) === PROPERTY_KIND.text;
  const listId = p.suggestions?.length && !multiline ? `sp-propgrid-suggest-${p.id}` : undefined;
  const common = {
    // Typing not applied yet: Ctrl+Z undoes it here.
    "data-own-undo": true,
    "data-invalid": invalid || undefined,
    "aria-invalid": invalid,
    autoFocus: true,
    spellCheck: false,
    value: editing.draft,
    // Not `disabled`: that would drop focus, and a rejected edit keeps typing.
    readOnly: busy,
    onFocus: (e: { currentTarget: HTMLInputElement | HTMLTextAreaElement }) => e.currentTarget.select(),
    onChange: (e: { target: HTMLInputElement | HTMLTextAreaElement }) => {
      setEditing({ ...editing, draft: e.target.value });
      if (invalid) setError(null);
    },
    onKeyDown: (e: KeyboardEvent) => {
      if (e.key === "Enter" && (!multiline || e.ctrlKey || e.metaKey)) {
        e.preventDefault();
        if (!busy) void commitText(p, editing.draft, false);
      } else if (e.key === "Escape") {
        e.preventDefault();
        cancelEdit();
      }
    },
    // Not while an Enter commit is in flight, nor once the session is over
    // (the box unmounting after a commit can blur it).
    onBlur: () => {
      if (!busy && editing.session === sessionRef.current) void commitText(p, editing.draft, true);
    },
  };
  if (multiline) {
    const rows = Math.min(MAX_TEXT_ROWS, Math.max(MIN_TEXT_ROWS, editing.draft.split("\n").length + 1));
    return <textarea className="sp-propgrid-editor is-text" rows={rows} {...common} />;
  }
  return (
    <>
      {listId && (
        <datalist id={listId}>
          {p.suggestions?.map((s) => (
            <option key={s} value={s} />
          ))}
        </datalist>
      )}
      <input className="sp-propgrid-editor" list={listId} {...common} />
    </>
  );
}
