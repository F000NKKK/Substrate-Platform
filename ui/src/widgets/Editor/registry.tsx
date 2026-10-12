import type { EditorComponent } from "./EditorBase";
import { ImageEditor } from "./ImageEditor";
import { PropertiesGridEditor } from "./PropertiesGridEditor";
import {
  GroovyEditor,
  JavaEditor,
  JsonEditor,
  MarkdownEditor,
  PlainTextEditor,
  PythonEditor,
  RustEditor,
  ShellEditor,
  TomlEditor,
  XmlEditor,
  YamlEditor,
} from "./languageEditors";

/** File extension (lowercase, no dot) → the editor kind that opens it. Mirrors `infra/icons/registry.tsx`'s name → component resolution. */
const extensionEditors: Record<string, EditorComponent> = {
  json: JsonEditor,
  rs: RustEditor,
  xml: XmlEditor,
  java: JavaEditor,
  md: MarkdownEditor,
  markdown: MarkdownEditor,
  toml: TomlEditor,
  yml: YamlEditor,
  yaml: YamlEditor,
  sh: ShellEditor,
  bash: ShellEditor,
  properties: PropertiesGridEditor,
  gradle: GroovyEditor,
  groovy: GroovyEditor,
  py: PythonEditor,
  png: ImageEditor,
  jpg: ImageEditor,
  jpeg: ImageEditor,
  gif: ImageEditor,
  webp: ImageEditor,
  bmp: ImageEditor,
  ico: ImageEditor,
};

/** Editors that render something other than text and ignore `EditorProps.content` — see `isBinaryFile`. */
const binaryEditors: ReadonlySet<EditorComponent> = new Set<EditorComponent>([ImageEditor]);

/** Resolves which editor opens `filename` — falls back to plain, unhighlighted text for anything unrecognized rather than refusing to open it. */
export function resolveEditor(filename: string): EditorComponent {
  const ext = filename.split(".").pop()?.toLowerCase() ?? "";
  return extensionEditors[ext] ?? PlainTextEditor;
}

/**
 * Whether `filename`'s editor works on the file's raw bytes rather than its
 * text (e.g. `ImageEditor`) — a host should skip reading such a file as
 * UTF-8 text before opening it (that read fails for binary content) and pass
 * an empty `content` instead.
 */
export function isBinaryFile(filename: string): boolean {
  return binaryEditors.has(resolveEditor(filename));
}
