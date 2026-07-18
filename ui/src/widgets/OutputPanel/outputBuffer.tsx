import { listen } from "@tauri-apps/api/event";

/**
 * Keeps each event name's accumulated lines outside any component's
 * lifecycle — a tool-window panel only stays *mounted* while it's pinned or
 * the active flyout (see `ToolWindowDock`), so a plain `useState` buffer
 * inside `OutputPanel` itself gets wiped every time the panel is hidden and
 * reopened. Listening here instead (once per event name, for the life of
 * the app) means the buffer keeps accumulating regardless of whether any
 * `OutputPanel` is currently mounted to look at it.
 */
const MAX_LINES_DEFAULT = 10_000;

const buffers = new Map<string, string[]>();
const subscribers = new Map<string, Set<() => void>>();
const listening = new Set<string>();

function ensureListening(eventName: string, maxLines: number) {
  if (listening.has(eventName)) return;
  listening.add(eventName);
  listen<string>(eventName, (event) => {
    const buf = buffers.get(eventName) ?? [];
    buf.push(event.payload);
    if (buf.length > maxLines) buf.splice(0, buf.length - maxLines);
    buffers.set(eventName, buf);
    subscribers.get(eventName)?.forEach((notify) => notify());
  });
}

export function getOutputBuffer(eventName: string): string[] {
  return buffers.get(eventName) ?? [];
}

/** Subscribes to new lines for `eventName`, starting the underlying Tauri listener the first time anything asks for this event name. Returns an unsubscribe function. */
export function subscribeOutput(eventName: string, maxLines: number, onChange: () => void): () => void {
  ensureListening(eventName, maxLines);
  let set = subscribers.get(eventName);
  if (!set) {
    set = new Set();
    subscribers.set(eventName, set);
  }
  set.add(onChange);
  return () => set!.delete(onChange);
}

export function clearOutputBuffer(eventName: string): void {
  buffers.set(eventName, []);
  subscribers.get(eventName)?.forEach((notify) => notify());
}

export { MAX_LINES_DEFAULT };
