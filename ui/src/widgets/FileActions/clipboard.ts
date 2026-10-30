import { useSyncExternalStore } from "react";
import type { FileActionEntry } from "./useFileActions";

type Clipboard = { entries: readonly FileActionEntry[]; operation: "copy" | "cut" } | null;
let clipboard: Clipboard = null;
const listeners = new Set<() => void>();
export function setFileClipboard(next: Clipboard) { clipboard = next; listeners.forEach(listener => listener()); }
export function useFileClipboard() {
  return useSyncExternalStore(listener => { listeners.add(listener); return () => listeners.delete(listener); }, () => clipboard);
}
