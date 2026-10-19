import { t } from "./i18n";

/** What a backend says to the user (`substrate_platform::message::Message`): an English template — the catalogs' key — and its values. */
export interface BackendMessage {
  text: string;
  args?: Record<string, string>;
}

export function isBackendMessage(value: unknown): value is BackendMessage {
  return typeof value === "object" && value !== null && typeof (value as BackendMessage).text === "string";
}

/** A backend message in the user's language. */
export function messageText(message: BackendMessage): string {
  return t(message.text, message.args);
}

/** An error object that only says what went wrong (`Error`, a service's `{ message }`). */
function hasMessage(value: unknown): value is { message: string } {
  return typeof value === "object" && value !== null && typeof (value as { message?: unknown }).message === "string";
}

/**
 * Whatever a failed call threw, as text for the user, translated: a backend
 * message, an error with a `message`, or anything else spelled out.
 */
export function errorText(err: unknown): string {
  if (isBackendMessage(err)) return messageText(err);
  if (hasMessage(err)) return t(err.message);
  return t(String(err));
}
