import { useEffect, useRef } from "react";
import { EditorState, type Extension } from "@codemirror/state";
import { basicSetup, EditorView } from "codemirror";
import { useTheme } from "../../infra/theme";
import { breakpointGutter, syncBreakpoints } from "./breakpointGutter";
import type { EditorProps } from "./EditorBase";
import { resolveEditorTheme } from "./editorThemes";
import "./TextEditor.css";

export interface TextEditorProps extends EditorProps {
  /** CodeMirror language support for this file kind — omit for plain, unhighlighted text. Every concrete language editor (JsonEditor, RustEditor, ...) is just this component composed with a fixed `language`. */
  language?: Extension;
  /** Lines (1-indexed) currently marked as breakpoints — omit entirely to skip the gutter altogether (most editors don't need one; only a debugger-aware host does). Updates without recreating the view/losing undo history. */
  breakpoints?: ReadonlySet<number>;
  /** Called when the user clicks the breakpoint gutter on a line. Required together with `breakpoints` to actually render the gutter — the host owns what "toggling" means (persistence, calling a debugger backend, ...). */
  onToggleBreakpoint?: (line: number) => void;
}

/**
 * The base text-editing implementation shared by every language-specific
 * editor — a CodeMirror 6 instance. `content` seeds the document once at
 * mount; further changes flow *out* via `onChange` as the user types, never
 * back in as a prop (re-syncing from outside would fight the user's cursor
 * position) — each open file tab mounts its own `TextEditor` instance, so
 * there's never a need to re-point one instance at a different file's
 * content.
 */
export function TextEditor({ content, onChange, language, readOnly }: TextEditorProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const onChangeRef = useRef(onChange);
  onChangeRef.current = onChange;
  const { editorColors } = useTheme();

  useEffect(() => {
    if (!containerRef.current) return;

    const extensions: Extension[] = [basicSetup, EditorView.lineWrapping, resolveEditorTheme(editorColors)];
    if (language) extensions.push(language);
    if (readOnly) extensions.push(EditorView.editable.of(false));
    extensions.push(
      EditorView.updateListener.of((update) => {
        if (update.docChanged) onChangeRef.current(update.state.doc.toString());
      })
    );

    const view = new EditorView({
      state: EditorState.create({ doc: content, extensions }),
      parent: containerRef.current,
    });

    return () => view.destroy();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [language, readOnly, editorColors]);

  return <div ref={containerRef} className="sp-text-editor" />;
}
