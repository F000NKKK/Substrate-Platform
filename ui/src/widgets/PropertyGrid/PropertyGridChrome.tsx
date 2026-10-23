import { Icon } from "../../infra/icons";
import { useT } from "../../infra/i18n";
import { IconButton } from "../IconButton";
import { TextField } from "../TextField";
import type { PropertyGridProps } from "./state/propertyTypes";
import { categoryLabel, VIEW_MODE, type ViewMode } from "./state/propertyValues";
import type { PropertyGridModel } from "./state/usePropertyGrid";

/** The object selector line: the object's name in bold, then its type. */
export function ObjectLine({ objectName, objectType, objectDescription }: Pick<PropertyGridProps, "objectName" | "objectType" | "objectDescription">) {
  return (
    <div className="sp-propgrid-object" title={objectName ? [`${objectName}  ${objectType ?? ""}`, objectDescription].filter(Boolean).join("\n") : undefined}>
      {objectName !== undefined && (
        <>
          <span className="sp-propgrid-object-name">{objectName}</span>
          {objectType && <span className="sp-propgrid-object-type">{objectType}</span>}
        </>
      )}
    </div>
  );
}

/** Categorized/Alphabetical, the search box, and the description pane's toggle. */
export function PropertyGridToolbar({ grid, disabled }: { grid: PropertyGridModel; disabled: boolean }) {
  const t = useT();
  const { mode, setMode, query, setQuery, descriptionVisible, setDescriptionVisible } = grid;
  const modes: readonly { mode: ViewMode; label: string; icon: string }[] = [
    { mode: VIEW_MODE.categorized, label: t("Categorized"), icon: "categorized" },
    { mode: VIEW_MODE.alphabetical, label: t("Alphabetical"), icon: "alphabetical" },
  ];
  return (
    <div className="sp-propgrid-toolbar">
      {modes.map((b) => (
        <IconButton key={b.mode} title={b.label} aria-pressed={mode === b.mode} data-active={mode === b.mode || undefined} disabled={disabled} onClick={() => setMode(b.mode)}>
          <Icon name={b.icon} />
        </IconButton>
      ))}
      <span className="sp-propgrid-toolbar-separator" />
      <TextField
        data-own-undo
        className="sp-propgrid-search"
        placeholder={t("Search Properties")}
        value={query}
        disabled={disabled}
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
  );
}

/** The description pane: the selected row's name and description — or the object's while none is. */
export function DescriptionPane({ grid, objectName, objectDescription }: { grid: PropertyGridModel } & Pick<PropertyGridProps, "objectName" | "objectDescription">) {
  const { current, descriptionHeight, startDescriptionDrag } = grid;
  const property = current?.type === "property" ? current.property : null;
  return (
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
            <div className="sp-propgrid-description-name">{current.type === "category" ? categoryLabel(current.name) : current.property.name}</div>
            {property?.description && <div className="sp-propgrid-description-text">{property.description}</div>}
          </>
        )}
      </div>
    </>
  );
}
