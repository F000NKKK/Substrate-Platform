import { useEffect, useRef, type CSSProperties } from "react";
import { EditorState, type Extension } from "@codemirror/state";
import { MergeView, unifiedMergeView } from "@codemirror/merge";
import { basicSetup, EditorView } from "codemirror";
import { useTheme } from "../../infra/theme";
import { useLocale } from "../../infra/i18n";
import { resolveEditorTheme } from "../Editor/code/editorThemes";
import { languageForPath } from "../Editor/code/languageEditors";
import { editorPhrases } from "../Editor/registry/editorPhrases";
import "../Editor/code/TextEditor.css";
import "./DiffEditor.css";

export type DiffLayout = "sideBySide" | "inline";

export interface DiffEditorProps {
  /** The older text — the left side, or the removed lines inline. */
  original: string;
  /** The newer text — the right side, or the document itself inline. */
  modified: string;
  /** Picks the syntax highlighting (by extension). */
  path?: string;
  /** Default `"sideBySide"`. */
  layout?: DiffLayout;
  /** Folds long stretches of unchanged lines down to a clickable bar (default `true`). */
  collapseUnchanged?: boolean;
}

const COLLAPSE = { margin: 3, minSize: 6 };

/**
 * A read-only diff of two texts — Visual Studio's diff viewer: side by side
 * with the changes lined up, or inline with removed lines interleaved.
 * Colors, font and tab size follow the editor's theme and preferences like
 * any `TextEditor`. Purely presentational: the host decides what's being
 * compared (a file against its last commit, two commits, ...).
 */
export function DiffEditor({ original, modified, path, layout = "sideBySide", collapseUnchanged = true }: DiffEditorProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const { editorColors, editorPreferences } = useTheme();
  const locale = useLocale();

  useEffect(() => {
    const parent = containerRef.current;
    if (!parent) return;
    const language = path ? languageForPath(path) : undefined;
    const extensions: Extension[] = [
      basicSetup,
      EditorView.editable.of(false),
      EditorState.readOnly.of(true),
      EditorState.tabSize.of(editorPreferences.tabSize),
      editorPreferences.wordWrap ? EditorView.lineWrapping : [],
      resolveEditorTheme(editorColors),
      editorPhrases(),
      language ?? [],
    ];
    const collapse = collapseUnchanged ? COLLAPSE : undefined;
    if (layout === "inline") {
      const view = new EditorView({
        parent,
        state: EditorState.create({
          doc: modified,
          extensions: [...extensions, unifiedMergeView({ original, mergeControls: false, collapseUnchanged: collapse })],
        }),
      });
      return () => view.destroy();
    }
    const merge = new MergeView({
      parent,
      a: { doc: original, extensions },
      b: { doc: modified, extensions },
      collapseUnchanged: collapse,
    });
    return () => merge.destroy();
  }, [original, modified, path, layout, collapseUnchanged, editorColors, editorPreferences.tabSize, editorPreferences.wordWrap, locale]);

  const fontStyle = {
    "--sp-editor-font-size": `${editorPreferences.fontSize}px`,
    "--sp-editor-font-family": editorPreferences.fontFamily || undefined,
  } as CSSProperties;

  return <div ref={containerRef} className="sp-text-editor sp-diff-editor" style={fontStyle} />;
}
