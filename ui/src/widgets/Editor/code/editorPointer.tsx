import type { ReactNode } from "react";
import type { Extension } from "@codemirror/state";
import { EditorView, hoverTooltip, type Tooltip } from "@codemirror/view";

/**
 * A point in a code editor the host is asked about — where the mouse
 * rests (`hoverTip`) or was right-clicked (`contextMenu`). Carries the
 * point's whole line so the host can read the language-specific token or
 * expression there without reaching into CodeMirror.
 */
export interface EditorPointRequest {
  /** Document offset of the point. */
  pos: number;
  /** 1-indexed line of the point. */
  line: number;
  /** Document offset where that line starts — `pos - lineStart` is the column. */
  lineStart: number;
  lineText: string;
  /** The selected text, when the point lies inside a non-empty selection. */
  selection: { text: string; from: number; to: number } | null;
}

/** What a `hoverTip` provider shows: `content` in a popup anchored under document range `from`..`to`. */
export interface EditorHoverTip {
  from: number;
  to: number;
  content: ReactNode;
}

export type EditorHoverProvider = (request: EditorPointRequest) => EditorHoverTip | null | Promise<EditorHoverTip | null>;

/** Where a tooltip's React content is portaled into — `CodeEditor` renders the portals, so the content keeps the app's React context. */
export interface HoverPortals {
  mount(dom: HTMLElement, content: ReactNode): void;
  unmount(dom: HTMLElement): void;
}

export function pointRequest(view: EditorView, pos: number): EditorPointRequest {
  const { state } = view;
  const line = state.doc.lineAt(pos);
  const range = state.selection.ranges.find((r) => !r.empty && r.from <= pos && pos <= r.to) ?? null;
  return {
    pos,
    line: line.number,
    lineStart: line.from,
    lineText: line.text,
    selection: range && { text: state.sliceDoc(range.from, range.to), from: range.from, to: range.to },
  };
}

/**
 * Hover popups whose content the host supplies (Visual Studio's DataTips
 * over a paused debugger's variables) — the content is React, portaled
 * into CodeMirror's tooltip element through `portals`.
 */
export function hostHoverTips(provider: () => EditorHoverProvider | undefined, portals: HoverPortals): Extension {
  return [
    hoverTooltip(
      async (view, pos): Promise<Tooltip | null> => {
        const get = provider();
        if (!get) return null;
        const tip = await get(pointRequest(view, pos));
        if (!tip) return null;
        return {
          pos: tip.from,
          end: tip.to,
          above: false,
          create() {
            const dom = document.createElement("div");
            dom.className = "sp-hover-tip";
            return {
              dom,
              mount: () => portals.mount(dom, tip.content),
              destroy: () => portals.unmount(dom),
            };
          },
        };
      },
      { hoverTime: 350 },
    ),
    EditorView.theme({
      ".cm-tooltip.cm-tooltip-hover": { border: "none", background: "transparent" },
    }),
  ];
}
