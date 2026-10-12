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
export { resolveEditor, isBinaryFile, registerEditorProvider, editorProvidersFor, useEditorProviders, generalEditorProviders, editorProviderById, editorProviderIcon } from "./registry";
export type { EditorProvider } from "./registry";
export { MultiViewEditor } from "./MultiViewEditor";
export { MarkdownPreview, MarkdownSplitEditor, setMarkdownLinkHandler } from "./MarkdownPreview";
export type { MarkdownPreviewProps, MarkdownLinkHandler } from "./MarkdownPreview";
export { renderMarkdown } from "./markdownRender";
export type { MultiViewEditorProps } from "./MultiViewEditor";
export { getEditorView, useEditorView } from "./editorRegistry";
export {
  editorCanUndo,
  editorCanRedo,
  editorUndo,
  editorRedo,
  editorCanToggleComment,
  editorToggleComment,
  editorFind,
  editorReplace,
  editorCommentSelection,
  editorUncommentSelection,
  editorSelectAll,
  editorGoToLine,
  editorChangeCase,
  editorHasSelection,
  editorCopy,
  editorCut,
  editorPaste,
  editorSetText,
  revealLine,
} from "./editorCommands";
