import { json } from "@codemirror/lang-json";
import { rust } from "@codemirror/lang-rust";
import { xml, type AttrSpec, type ElementSpec } from "@codemirror/lang-xml";
import { java } from "@codemirror/lang-java";
import { markdown } from "@codemirror/lang-markdown";
import { StreamLanguage } from "@codemirror/language";
import { toml } from "@codemirror/legacy-modes/mode/toml";
import { yaml } from "@codemirror/legacy-modes/mode/yaml";
import { shell } from "@codemirror/legacy-modes/mode/shell";
import { properties } from "@codemirror/legacy-modes/mode/properties";
import { groovy } from "@codemirror/legacy-modes/mode/groovy";
import { python } from "@codemirror/legacy-modes/mode/python";
import type { Extension } from "@codemirror/state";
import { CodeEditor, type CodeEditorProps } from "./CodeEditor";
import { TextEditor, type TextEditorProps } from "./TextEditor";

// Every one of these "subclasses" a base editor the same way a language-aware
// editor would extend a base text-editing class in an OOP hierarchy — fixed
// language support, everything else (content sync, read-only, styling,
// breakpoints where applicable) inherited unchanged. New languages join by
// adding one more of these, not by touching `TextEditor`/`CodeEditor`
// themselves.
//
// Genuine programming-language editors compose `CodeEditor` (breakpoint
// gutter support); data/config/prose formats compose plain `TextEditor` —
// a JSON/YAML/TOML/Markdown/XML/properties file has no notion of a
// breakpoint, so their own contract simply doesn't offer one.
//
// Every `language` extension is built exactly once at module load, not
// inline in JSX: `TextEditor` reconfigures its CodeMirror view whenever the
// `language` prop's *identity* changes, and `json()`/`rust()`/... each
// return a brand new object on every call — calling them inline would
// trigger a pointless (re-highlighting) reconfigure on every single render.

const jsonLanguage = json();
const rustLanguage = rust();
const xmlLanguage = xml();
const javaLanguage = java();
const markdownLanguage = markdown();
const tomlLanguage = StreamLanguage.define(toml);
const yamlLanguage = StreamLanguage.define(yaml);
const shellLanguage = StreamLanguage.define(shell);
const propertiesLanguage = StreamLanguage.define(properties);
const groovyLanguage = StreamLanguage.define(groovy);
const pythonLanguage = StreamLanguage.define(python);

export function PlainTextEditor(props: TextEditorProps) {
  return <TextEditor {...props} />;
}

export function JsonEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={jsonLanguage} />;
}

export function RustEditor(props: CodeEditorProps) {
  return <CodeEditor {...props} language={rustLanguage} />;
}

export function XmlEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={xmlLanguage} />;
}

export function JavaEditor(props: CodeEditorProps) {
  return <CodeEditor {...props} language={javaLanguage} />;
}

export function MarkdownEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={markdownLanguage} />;
}

export function TomlEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={tomlLanguage} />;
}

export function YamlEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={yamlLanguage} />;
}

export function ShellEditor(props: CodeEditorProps) {
  return <CodeEditor {...props} language={shellLanguage} />;
}

export function PropertiesTextEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={propertiesLanguage} />;
}

export function GroovyEditor(props: CodeEditorProps) {
  return <CodeEditor {...props} language={groovyLanguage} />;
}

export function PythonEditor(props: CodeEditorProps) {
  return <CodeEditor {...props} language={pythonLanguage} />;
}

/** An XML vocabulary — its elements and their attributes (with values to offer, for enums) — for completion. */
export interface XmlSchema {
  elements: readonly ElementSpec[];
  /** Attributes the elements refer to by name; `global: true` ones are offered on every element. */
  attributes?: readonly AttrSpec[];
}

/**
 * XML highlighting with completion of `schema`'s element and attribute names
 * and attribute values, and auto-closed tags — for an editor of one XML
 * dialect. Build it once (module level or `useMemo`), like the languages above.
 */
export function xmlSchemaLanguage(schema: XmlSchema): Extension {
  return xml({ elements: schema.elements, attributes: schema.attributes, autoCloseTags: true });
}

const LANGUAGES_BY_EXTENSION: Record<string, Extension> = {
  json: jsonLanguage,
  rs: rustLanguage,
  xml: xmlLanguage,
  svg: xmlLanguage,
  java: javaLanguage,
  md: markdownLanguage,
  markdown: markdownLanguage,
  toml: tomlLanguage,
  yml: yamlLanguage,
  yaml: yamlLanguage,
  sh: shellLanguage,
  bash: shellLanguage,
  properties: propertiesLanguage,
  gradle: groovyLanguage,
  groovy: groovyLanguage,
  py: pythonLanguage,
};

/** The syntax highlighting the language editors use for `path`'s kind of file — for a CodeMirror view outside them (a diff). `undefined` for plain text. */
export function languageForPath(path: string): Extension | undefined {
  const name = path.split(/[/\\]/).pop() ?? "";
  const ext = name.includes(".") ? name.split(".").pop()!.toLowerCase() : "";
  return LANGUAGES_BY_EXTENSION[ext];
}
