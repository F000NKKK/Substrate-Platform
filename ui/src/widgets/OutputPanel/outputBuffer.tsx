import { listen } from "@tauri-apps/api/event";
import { isBackendMessage, messageText, type BackendMessage } from "../../infra/i18n";

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
/** Lines ever dropped from the front of each buffer (cap trimming or clear) — `start + index` is a stable per-line id. */
const dropped = new Map<string, number>();
const subscribers = new Map<string, Set<() => void>>();
const listening = new Set<string>();

function ensureListening(eventName: string, maxLines: number) {
  if (listening.has(eventName)) return;
  listening.add(eventName);
  // A line is text, or a backend message — shown in the user's language.
  listen<string | BackendMessage>(eventName, (event) => appendOutputLine(eventName, isBackendMessage(event.payload) ? messageText(event.payload) : event.payload, maxLines));
}

/** Appends one line to `eventName`'s buffer (dropping the oldest beyond `maxLines`) and tells its subscribers. */
export function appendOutputLine(eventName: string, line: string, maxLines: number): void {
  const buf = buffers.get(eventName) ?? [];
  buf.push(line);
  if (buf.length > maxLines) {
    const excess = buf.length - maxLines;
    buf.splice(0, excess);
    dropped.set(eventName, (dropped.get(eventName) ?? 0) + excess);
  }
  buffers.set(eventName, buf);
  subscribers.get(eventName)?.forEach((notify) => notify());
}

/** Starts buffering `eventName` now, before any `OutputPanel` shows it — for a source the user may switch to later (lines emitted before the first subscribe are otherwise never seen). */
export function startOutputBuffer(eventName: string, maxLines = MAX_LINES_DEFAULT): void {
  ensureListening(eventName, maxLines);
}

export function getOutputBuffer(eventName: string): string[] {
  return buffers.get(eventName) ?? [];
}

/** Absolute index of `getOutputBuffer(eventName)[0]` since the app started — stable row keys across trimming. */
export function getOutputBufferStart(eventName: string): number {
  return dropped.get(eventName) ?? 0;
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
  dropped.set(eventName, getOutputBufferStart(eventName) + getOutputBuffer(eventName).length);
  buffers.set(eventName, []);
  subscribers.get(eventName)?.forEach((notify) => notify());
}

export { MAX_LINES_DEFAULT };
