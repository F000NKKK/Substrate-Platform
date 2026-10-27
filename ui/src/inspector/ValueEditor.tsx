import { useState } from "react";
import { useT } from "../infra/i18n";
import type { DataPath } from "./dataPath";
import type { DataValue } from "./dataValue";
import { editErrorText } from "./displayText";
import { parseEdit } from "./editValue";
import type { InspectorRow, RowEditor } from "./inspectorRow";

export type EditHandler = (path: DataPath, value: DataValue) => void;

interface EditorProps {
  readonly row: InspectorRow;
  readonly editor: RowEditor;
  readonly onEdit: EditHandler;
}

function variantValue(tag: string | null, name: string): DataValue {
  return tag === null ? name : { [tag]: name };
}

/** A text box that commits on Enter or blur when its text is a valid value, and goes back to the value on Escape. */
function TextEditor({ row, editor, onEdit }: EditorProps) {
  const t = useT();
  const shown = row.value === undefined ? "" : String(row.value);
  const [draft, setDraft] = useState(shown);
  const parsed = parseEdit(editor, draft);
  const commit = () => {
    if (draft !== shown && parsed.ok) onEdit(row.path, parsed.value);
  };
  return (
    <input
      className="sp-inspector-input"
      value={draft}
      aria-label={t("Value of {name}", { name: row.name })}
      aria-invalid={parsed.ok ? undefined : true}
      title={parsed.ok ? undefined : editErrorText(parsed.error)}
      inputMode={editor.kind === "text" ? undefined : "decimal"}
      onChange={(event) => setDraft(event.target.value)}
      onBlur={commit}
      onClick={(event) => event.stopPropagation()}
      onKeyDown={(event) => {
        event.stopPropagation();
        if (event.key === "Enter") commit();
        if (event.key === "Escape") setDraft(shown);
      }}
    />
  );
}

/** The in-place editor of a scalar row; the host decides whether to show it (it is only for `readOnly={false}`). */
export function ValueEditor({ row, editor, onEdit }: EditorProps) {
  const t = useT();
  if (editor.kind === "boolean") {
    return (
      <input
        type="checkbox"
        className="sp-inspector-check"
        checked={row.value === true}
        aria-label={t("Value of {name}", { name: row.name })}
        onClick={(event) => event.stopPropagation()}
        onChange={(event) => onEdit(row.path, event.target.checked)}
      />
    );
  }
  if (editor.kind === "variant") {
    return (
      <select
        className="sp-inspector-select"
        value={String(row.value)}
        aria-label={t("Variant of {name}", { name: row.name })}
        onClick={(event) => event.stopPropagation()}
        onKeyDown={(event) => event.stopPropagation()}
        onChange={(event) => onEdit(row.path, variantValue(editor.tag, event.target.value))}
      >
        {editor.options.map((name) => (
          <option key={name} value={name}>
            {name}
          </option>
        ))}
      </select>
    );
  }
  return <TextEditor key={String(row.value)} row={row} editor={editor} onEdit={onEdit} />;
}
