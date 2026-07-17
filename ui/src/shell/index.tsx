export { PlatformShell } from "./PlatformShell";
export type { PlatformShellProps } from "./types";
// The dock/panel/DnD engine itself lives in "../dock" — re-exported here so
// the package's public API is unchanged; import directly from "../dock" for
// engine-level work that isn't about composing the top-level shell.
export * from "../dock";
