// What the IDE remembers per project and app-wide (through the host
// plugin's feature `state`), and closing the window with it saved.

export { WorkspaceProvider, addWorkspaceCollector, useGlobalState, useGlobalStateStore, useWorkspaceState } from "./WorkspaceProvider";
export { addCloseFlush, addCloseGuard, installCloseHandler } from "./windowClose";
