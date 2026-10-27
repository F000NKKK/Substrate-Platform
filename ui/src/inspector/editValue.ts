import type { DataValue } from "./dataValue";
import type { RowEditor } from "./inspectorRow";
import { fail, ok, type Result } from "./result";

/** Why typed text is not a value for its editor. */
export type EditError =
  | { readonly kind: "notNumber" }
  | { readonly kind: "notInteger" }
  | { readonly kind: "outOfRange"; readonly min: number; readonly max: number };

/** The value typed `text` stands for in `editor` (a number, an integer within its type, or the text itself). */
export function parseEdit(editor: RowEditor, text: string): Result<DataValue, EditError> {
  switch (editor.kind) {
    case "integer":
      return parseInteger(text, editor.min, editor.max);
    case "number":
      return parseNumber(text, editor.range);
    default:
      return ok(text);
  }
}

function parseFinite(text: string): number | null {
  const trimmed = text.trim();
  const number = trimmed === "" ? Number.NaN : Number(trimmed);
  return Number.isFinite(number) ? number : null;
}

function parseInteger(text: string, min: number, max: number): Result<DataValue, EditError> {
  const number = parseFinite(text);
  if (number === null) return fail({ kind: "notNumber" });
  if (!Number.isInteger(number)) return fail({ kind: "notInteger" });
  return number < min || number > max ? fail({ kind: "outOfRange", min, max }) : ok(number);
}

function parseNumber(text: string, range: readonly [number, number] | null): Result<DataValue, EditError> {
  const number = parseFinite(text);
  if (number === null) return fail({ kind: "notNumber" });
  return range && (number < range[0] || number > range[1]) ? fail({ kind: "outOfRange", min: range[0], max: range[1] }) : ok(number);
}
