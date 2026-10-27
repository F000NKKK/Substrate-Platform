// Debug adapters through the host plugin (feature `dap`): the host's
// commands, one shared session per root and adapter, the editor's extension
// for a file and the debug toolbar and panels.

export { tauriDapHost, adapterFor } from "./dapHost";
export type { AdapterInfo, DapEvent, DapEventMessage, DapHost, DapHostNames, DapOutput } from "./dapHost";
export { DapSession, dapSession, stopDapSessions } from "./dapSession";
export type { DebugFrame, DebugOutputLine, DebugScope, DebugState, DebugStatus, DebugStop, DebugVariable } from "./debugState";
export { dapExtension } from "./dapExtension";
export { useDebugState } from "./useDebugState";
export { DebugToolbar } from "./DebugToolbar";
export type { DebugToolbarProps } from "./DebugToolbar";
export { CallStackPanel } from "./CallStackPanel";
export type { CallStackPanelProps } from "./CallStackPanel";
export { VariablesPanel } from "./VariablesPanel";
