import { useEffect, useMemo, useRef } from "react";
import type { EditorView } from "@codemirror/view";
import { breakpointGutter, syncBreakpoints } from "./breakpointGutter";
import { TextEditor, type TextEditorProps } from "./TextEditor";

/**
 * A *code* editor's contract — everything `TextEditorProps` has, plus
 * breakpoint gutter support. Deliberately its own component (not just
 * `TextEditor` re-typed): most language editors built on `TextEditor`
 * (`JsonEditor`, `YamlEditor`, ...) are data/config/prose formats with no
 * notion of a breakpoint. Only genuine programming-language editors
 * (`RustEditor`, `JavaEditor`, `PythonEditor`, `GroovyEditor`,
 * `ShellEditor`) compose `CodeEditor` instead of `TextEditor` directly, so
 * only they can be passed `breakpoints` at all — a `JsonEditor` caller gets
 * a compile error trying to, exactly as it should.
 */
export interface CodeEditorProps extends TextEditorProps {
  /** Lines (1-indexed) with a breakpoint set. Omit entirely for no gutter. */
  breakpoints?: ReadonlySet<number>;
  /** Called when the user toggles a breakpoint via the gutter — see `breakpoints`. */
  onToggleBreakpoint?: (line: number) => void;
}

/**
 * Composes `TextEditor` via its generic `extensions`/`onViewReady` escape
 * hatches to add a breakpoint gutter, instead of `TextEditor` itself
 * knowing anything about breakpoints — keeps the two concerns (plain text
 * editing vs. debugger-aware code editing) in separate components rather
 * than one growing conditional logic for the other.
 */
export function CodeEditor({ breakpoints, onToggleBreakpoint, ...rest }: CodeEditorProps) {
  const viewRef = useRef<EditorView | null>(null);
  const onToggleRef = useRef(onToggleBreakpoint);
  onToggleRef.current = onToggleBreakpoint;

  const extensions = useMemo(
    () => (onToggleBreakpoint ? [breakpointGutter((line) => onToggleRef.current?.(line))] : []),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [!!onToggleBreakpoint]
  );

  // Separate from `TextEditor`'s own recreate-on-language/readOnly/theme
  // effect on purpose: updating which lines have a breakpoint must not
  // recreate the view (that would drop undo history and cursor position
  // every time a breakpoint is toggled, including from outside this editor
  // instance — e.g. a debug panel's own toggle).
  useEffect(() => {
    if (viewRef.current && breakpoints) syncBreakpoints(viewRef.current, breakpoints);
  }, [breakpoints]);

  return (
    <TextEditor
      {...rest}
      extensions={extensions}
      onViewReady={(view) => {
        viewRef.current = view;
        if (breakpoints) syncBreakpoints(view, breakpoints);
      }}
    />
  );
}
