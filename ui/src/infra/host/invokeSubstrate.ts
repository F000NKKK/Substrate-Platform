import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
import { plugin } from "./host.generated";

/** A command of Substrate's host plugin (`tauri-plugin-substrate`). */
export function invokeSubstrate<T>(command: string, args?: InvokeArgs): Promise<T> {
  return invoke<T>(`plugin:${plugin}|${command}`, args);
}
