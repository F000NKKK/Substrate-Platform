import { useT } from "../../infra/i18n";
import "./PropertyGrid.css";
import { DescriptionPane, ObjectLine, PropertyGridToolbar } from "./PropertyGridChrome";
import { PropertyRows } from "./PropertyRows";
import type { PropertyGridProps } from "./propertyTypes";
import { usePropertyGrid } from "./usePropertyGrid";

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
export function PropertyGrid(props: PropertyGridProps) {
  const { objectName, objectType, objectDescription, emptyLabel, className, compact = false } = props;
  const t = useT();
  const grid = usePropertyGrid(props);
  return (
    <div className={["sp-propgrid", compact && "is-compact", className].filter(Boolean).join(" ")}>
      {(!compact || objectName !== undefined) && <ObjectLine objectName={objectName} objectType={objectType} objectDescription={objectDescription} />}
      {!compact && <PropertyGridToolbar grid={grid} disabled={objectName === undefined} />}
      <div className="sp-propgrid-body" style={{ ["--sp-propgrid-split" as string]: `${grid.split * 100}%` }}>
        {objectName === undefined ? <div className="sp-propgrid-empty">{emptyLabel ?? t("No object selected.")}</div> : <PropertyRows grid={grid} objectName={objectName} />}
      </div>
      {grid.descriptionVisible && <DescriptionPane grid={grid} objectName={objectName} objectDescription={objectDescription} />}
    </div>
  );
}
