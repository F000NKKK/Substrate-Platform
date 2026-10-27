import { useSyncExternalStore } from "react";
import type { DapSession } from "./dapSession";
import type { DebugState } from "./debugState";

/** The debug session's state, re-read on every change. */
export function useDebugState(session: DapSession): DebugState {
  return useSyncExternalStore(session.subscribe, session.snapshot);
}
