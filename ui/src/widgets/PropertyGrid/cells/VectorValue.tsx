import type { PropertyDescriptor } from "../state/propertyTypes";
import { parseNumber } from "../state/propertyEditing";
import { AXES, isEditable, propertyKey, shortNumber } from "../state/propertyValues";
import type { PropertyGridModel } from "../state/usePropertyGrid";

/** A vector's fields: each component its own box, committed on Enter or leaving it. */
export function VectorValue({ grid, p }: { grid: PropertyGridModel; p: PropertyDescriptor }) {
  const { busyId, setCursor, setError, commitChoice, focusRows, startScrub } = grid;
  const v = Array.isArray(p.value) ? (p.value as readonly number[]) : [];
  const labels = p.labels ?? AXES;
  const disabled = !isEditable(p) || busyId === p.id;
  const commitAt = (i: number, text: string) => {
    const n = parseNumber(text);
    if (typeof n === "object") {
      setError({ id: p.id, message: n.error });
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
            disabled={disabled}
            spellCheck={false}
            inputMode="decimal"
            onFocus={(e) => {
              setCursor(propertyKey(p));
              e.currentTarget.select();
            }}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                e.preventDefault();
                commitAt(i, e.currentTarget.value);
                // Done typing: the keyboard is the grid's again (Ctrl+Z undoes the edit, not the typing).
                focusRows();
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
