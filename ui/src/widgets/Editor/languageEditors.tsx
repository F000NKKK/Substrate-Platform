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
import { TextEditor, type CodeEditorProps, type TextEditorProps } from "./TextEditor";

// Every one of these "subclasses" `TextEditor` the same way a language-aware
// editor would extend a base text-editing class in an OOP hierarchy — fixed
// language support, everything else (content sync, read-only, styling)
// inherited unchanged. New languages join by adding one more of these, not by
// touching `TextEditor` itself.
//
// Only `RustEditor` takes `CodeEditorProps` (breakpoints) — the rest take
// plain `TextEditorProps`: a JSON/YAML/TOML/... file has no notion of a
// breakpoint, so their own contract simply doesn't offer one, and a caller
// can't pass one by mistake.

export function PlainTextEditor(props: TextEditorProps) {
  return <TextEditor {...props} />;
}

export function JsonEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={json()} />;
}

export function RustEditor(props: CodeEditorProps) {
  return <TextEditor {...props} language={rust()} />;
}

export function XmlEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={xml()} />;
}

export function JavaEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={java()} />;
}

export function MarkdownEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={markdown()} />;
}

export function TomlEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={StreamLanguage.define(toml)} />;
}

export function YamlEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={StreamLanguage.define(yaml)} />;
}

export function ShellEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={StreamLanguage.define(shell)} />;
}

export function PropertiesTextEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={StreamLanguage.define(properties)} />;
}

export function GroovyEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={StreamLanguage.define(groovy)} />;
}

export function PythonEditor(props: TextEditorProps) {
  return <TextEditor {...props} language={StreamLanguage.define(python)} />;
}
