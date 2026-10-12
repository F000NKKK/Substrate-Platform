export type { EditorProps, EditorComponent } from "./EditorBase";
export { TextEditor } from "./TextEditor";
export type { TextEditorProps } from "./TextEditor";
export { CodeEditor } from "./CodeEditor";
export type { CodeEditorProps } from "./CodeEditor";
export {
  PlainTextEditor,
  JsonEditor,
  RustEditor,
  XmlEditor,
  JavaEditor,
  MarkdownEditor,
  TomlEditor,
  YamlEditor,
  ShellEditor,
  PropertiesTextEditor,
  GroovyEditor,
  PythonEditor,
} from "./languageEditors";
export { ImageEditor } from "./ImageEditor";
export type { ImageEditorProps } from "./ImageEditor";
export { PropertiesGridEditor } from "./PropertiesGridEditor";
export { resolveEditor, isBinaryFile } from "./registry";
export { getEditorView, useEditorView } from "./editorRegistry";
export {
  editorCanUndo,
  editorCanRedo,
  editorUndo,
  editorRedo,
  editorCanToggleComment,
  editorToggleComment,
  editorFind,
  editorHasSelection,
  editorCopy,
  editorCut,
  editorPaste,
  editorSetText,
  revealLine,
} from "./editorCommands";
