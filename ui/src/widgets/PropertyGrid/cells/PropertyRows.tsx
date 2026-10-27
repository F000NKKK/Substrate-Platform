import { Icon } from "../../../infra/icons";
import { useT } from "../../../infra/i18n";
import { PROPERTY_KIND, type PropertyDescriptor } from "../state/propertyTypes";
import { categoryLabel, displayText, type GridItem, isEditable, kindOf } from "../state/propertyValues";
import { editedInPlace } from "../state/propertyEditing";
import type { PropertyGridModel } from "../state/usePropertyGrid";
import { ValueCell } from "./ValueCell";

type Category = Extract<GridItem, { type: "category" }>;
type Property = Extract<GridItem, { type: "property" }>;

/** The rows: categories, custom rows and properties, keyboard-navigable as one grid. */
export function PropertyRows({ grid, objectName }: { grid: PropertyGridModel; objectName: string }) {
  const t = useT();
  const { rowsRef, mode, handleKeyDown, items } = grid;
  return (
    <div className="sp-propgrid-rows" ref={rowsRef} role="grid" aria-label={t("{object} properties", { object: objectName })} tabIndex={0} data-mode={mode} onKeyDown={handleKeyDown}>
      {items.map((item) =>
        item.type === "category" ? (
          <CategoryRow key={item.key} grid={grid} item={item} />
        ) : kindOf(item.property) === PROPERTY_KIND.custom ? (
          <CustomRow key={item.key} grid={grid} item={item} />
        ) : (
          <PropertyRow key={item.key} grid={grid} item={item} />
        ),
      )}
    </div>
  );
}

/** What every row carries: its key, whether it's the cursor's, and that a click puts the cursor there. */
function rowProps(grid: PropertyGridModel, key: string) {
  const active = key === grid.cursor;
  return { "data-key": key, role: "row", "aria-selected": active, "data-selected": active || undefined, onClick: () => grid.setCursor(key) };
}

function CategoryRow({ grid, item }: { grid: PropertyGridModel; item: Category }) {
  const { searching, toggleCategory, setCursor } = grid;
  return (
    <div className="sp-propgrid-category" aria-expanded={!item.collapsed} {...rowProps(grid, item.key)} onDoubleClick={() => !searching && toggleCategory(item.name)}>
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
      {categoryLabel(item.name)}
    </div>
  );
}

/** A `"custom"` row: what it renders, across the whole width. */
function CustomRow({ grid, item }: { grid: PropertyGridModel; item: Property }) {
  const p = item.property;
  return (
    <div className="sp-propgrid-custom" data-property-id={p.id} {...rowProps(grid, item.key)}>
      {p.name && <div className="sp-propgrid-custom-name">{p.name}</div>}
      {p.render?.()}
    </div>
  );
}

function PropertyRow({ grid, item }: { grid: PropertyGridModel; item: Property }) {
  const { editing, error, beginEdit, cycle } = grid;
  const p = item.property;
  const kind = kindOf(p);
  const editable = isEditable(p);
  const isEditing = editing?.id === p.id;
  const rowError = error?.id === p.id ? error.message : null;
  return (
    <div className="sp-propgrid-entry">
      <div
        className="sp-propgrid-row"
        data-property-id={p.id}
        data-multiline={(isEditing && kind === PROPERTY_KIND.text) || undefined}
        data-readonly={!editable || undefined}
        {...rowProps(grid, item.key)}
      >
        <NameCell grid={grid} p={p} />
        <div
          className="sp-propgrid-value"
          role="gridcell"
          title={isEditing ? undefined : displayText(p)}
          onClick={() => {
            if (editable && !isEditing && !editedInPlace(p)) beginEdit(p);
          }}
          onDoubleClick={() => kind !== PROPERTY_KIND.bool && cycle(p)}
        >
          <ValueCell grid={grid} p={p} />
          {p.edit && <EllipsisButton p={p} />}
        </div>
      </div>
      {rowError && (
        <div className="sp-propgrid-error" role="alert">
          {rowError}
        </div>
      )}
    </div>
  );
}

/** A property's name: scrubbed for a number, double-clicked to edit or step, with the column splitter on its edge. */
function NameCell({ grid, p }: { grid: PropertyGridModel; p: PropertyDescriptor }) {
  const { startScrub, startSplitDrag, cycle, beginEdit } = grid;
  const kind = kindOf(p);
  const isNumber = kind === PROPERTY_KIND.number;
  return (
    <div
      className="sp-propgrid-name"
      role="gridcell"
      title={p.name}
      data-scrubbable={(isEditable(p) && isNumber) || undefined}
      onPointerDown={(e) => isNumber && startScrub(e, p, null)}
      onDoubleClick={() => (kind === PROPERTY_KIND.bool || kind === PROPERTY_KIND.enum ? cycle(p) : beginEdit(p))}
    >
      <span className="sp-propgrid-name-text">{p.name}</span>
      {/* The column splitter rides on every row's name/value boundary. */}
      <span className="sp-propgrid-splitter" onPointerDown={startSplitDrag} onClick={(e) => e.stopPropagation()} onDoubleClick={(e) => e.stopPropagation()} />
    </div>
  );
}

/** The `…` button of a property with its own editor (`edit`). */
function EllipsisButton({ p }: { p: PropertyDescriptor }) {
  const t = useT();
  return (
    <button
      type="button"
      className="sp-propgrid-ellipsis"
      title={t("Edit…")}
      disabled={p.readOnly}
      onClick={(e) => {
        e.stopPropagation();
        p.edit?.();
      }}
    >
      …
    </button>
  );
}
