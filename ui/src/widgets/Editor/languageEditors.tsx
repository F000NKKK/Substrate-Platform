import { json } from "@codemirror/lang-json";
import { rust } from "@codemirror/lang-rust";
import { xml } from "@codemirror/lang-xml";
import { java } from "@codemirror/lang-java";
import { markdown } from "@codemirror/lang-markdown";
import { StreamLanguage } from "@codemirror/language";
import { toml } from "@codemirror/legacy-modes/mode/toml";
import { yaml } from "@codemirror/legacy-modes/mode/yaml";
import { shell } from "@codemirror/legacy-modes/mode/shell";
import { properties } from "@codemirror/legacy-modes/mode/properties";
import { groovy } from "@codemirror/legacy-modes/mode/groovy";
import { python } from "@codemirror/legacy-modes/mode/python";
import { TextEditor } from "./TextEditor";
import type { EditorProps } from "./EditorBase";

// Every one of these "subclasses" `TextEditor` the same way a language-aware
// editor would extend a base text-editing class in an OOP hierarchy — fixed
// language support, everything else (content sync, read-only, styling)
// inherited unchanged. New languages join by adding one more of these, not by
// touching `TextEditor` itself.

export function PlainTextEditor(props: EditorProps) {
  return <TextEditor {...props} />;
}

export function JsonEditor(props: EditorProps) {
  return <TextEditor {...props} language={json()} />;
}

export function RustEditor(props: EditorProps) {
  return <TextEditor {...props} language={rust()} />;
}

export function XmlEditor(props: EditorProps) {
  return <TextEditor {...props} language={xml()} />;
}

export function JavaEditor(props: EditorProps) {
  return <TextEditor {...props} language={java()} />;
}

export function MarkdownEditor(props: EditorProps) {
  return <TextEditor {...props} language={markdown()} />;
}

export function TomlEditor(props: EditorProps) {
  return <TextEditor {...props} language={StreamLanguage.define(toml)} />;
}

export function YamlEditor(props: EditorProps) {
  return <TextEditor {...props} language={StreamLanguage.define(yaml)} />;
}

export function ShellEditor(props: EditorProps) {
  return <TextEditor {...props} language={StreamLanguage.define(shell)} />;
}

export function PropertiesTextEditor(props: EditorProps) {
  return <TextEditor {...props} language={StreamLanguage.define(properties)} />;
}

export function GroovyEditor(props: EditorProps) {
  return <TextEditor {...props} language={StreamLanguage.define(groovy)} />;
}

export function PythonEditor(props: EditorProps) {
  return <TextEditor {...props} language={StreamLanguage.define(python)} />;
}
