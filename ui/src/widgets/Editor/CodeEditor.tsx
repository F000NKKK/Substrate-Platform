import { useEffect, useMemo, useRef, useState, type MouseEvent as ReactMouseEvent, type ReactNode } from "react";
import { createPortal } from "react-dom";
import { EditorSelection } from "@codemirror/state";
import type { EditorView } from "@codemirror/view";
import { t } from "../../infra/i18n";
import { ContextMenu, useContextMenu, type ContextMenuItem } from "../ContextMenu";
import { breakpointGutter, syncBreakpoints, syncExecutionLine } from "./breakpointGutter";
import { editorCopy, editorCut, editorHasSelection, editorPaste } from "./editorCommands";
import { hostHoverTips, pointRequest, type EditorHoverProvider, type EditorPointRequest, type HoverPortals } from "./editorPointer";
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
  /** The line (1-indexed) a paused debugger is stopped at in this file — drawn as an arrow in the breakpoint gutter plus a highlighted line, and scrolled into view. Needs `onToggleBreakpoint` (the gutter). */
  executionLine?: number | null;
  /**
   * Asked whenever the mouse rests over the text — return a popup to show
   * there (a debugger's DataTip), or `null` for none. Read on every hover,
   * so it can change freely without reconfiguring the editor.
   */
  hoverTip?: EditorHoverProvider;
  /**
   * Host items for the editor's right-click menu at the clicked point (the
   * cursor moves there first unless the click lands in the selection). When
   * it returns any, the menu shows them above Cut/Copy/Paste; when it
   * returns none — or is omitted — the editor keeps no menu of its own.
   */
  contextMenu?: (request: EditorPointRequest) => ContextMenuItem[];
}

/**
 * Composes `TextEditor` via its generic `extensions`/`onViewReady` escape
 * hatches to add a breakpoint gutter, instead of `TextEditor` itself
 * knowing anything about breakpoints — keeps the two concerns (plain text
 * editing vs. debugger-aware code editing) in separate components rather
 * than one growing conditional logic for the other.
 */
export function CodeEditor({
  breakpoints,
  onToggleBreakpoint,
  executionLine = null,
  hoverTip,
  contextMenu,
  extensions: extraExtensions,
  ...rest
}: CodeEditorProps) {
  const viewRef = useRef<EditorView | null>(null);
  const onToggleRef = useRef(onToggleBreakpoint);
  onToggleRef.current = onToggleBreakpoint;
  const hoverTipRef = useRef(hoverTip);
  hoverTipRef.current = hoverTip;
  const contextMenuRef = useRef(contextMenu);
  contextMenuRef.current = contextMenu;

  // Hover popups' React content, keyed by the tooltip element CodeMirror
  // created for it — rendered below as portals, so the content lives in
  // this component's tree and sees the host's contexts.
  const [tips, setTips] = useState<ReadonlyMap<HTMLElement, { key: number; content: ReactNode }>>(() => new Map());
  const nextTip = useRef(0);
  const portals = useMemo<HoverPortals>(
    () => ({
      mount: (dom, content) => {
        const key = nextTip.current++;
        setTips((m) => new Map(m).set(dom, { key, content }));
      },
      unmount: (dom) =>
        setTips((m) => {
          const next = new Map(m);
          next.delete(dom);
          return next;
        }),
    }),
    []
  );

  const extensions = useMemo(
    () => [
      ...(onToggleBreakpoint ? [breakpointGutter((line) => onToggleRef.current?.(line))] : []),
      ...(hoverTip ? [hostHoverTips(() => hoverTipRef.current, portals)] : []),
      ...(extraExtensions ?? []),
    ],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [!!onToggleBreakpoint, !!hoverTip, extraExtensions]
  );

  const menu = useContextMenu<ContextMenuItem[]>();
  function openContextMenu(e: ReactMouseEvent) {
    const view = viewRef.current;
    const host = contextMenuRef.current;
    if (!view || !host) return;
    const pos = view.posAtCoords({ x: e.clientX, y: e.clientY });
    if (pos === null) return;
    const inSelection = view.state.selection.ranges.some((r) => !r.empty && r.from <= pos && pos <= r.to);
    if (!inSelection) view.dispatch({ selection: EditorSelection.cursor(pos) });
    const items = host(pointRequest(view, pos));
    if (items.length === 0) return;
    const readOnly = view.state.readOnly;
    const selected = editorHasSelection(view);
    menu.openAtPoint(
      [
        ...items,
        { label: t("Cut"), shortcut: "Ctrl+X", disabled: readOnly || !selected, separatorBefore: true, onSelect: () => void editorCut(view) },
        { label: t("Copy"), shortcut: "Ctrl+C", disabled: !selected, onSelect: () => void editorCopy(view) },
        { label: t("Paste"), shortcut: "Ctrl+V", disabled: readOnly, onSelect: () => void editorPaste(view) },
      ],
      e
    );
  }

  // Breakpoints are synced as plain transactions into the live view, not
  // passed through `extensions`: that would reconfigure the gutter on every
  // toggle (including ones from outside this editor instance — e.g. a debug
  // panel's own toggle) for what is really just a state update.
  useEffect(() => {
    if (viewRef.current && breakpoints) syncBreakpoints(viewRef.current, breakpoints);
  }, [breakpoints]);

  useEffect(() => {
    if (viewRef.current && onToggleRef.current) syncExecutionLine(viewRef.current, executionLine);
  }, [executionLine]);

  return (
    <div style={{ display: "contents" }} onContextMenu={openContextMenu}>
      <TextEditor
        {...rest}
        extensions={extensions}
        onViewReady={(view) => {
          viewRef.current = view;
          if (breakpoints) syncBreakpoints(view, breakpoints);
          if (onToggleRef.current && executionLine !== null) syncExecutionLine(view, executionLine);
        }}
      />
      {[...tips].map(([dom, tip]) => createPortal(tip.content, dom, tip.key))}
      <ContextMenu target={menu.target} items={menu.target?.node ?? []} onClose={menu.close} />
    </div>
  );
}
