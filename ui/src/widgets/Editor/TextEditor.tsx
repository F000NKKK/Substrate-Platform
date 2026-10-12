import { useEffect, useMemo, useRef, type CSSProperties, type RefObject } from "react";
import { Annotation, Compartment, EditorSelection, EditorState, type Extension } from "@codemirror/state";
import { indentUnit } from "@codemirror/language";
import { getSearchQuery, SearchQuery, setSearchQuery } from "@codemirror/search";
import { basicSetup, EditorView } from "codemirror";
import { useTheme, type EditorPreferences } from "../../infra/theme";
import { useStateStore } from "../../infra/state";
import type { EditorProps } from "./EditorBase";
import { minimalChange } from "./editorCommands";
import { editorKeymap } from "./editorKeymap";
import { notifyEditorUpdate, registerEditorView } from "./editorRegistry";
import { resolveEditorTheme } from "./editorThemes";
import "./TextEditor.css";

export interface TextEditorProps extends EditorProps {
  /** CodeMirror language support for this file kind — omit for plain, unhighlighted text. Every concrete language editor (JsonEditor, RustEditor, ...) is just this component composed with a fixed `language`. */
  language?: Extension;
  /** Extra CodeMirror extensions layered on top of the base set — the escape hatch a composing component (`CodeEditor`'s breakpoint gutter) uses instead of `TextEditor` knowing anything about what they're for. */
  extensions?: Extension[];
  /** Called once with the live `EditorView` right after it's created (the view lives for the component's whole lifetime — prop changes reconfigure it in place) — lets a composing component (that supplied `extensions`) dispatch its own transactions into it later, without `TextEditor` itself needing to know why. */
  onViewReady?: (view: EditorView) => void;
}

/**
 * The base text-editing implementation shared by every language-specific
 * editor — a CodeMirror 6 instance. `content` seeds the document once at
 * mount; further changes flow *out* via `onChange` as the user types, never
 * back in as a prop (re-syncing from outside would fight the user's cursor
 * position) — except when the host bumps `contentVersion` (the file changed
 * on disk), which replaces the document with `content` once. Each open file
 * tab mounts its own `TextEditor` instance, so there's never a need to
 * re-point one instance at a different file's content.
 *
 * The live view is registered under `path` (see `useEditorView`), which is
 * how a host's menus/toolbar reach the active editor; font, tab size and
 * word wrap follow the theme's `editorPreferences` live. The find/replace
 * query is shared through the current `StateStore` (`"editor.search"`), so
 * every editor — and the next session — starts with the last one used.
 *
 * Deliberately knows nothing about breakpoints or any other code-editing
 * concern beyond plain text + syntax highlighting — see `CodeEditor`, which
 * composes this via `extensions`/`onViewReady` rather than this component
 * growing that logic itself.
 */
export function TextEditor({
  path,
  content,
  contentVersion,
  onChange,
  language,
  readOnly,
  extensions: extraExtensions,
  onViewReady,
}: TextEditorProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const viewRef = useRef<EditorView | null>(null);
  const onChangeRef = useRef(onChange);
  onChangeRef.current = onChange;
  const onViewReadyRef = useRef(onViewReady);
  onViewReadyRef.current = onViewReady;
  const { editorColors, editorPreferences } = useTheme();
  const store = useStateStore();
  const storeRef = useRef(store);
  storeRef.current = store;

  // One compartment per prop that can change after mount — each is swapped
  // in place via `reconfigure` rather than recreating the view, which would
  // throw away undo history, selection and scroll position.
  const compartments = useMemo(
    () => ({
      language: new Compartment(),
      readOnly: new Compartment(),
      theme: new Compartment(),
      preferences: new Compartment(),
      extra: new Compartment(),
    }),
    []
  );

  useEffect(() => {
    if (!containerRef.current) return;

    const extensions: Extension[] = [
      basicSetup,
      editorKeymap,
      compartments.theme.of(resolveEditorTheme(editorColors)),
      compartments.preferences.of(preferencesExtension(editorPreferences)),
      compartments.language.of(language ?? []),
      compartments.readOnly.of(readOnlyExtension(readOnly)),
      compartments.extra.of(extraExtensions ?? []),
      EditorView.updateListener.of((update) => {
        const external = update.transactions.some((tr) => tr.annotation(externalContent));
        if (update.docChanged && !external) onChangeRef.current(update.state.doc.toString());
        if (update.docChanged || update.selectionSet || update.transactions.some((tr) => tr.reconfigured)) notifyEditorUpdate();
        const query = getSearchQuery(update.state);
        if (!query.eq(getSearchQuery(update.startState))) storeRef.current.set(SEARCH_STATE_KEY, toStoredQuery(query));
      }),
    ];

    const view = new EditorView({
      state: EditorState.create({ doc: content, extensions }),
      parent: containerRef.current,
    });
    viewRef.current = view;
    const savedQuery = fromStoredQuery(storeRef.current.get(SEARCH_STATE_KEY));
    if (savedQuery) view.dispatch({ effects: setSearchQuery.of(savedQuery) });
    onViewReadyRef.current?.(view);

    return () => {
      viewRef.current = null;
      view.destroy();
    };
    // Mount-only: every later prop change goes through a compartment below.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useReconfigure(viewRef, compartments.language, language ?? EMPTY_EXTENSIONS);
  useReconfigure(viewRef, compartments.readOnly, useMemo(() => readOnlyExtension(readOnly), [readOnly]));
  useReconfigure(viewRef, compartments.theme, useMemo(() => resolveEditorTheme(editorColors), [editorColors]));
  useReconfigure(viewRef, compartments.preferences, useMemo(() => preferencesExtension(editorPreferences), [editorPreferences]));
  useReconfigure(viewRef, compartments.extra, extraExtensions ?? EMPTY_EXTENSIONS);

  // Re-keyed on `path` so a renamed file's editor stays reachable under its
  // new path (the view itself lives on, undo history included).
  useEffect(() => {
    if (!viewRef.current) return;
    return registerEditorView(path, viewRef.current);
  }, [path]);

  const loadedVersion = useRef(contentVersion);
  useEffect(() => {
    const view = viewRef.current;
    if (!view || contentVersion === loadedVersion.current) return;
    loadedVersion.current = contentVersion;
    const current = view.state.doc.toString();
    if (current === content) return;
    // Only the differing middle is replaced, and the cursor stays put
    // (clamped), so a reload of a file that changed in one place doesn't
    // throw the user back to the top. Undoable, like VS Code's reload.
    const head = Math.min(view.state.selection.main.head, content.length);
    view.dispatch({
      changes: minimalChange(current, content),
      selection: EditorSelection.cursor(head),
      annotations: externalContent.of(true),
    });
  }, [contentVersion, content]);

  // Font changes resize every line — CodeMirror only notices on its next
  // measure, so ask for one.
  useEffect(() => {
    viewRef.current?.requestMeasure();
  }, [editorPreferences.fontFamily, editorPreferences.fontSize]);

  const fontStyle = {
    "--sp-editor-font-size": `${editorPreferences.fontSize}px`,
    "--sp-editor-font-family": editorPreferences.fontFamily || undefined,
  } as CSSProperties;

  return <div ref={containerRef} className="sp-text-editor" style={fontStyle} />;
}

/** Marks a transaction as a reload from outside (`contentVersion`) rather than a user edit — the update listener doesn't report it through `onChange`. */
const externalContent = Annotation.define<boolean>();

/** Tab size (and the matching indent unit) and word wrap — font family/size are CSS variables on the container instead (see TextEditor.css), since they don't need CodeMirror's involvement beyond a re-measure. */
function preferencesExtension(preferences: EditorPreferences): Extension {
  return [
    EditorState.tabSize.of(preferences.tabSize),
    indentUnit.of(" ".repeat(preferences.tabSize)),
    preferences.wordWrap ? EditorView.lineWrapping : [],
  ];
}

const EMPTY_EXTENSIONS: Extension[] = [];

function readOnlyExtension(readOnly: boolean | undefined): Extension {
  return readOnly ? [EditorView.editable.of(false), EditorState.readOnly.of(true)] : [];
}

const SEARCH_STATE_KEY = "editor.search";

interface StoredQuery {
  search: string;
  replace: string;
  caseSensitive: boolean;
  regexp: boolean;
  wholeWord: boolean;
}

function toStoredQuery(q: SearchQuery): StoredQuery {
  return { search: q.search, replace: q.replace, caseSensitive: q.caseSensitive, regexp: q.regexp, wholeWord: q.wholeWord };
}

function fromStoredQuery(value: unknown): SearchQuery | null {
  const v = value as StoredQuery | null;
  if (!v || typeof v.search !== "string" || !v.search) return null;
  return new SearchQuery({
    search: v.search,
    replace: typeof v.replace === "string" ? v.replace : "",
    caseSensitive: v.caseSensitive === true,
    regexp: v.regexp === true,
    wholeWord: v.wholeWord === true,
  });
}

/** Swaps `compartment`'s content whenever `extension`'s identity changes after mount (the mount itself already installed the initial value). */
function useReconfigure(viewRef: RefObject<EditorView | null>, compartment: Compartment, extension: Extension) {
  const mounted = useRef(false);
  useEffect(() => {
    if (!mounted.current) {
      mounted.current = true;
      return;
    }
    viewRef.current?.dispatch({ effects: compartment.reconfigure(extension) });
  }, [viewRef, compartment, extension]);
}
