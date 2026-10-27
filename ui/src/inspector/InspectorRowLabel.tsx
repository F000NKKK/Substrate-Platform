import { useT } from "../infra/i18n";
import { displayText, problemText } from "./displayText";
import type { InspectorRow } from "./inspectorRow";
import { ValueEditor, type EditHandler } from "./ValueEditor";

interface LabelProps {
  readonly row: InspectorRow;
  /** The edit handler, when the inspector is editable. */
  readonly onEdit: EditHandler | null;
}

/** One row's content: name, type, then the value (or its editor). */
export function InspectorRowLabel({ row, onEdit }: LabelProps) {
  const t = useT();
  const text = displayText(row.display);
  const editable = onEdit !== null && row.editor !== null;
  const issues = row.messages.length > 0 ? row.messages.join("\n") : row.issueCount > 0 ? t("{count} problems inside", { count: row.issueCount }) : null;
  return (
    <span className="sp-inspector-row" data-problem={row.problem ? row.problem.kind : undefined}>
      <span className="sp-inspector-name" title={row.doc ?? undefined}>
        {row.name}
      </span>
      <span className="sp-inspector-type">{row.typeLabel}</span>
      {editable ? (
        <ValueEditor row={row} editor={row.editor!} onEdit={onEdit} />
      ) : (
        <span className="sp-inspector-value" data-kind={row.display.kind}>
          {text}
        </span>
      )}
      {row.problem && <span className="sp-inspector-problem">{problemText(row.problem)}</span>}
      {issues && (
        <span className="sp-inspector-issue" role="img" aria-label={issues} title={issues}>
          {row.messages.length > 0 ? "⚠" : "●"}
        </span>
      )}
    </span>
  );
}
