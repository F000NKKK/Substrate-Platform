// `file://` URIs, as language servers name documents — the same encoding
// as the host's (`substrate_platform::lsp::uri`), so a URI the server sends
// back names the path the editor opened.

const SCHEME = "file://";

/** Characters a path keeps as they are in a URI. */
const UNRESERVED = /[A-Za-z0-9\-._~/:]/;

/** `path` (absolute) as a `file://` URI. */
export function uriFromPath(path: string): string {
  let slashed = path.replace(/\\/g, "/");
  if (!slashed.startsWith("/")) slashed = `/${slashed}`;
  let out = SCHEME;
  for (const byte of new TextEncoder().encode(slashed)) {
    const char = String.fromCharCode(byte);
    out += byte < 0x80 && UNRESERVED.test(char) ? char : `%${byte.toString(16).toUpperCase().padStart(2, "0")}`;
  }
  return out;
}

/** The path a `file://` URI names — `undefined` for another scheme or bad encoding. */
export function pathFromUri(uri: string): string | undefined {
  if (!uri.startsWith(SCHEME)) return undefined;
  let rest = uri.slice(SCHEME.length);
  if (rest.startsWith("localhost")) rest = rest.slice("localhost".length);
  if (!rest.startsWith("/")) return undefined;
  let decoded: string;
  try {
    decoded = decodeURIComponent(rest);
  } catch {
    return undefined;
  }
  return /^\/[A-Za-z]:/.test(decoded) ? decoded.slice(1).replace(/\//g, "\\") : decoded;
}
