import { t } from "../infra/i18n";

/** Why a description or a value could not be shown; `at` is a JSON pointer into the input. */
export type InspectorError =
  | { readonly kind: "malformed"; readonly at: string; readonly expected: string; readonly found: string }
  | { readonly kind: "unknownShapeKind"; readonly at: string; readonly found: string }
  | { readonly kind: "unknownDefinitionKind"; readonly at: string; readonly found: string }
  | { readonly kind: "unknownDefinition"; readonly at: string; readonly name: string };

/** The error in the user's language. */
export function describeInspectorError(error: InspectorError): string {
  switch (error.kind) {
    case "malformed":
      return t("{at}: expected {expected}, found {found}.", { at: error.at || "/", expected: error.expected, found: error.found });
    case "unknownShapeKind":
      return t("{at}: unknown shape kind {found}.", { at: error.at || "/", found: error.found });
    case "unknownDefinitionKind":
      return t("{at}: unknown definition kind {found}.", { at: error.at || "/", found: error.found });
    case "unknownDefinition":
      return t("{at}: no definition named {name}.", { at: error.at || "/", name: error.name });
  }
}
