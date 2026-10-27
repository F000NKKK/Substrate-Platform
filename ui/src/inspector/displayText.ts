import { t, tn } from "../infra/i18n";
import type { EditError } from "./editValue";
import type { RowDisplay, RowProblem } from "./inspectorRow";

/** What a row shows as its value, in the user's language. */
export function displayText(display: RowDisplay): string {
  switch (display.kind) {
    case "scalar":
      return JSON.stringify(display.value);
    case "variant":
      return display.name;
    case "list":
      return tn(display.length, "{count} item", "{count} items", { count: display.length });
    case "map":
      return tn(display.length, "{count} entry", "{count} entries", { count: display.length });
    case "struct":
      return display.name;
    case "none":
      return t("none");
    case "default":
      return t("default");
    case "missing":
      return t("missing");
  }
}

export function problemText(problem: RowProblem): string {
  switch (problem.kind) {
    case "mismatch":
      return t("Expected {expected}, found {found}.", { expected: problem.expected, found: problem.found });
    case "unknownVariant":
      return t("Unknown variant {name}.", { name: problem.name });
    case "unknownDefinition":
      return t("No definition named {name}.", { name: problem.name });
    case "missing":
      return t("Required value is missing.");
  }
}

export function editErrorText(error: EditError): string {
  switch (error.kind) {
    case "notNumber":
      return t("Enter a number.");
    case "notInteger":
      return t("Enter a whole number.");
    case "outOfRange":
      return t("Enter a value from {min} to {max}.", { min: error.min, max: error.max });
  }
}
