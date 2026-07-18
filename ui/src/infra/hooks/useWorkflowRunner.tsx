import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export interface WorkflowSummary {
  name: string;
  description?: string;
}

/** Tauri command/event names — injected so this hook stays product-agnostic, same "commands as props" shape `TerminalPanel` already uses. */
export interface WorkflowRunnerCommands {
  list?: string;
  run?: string;
  exitEvent?: string;
}

const DEFAULT_COMMANDS = { list: "workflow_list", run: "workflow_run", exitEvent: "workflow-exit" };

/**
 * Lists a project's named workflows (from its own `workflow.toml`, if it has
 * one — this is a generic capability open to any project, not gated by
 * project kind) and lets a consumer run one by name — output itself isn't
 * handled here, since `OutputPanel`'s own `eventName` prop already listens
 * for `workflow-output` lines directly; this hook only covers "what
 * workflows exist" and "is one currently running" (so a Build menu can
 * disable itself mid-run).
 */
export function useWorkflowRunner(projectRoot: string, commands: WorkflowRunnerCommands = {}) {
  const { list, run: runCommand, exitEvent } = { ...DEFAULT_COMMANDS, ...commands };
  const [workflows, setWorkflows] = useState<WorkflowSummary[]>([]);
  const [running, setRunning] = useState(false);

  useEffect(() => {
    invoke<WorkflowSummary[]>(list, { projectRoot })
      .then(setWorkflows)
      .catch(() => setWorkflows([]));
  }, [list, projectRoot]);

  useEffect(() => {
    const unlistenPromise = listen(exitEvent, () => setRunning(false));
    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, [exitEvent]);

  const run = useCallback(
    (name: string, vars: Record<string, string> = {}) => {
      setRunning(true);
      return invoke(runCommand, { projectRoot, name, vars }).catch((err) => {
        setRunning(false);
        throw err;
      });
    },
    [runCommand, projectRoot]
  );

  return { workflows, running, run };
}
