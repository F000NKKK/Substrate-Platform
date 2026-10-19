import { PROPERTY_KIND, type PropertyDescriptor } from "./propertyTypes";
import { displayText, draftOf, isEditable, kindOf, propertyKey } from "./propertyValues";
import { TextBox } from "./TextBox";
import type { PropertyGridModel } from "./usePropertyGrid";
import { VectorValue } from "./VectorValue";

/** A property's value cell: the control its kind is edited with, or its text. */
export function ValueCell({ grid, p }: { grid: PropertyGridModel; p: PropertyDescriptor }) {
  const { busyId, editing, setCursor, commitChoice, focusRows } = grid;
  const kind = kindOf(p);
  const editable = isEditable(p);
  const busy = busyId === p.id;
  const onFocus = () => setCursor(propertyKey(p));
  if (kind === PROPERTY_KIND.vector) return <VectorValue grid={grid} p={p} />;
  if (kind === PROPERTY_KIND.bool) {
    // A check box, as a designer's inspector has — toggled in place.
    return <input type="checkbox" className="sp-propgrid-check" checked={p.value === true} disabled={!editable || busy} aria-label={p.name} onChange={(e) => void commitChoice(p, e.target.checked)} onFocus={onFocus} />;
  }
  if (editable && kind === PROPERTY_KIND.enum) {
    const value = draftOf(p);
    // A value outside the declared options still shows as itself.
    const choices = p.options?.includes(value) ? p.options : [value, ...(p.options ?? [])];
    return (
      <select
        className="sp-propgrid-select"
        value={value}
        disabled={busy}
        onChange={(e) => void commitChoice(p, e.target.value)}
        onKeyDown={(e) => {
          if (e.key === "Escape") {
            e.preventDefault();
            focusRows();
          }
        }}
        onFocus={onFocus}
      >
        {choices.map((c) => (
          <option key={c} value={c}>
            {p.optionLabel?.(c) ?? c}
          </option>
        ))}
      </select>
    );
  }
  if (editing?.id === p.id) return <TextBox grid={grid} p={p} editing={editing} />;
  return (
    <span className="sp-propgrid-value-text">
      {p.swatch && <span className="sp-propgrid-swatch" style={{ boxShadow: `inset 0 0 0 12px ${p.swatch}` }} />}
      {p.image && <img className="sp-propgrid-image" src={p.image} alt="" />}
      {kind === PROPERTY_KIND.number ? <span data-scrub="value">{displayText(p)}</span> : displayText(p)}
    </span>
  );
}
