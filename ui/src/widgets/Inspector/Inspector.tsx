import type { ReactNode } from "react";
import { PropertyGrid, type PropertyGridProps } from "../PropertyGrid";
import "./Inspector.css";

export interface InspectorProps extends Omit<PropertyGridProps, "compact" | "className"> {
  /** Above the properties: a preview or a picker that isn't a property. */
  header?: ReactNode;
  /** Under them, as muted text: how to use the editor, what to do next. */
  hint?: ReactNode;
  /** Under them, as given: a pane of its own (a UV editor, actions). */
  footer?: ReactNode;
  /** Its width, px — default 320. */
  width?: number;
  /** Which edge it stands on — the border goes on the other side. Default `right`. */
  side?: "left" | "right";
  className?: string;
}

/**
 * An editor's inspector — the one look every designer's side panel has:
 * the selected object's name, its properties as Visual Studio's Properties
 * window lists them (a `PropertyGrid`: categories, a name/value grid with a
 * movable split, the description pane), the author's order by default, no
 * toolbar; a header or footer for what isn't a property.
 */
export function Inspector({ header, hint, footer, width = 320, side = "right", className, order = "declared", ...grid }: InspectorProps) {
  return (
    <aside className={["sp-inspector", `is-${side}`, className].filter(Boolean).join(" ")} style={{ width }}>
      {header && <div className="sp-inspector-header">{header}</div>}
      <div className="sp-inspector-grid">
        <PropertyGrid {...grid} order={order} compact />
      </div>
      {hint && <div className="sp-inspector-hint">{hint}</div>}
      {footer && <div className="sp-inspector-footer">{footer}</div>}
    </aside>
  );
}
