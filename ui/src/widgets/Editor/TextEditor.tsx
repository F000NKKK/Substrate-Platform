import { useEffect, useRef } from "react";
import { EditorState, type Extension } from "@codemirror/state";
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
  /** Called once with the live `EditorView` right after it's created — lets a composing component (that supplied `extensions`) dispatch its own transactions into it later, without `TextEditor` itself needing to know why. */
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
  const onChangeRef = useRef(onChange);
  onChangeRef.current = onChange;
  const onViewReadyRef = useRef(onViewReady);
  onViewReadyRef.current = onViewReady;
  const { editorColors } = useTheme();

  useEffect(() => {
    if (!containerRef.current) return;

    const extensions: Extension[] = [basicSetup, EditorView.lineWrapping, resolveEditorTheme(editorColors)];
    if (language) extensions.push(language);
    if (readOnly) extensions.push(EditorView.editable.of(false));
    if (extraExtensions) extensions.push(...extraExtensions);
    extensions.push(
      EditorView.updateListener.of((update) => {
        if (update.docChanged) onChangeRef.current(update.state.doc.toString());
      })
    );

    const view = new EditorView({
      state: EditorState.create({ doc: content, extensions }),
      parent: containerRef.current,
    });
    onViewReadyRef.current?.(view);

    return () => view.destroy();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [language, readOnly, editorColors, extraExtensions]);

  return <div ref={containerRef} className="sp-text-editor" />;
}
