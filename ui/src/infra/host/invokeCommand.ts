import { invoke, type InvokeArgs } from "@tauri-apps/api/core";

/**
 * A host command: its plugin and name. A product's domains generate them
 * (`substrate.<domain>.<command>` for Substrate's own) from the backend's
 * lists — never typed as strings at the call.
 */
export interface HostCommand {
  readonly plugin: string;
  readonly command: string;
}

/** Runs a host command. */
export function invokeCommand<T>({ plugin, command }: HostCommand, args?: InvokeArgs): Promise<T> {
  return invoke<T>(`plugin:${plugin}|${command}`, args);
}
