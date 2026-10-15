import { useSyncExternalStore } from "react";
import type { Extension } from "@codemirror/state";

/**
 * A kind of file — the one place everything about it is said: what it's
 * called (Properties' "Type", New Item), which files are it, its icon (the
 * explorer, tabs), its MIME type and its syntax highlighting. Editors name
 * the types they open (`EditorProvider.fileTypes`) instead of repeating
 * extensions. Products add their own with `registerFileType`.
 */
export interface FileType {
  /** Stable, unique (`"rust"`, `"yog.yui"`). */
  id: string;
  /** What it's called — English, shown through `t` (`"Rust"`, `"PNG Image"`). */
  name: string;
  /** Lowercase extensions without the dot. */
  extensions?: readonly string[];
  /** Finer than extensions (only `lang/*.json`) — checked on the path; with both set, both must hold. Such a type comes before plain extension ones. */
  match?: (path: string) => boolean;
  /** `Icon` name for its files — default the generic file. */
  icon?: string;
  /** Its MIME type, for a blob made of its bytes. */
  mime?: string;
  /** Syntax highlighting for its text (built once). */
  language?: Extension;
}

const types: FileType[] = [];
const subscribers = new Set<() => void>();
let revision = 0;

function changed() {
  revision++;
  subscribers.forEach((notify) => notify());
}

/** Adds a kind of file — returns its removal. A later registration wins over an earlier one for the same files. */
export function registerFileType(type: FileType): () => void {
  types.push(type);
  changed();
  return () => {
    const index = types.indexOf(type);
    if (index < 0) return;
    types.splice(index, 1);
    changed();
  };
}

/** Adds several kinds of file at once. */
export function registerFileTypes(list: readonly FileType[]): () => void {
  const removals = list.map(registerFileType);
  return () => removals.forEach((remove) => remove());
}

/** The file name of `path` (last segment, either separator). */
export function fileNameOf(path: string): string {
  return path.split(/[/\\]/).pop() ?? path;
}

/** The lowercased extension of `path` without the dot, or `""` (a leading dot is a name: `.gitignore`). */
export function extensionOf(path: string): string {
  const name = fileNameOf(path);
  const dot = name.lastIndexOf(".");
  return dot > 0 ? name.slice(dot + 1).toLowerCase() : "";
}

/** Every kind `path` is, most specific first: matched types, then by extension; later registrations first. */
export function fileTypesOf(path: string): FileType[] {
  const ext = extensionOf(path);
  const fits = (t: FileType) => (!t.extensions || t.extensions.includes(ext)) && (!t.match || t.match(path)) && (!!t.extensions || !!t.match);
  const found = types.filter(fits).reverse();
  return [...found.filter((t) => t.match), ...found.filter((t) => !t.match)];
}

/** What kind of file `path` is, if any is registered. */
export function fileTypeOf(path: string): FileType | undefined {
  return fileTypesOf(path)[0];
}

export function fileTypeById(id: string): FileType | undefined {
  for (let i = types.length - 1; i >= 0; i--) if (types[i].id === id) return types[i];
  return undefined;
}

export function subscribeFileTypes(notify: () => void): () => void {
  subscribers.add(notify);
  return () => {
    subscribers.delete(notify);
  };
}

/** Re-renders the caller when kinds of file are registered or removed; returns `fileTypeOf`. */
export function useFileType(path: string): FileType | undefined {
  useSyncExternalStore(subscribeFileTypes, () => revision);
  return fileTypeOf(path);
}

/** A changing number while kinds of file are added or removed (for `useSyncExternalStore`). */
export function fileTypesRevision(): number {
  return revision;
}
