// A product-neutral data inspector: a value and the Substrate Data shape that
// describes it as a tree of properties, read-only first, with typed in-place
// edits the host applies.

export { DataInspector } from "./DataInspector";
export type { DataInspectorProps } from "./DataInspector";
export { parseShapeDescription } from "./parseShapeDescription";
export { describeInspectorError } from "./inspectorError";
export type { InspectorError } from "./inspectorError";
export { buildRows } from "./buildRows";
export type { BuildRowsOptions } from "./buildRows";
export { defaultExpansion } from "./expansion";
export { pathId, samePath } from "./dataPath";
export type { DataPath, PathSegment } from "./dataPath";
export { isDataValue } from "./dataValue";
export type { DataValue } from "./dataValue";
export { parseEdit } from "./editValue";
export type { EditError } from "./editValue";
export { ok, fail } from "./result";
export type { Result } from "./result";
export type { InspectorDiagnostic, InspectorRow, RowDisplay, RowEditor, RowProblem } from "./inspectorRow";
export type { Definition, FieldShape, ShapeDescription, ShapeNode, VariantShape } from "./shapeTypes";
