import { useT } from "../infra/i18n";
import type { DapSession } from "./dapSession";
import { selectedFrame, type DebugFrame } from "./debugState";
import { useDebugState } from "./useDebugState";
import "./dap.css";

export interface CallStackPanelProps {
  session: DapSession;
  /** Opens `frame`'s file at its line — a frame the user picked. */
  onNavigate?: (frame: DebugFrame) => void;
}

const baseName = (path: string) => path.slice(Math.max(path.lastIndexOf("/"), path.lastIndexOf("\\")) + 1);

/** The paused program's call stack; picking a frame shows its place and variables. */
export function CallStackPanel({ session, onNavigate }: CallStackPanelProps) {
  const t = useT();
  const state = useDebugState(session);
  const current = selectedFrame(state);
  if (state.status !== "stopped") return <div className="sp-debug-empty">{state.status === "running" ? t("The program is running.") : t("Not paused.")}</div>;
  return (
    <ul className="sp-debug-list" role="listbox" aria-label={t("Call Stack")}>
      {state.frames.map((frame) => (
        <li
          key={frame.id}
          role="option"
          aria-selected={frame === current}
          className={["sp-debug-row", frame === current && "sp-debug-row--selected"].filter(Boolean).join(" ")}
          onClick={() => {
            void session.selectFrame(frame.id);
            onNavigate?.(frame);
          }}
        >
          <span className="sp-debug-name">{frame.name}</span>
          {frame.path && <span className="sp-debug-where">{`${baseName(frame.path)}:${frame.line}`}</span>}
        </li>
      ))}
    </ul>
  );
}
