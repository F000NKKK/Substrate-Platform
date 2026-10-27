import { useState } from "react";
import { useT } from "../infra/i18n";
import type { DapSession } from "./dapSession";
import type { DebugVariable } from "./debugState";
import { useDebugState } from "./useDebugState";
import "./dap.css";

/** One variable, expandable when it has children (loaded on the first expand). */
function VariableRow({ session, variable }: { session: DapSession; variable: DebugVariable }) {
  const [open, setOpen] = useState(false);
  const { variables } = useDebugState(session);
  const expandable = variable.variablesReference > 0;
  const children = variables.get(variable.variablesReference);
  const toggle = () => {
    if (!expandable) return;
    if (!open) void session.loadVariables(variable.variablesReference);
    setOpen(!open);
  };
  return (
    <li>
      <div className="sp-debug-row" onClick={toggle} aria-expanded={expandable ? open : undefined}>
        <span className="sp-debug-twisty">{expandable ? (open ? "▾" : "▸") : ""}</span>
        <span className="sp-debug-name">{variable.name}</span>
        <span className="sp-debug-value" title={variable.type}>{variable.value}</span>
      </div>
      {open && children && <VariableList session={session} variables={children} />}
    </li>
  );
}

function VariableList({ session, variables }: { session: DapSession; variables: readonly DebugVariable[] }) {
  return (
    <ul className="sp-debug-list sp-debug-nested">
      {variables.map((v) => (
        <VariableRow key={v.name} session={session} variable={v} />
      ))}
    </ul>
  );
}

/** The selected frame's scopes and their variables. */
export function VariablesPanel({ session }: { session: DapSession }) {
  const t = useT();
  const { status, scopes, variables } = useDebugState(session);
  if (status !== "stopped") return <div className="sp-debug-empty">{t("Not paused.")}</div>;
  return (
    <div className="sp-debug-scopes" aria-label={t("Variables")}>
      {scopes.map((scope) => {
        const children = variables.get(scope.variablesReference);
        return (
          <section key={scope.name}>
            <h4 className="sp-debug-scope" onClick={() => void session.loadVariables(scope.variablesReference)}>{scope.name}</h4>
            {children && <VariableList session={session} variables={children} />}
          </section>
        );
      })}
    </div>
  );
}
