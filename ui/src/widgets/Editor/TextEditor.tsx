import { useEffect, useMemo, useRef, type RefObject } from "react";
import { Compartment, EditorState, type Extension } from "@codemirror/state";
import { basicSetup, EditorView } from "codemirror";
import { useTheme } from "../../infra/theme";
import type { EditorProps } from "./EditorBase";
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
 * position) — each open file tab mounts its own `TextEditor` instance, so
 * there's never a need to re-point one instance at a different file's
 * content.
 *
 * Deliberately knows nothing about breakpoints or any other code-editing
 * concern beyond plain text + syntax highlighting — see `CodeEditor`, which
 * composes this via `extensions`/`onViewReady` rather than this component
 * growing that logic itself.
 */
export function TextEditor({ content, onChange, language, readOnly, extensions: extraExtensions, onViewReady }: TextEditorProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const viewRef = useRef<EditorView | null>(null);
  const onChangeRef = useRef(onChange);
  onChangeRef.current = onChange;
  const onViewReadyRef = useRef(onViewReady);
  onViewReadyRef.current = onViewReady;
  const { editorColors } = useTheme();

  // One compartment per prop that can change after mount — each is swapped
  // in place via `reconfigure` rather than recreating the view, which would
  // throw away undo history, selection and scroll position.
  const compartments = useMemo(
    () => ({ language: new Compartment(), readOnly: new Compartment(), theme: new Compartment(), extra: new Compartment() }),
    []
  );

  useEffect(() => {
    if (!containerRef.current) return;

    const extensions: Extension[] = [
      basicSetup,
      EditorView.lineWrapping,
      compartments.theme.of(resolveEditorTheme(editorColors)),
      compartments.language.of(language ?? []),
      compartments.readOnly.of(readOnlyExtension(readOnly)),
      compartments.extra.of(extraExtensions ?? []),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) onChangeRef.current(update.state.doc.toString());
      }),
    ];

    const view = new EditorView({
      state: EditorState.create({ doc: content, extensions }),
      parent: containerRef.current,
    });
    viewRef.current = view;
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
  useReconfigure(viewRef, compartments.extra, extraExtensions ?? EMPTY_EXTENSIONS);

  return <div ref={containerRef} className="sp-text-editor" />;
}

const EMPTY_EXTENSIONS: Extension[] = [];

function readOnlyExtension(readOnly: boolean | undefined): Extension {
  return readOnly ? [EditorView.editable.of(false), EditorState.readOnly.of(true)] : [];
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
