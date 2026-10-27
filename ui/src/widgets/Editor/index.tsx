export type { EditorProps, EditorComponent } from "./registry/EditorBase";
export { TextEditor } from "./code/TextEditor";
export type { TextEditorProps } from "./code/TextEditor";
export { CodeEditor } from "./code/CodeEditor";
export type { CodeEditorProps } from "./code/CodeEditor";
export type { EditorPointRequest, EditorHoverTip, EditorHoverProvider } from "./code/editorPointer";
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
} from "./code/languageEditors";
export type { XmlSchema } from "./code/languageEditors";
export { editorSyncListener, editorApplyText, editorRevealRange, editorSetLinkedRange, linkedRangeHighlight } from "./code/editorSync";
export type { EditorSyncUpdate } from "./code/editorSync";
export type { EditorView } from "@codemirror/view";
export { changeGutter, setChangeBaseline } from "./gutters/changeGutter";
export { ImageEditor } from "./views/ImageEditor";
export type { ImageEditorProps } from "./views/ImageEditor";
export { PropertiesGridEditor } from "./views/PropertiesGridEditor";
export { isBinaryFile, registerEditorProvider, editorProvidersFor, useEditorProviders, generalEditorProviders, editorProviderById, editorProviderIcon, EDITOR_KIND } from "./registry/registry";
export type { EditorKind, EditorProvider } from "./registry/registry";
export { MultiViewEditor } from "./views/MultiViewEditor";
export { MarkdownPreview, MarkdownSplitEditor, setMarkdownLinkHandler } from "./markdown/MarkdownPreview";
export type { MarkdownPreviewProps, MarkdownLinkHandler } from "./markdown/MarkdownPreview";
export { renderMarkdown } from "./markdown/markdownRender";
export type { MultiViewEditorProps } from "./views/MultiViewEditor";
export { getEditorView, useEditorView } from "./registry/editorRegistry";
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
} from "./code/editorCommands";
export { registerDocumentCommands, useDocumentCommands, notifyDocumentCommands } from "./code/documentCommands";
export type { DocumentCommands } from "./code/documentCommands";
