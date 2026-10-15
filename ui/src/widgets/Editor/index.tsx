export type { EditorProps, EditorComponent } from "./EditorBase";
export { TextEditor } from "./TextEditor";
export type { TextEditorProps } from "./TextEditor";
export { CodeEditor } from "./CodeEditor";
export type { CodeEditorProps } from "./CodeEditor";
export type { EditorPointRequest, EditorHoverTip, EditorHoverProvider } from "./editorPointer";
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
  languageForPath,
  jsonLanguage,
  xmlLanguage,
  xmlSchemaLanguage,
} from "./languageEditors";
export type { XmlSchema } from "./languageEditors";
export { editorSyncListener, editorApplyText, editorRevealRange, editorSetLinkedRange, linkedRangeHighlight } from "./editorSync";
export type { EditorSyncUpdate } from "./editorSync";
export type { EditorView } from "@codemirror/view";
export { changeGutter, setChangeBaseline } from "./changeGutter";
export { ImageEditor } from "./ImageEditor";
export type { ImageEditorProps } from "./ImageEditor";
export { PropertiesGridEditor } from "./PropertiesGridEditor";
export { isBinaryFile, registerEditorProvider, editorProvidersFor, useEditorProviders, generalEditorProviders, editorProviderById, editorProviderIcon } from "./registry";
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
export { registerDocumentCommands, useDocumentCommands, notifyDocumentCommands } from "./documentCommands";
export type { DocumentCommands } from "./documentCommands";
