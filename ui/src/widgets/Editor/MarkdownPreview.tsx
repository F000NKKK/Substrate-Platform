import { useEffect, useMemo, useRef, useState, type CSSProperties, type MouseEvent } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import type { EditorView } from "@codemirror/view";
import { useTheme } from "../../infra/theme";
import { hslToCss } from "../../infra/color";
import type { EditorProps } from "./EditorBase";
import { MarkdownEditor } from "./languageEditors";
import { renderMarkdown } from "./markdownRender";
import { revealLine } from "./editorCommands";
import "katex/dist/katex.min.css";
import "./MarkdownPreview.css";

/** What a click on a link in a preview does — `target` is an absolute path (fragment dropped) for a link to a local file, the URL as written otherwise. */
export type MarkdownLinkHandler = (target: string, kind: "file" | "external") => void;

let linkHandler: MarkdownLinkHandler = (target, kind) => {
  if (kind === "external") window.open(target, "_blank");
};

/** How the host opens links clicked in a Markdown preview — local files in its editor, URLs in the browser. */
export function setMarkdownLinkHandler(handler: MarkdownLinkHandler): void {
  linkHandler = handler;
}

function dirOf(path: string): string {
  return path.slice(0, Math.max(path.lastIndexOf("/"), path.lastIndexOf("\\")));
}

/** `relative` against `dir`, with `.`/`..` folded — `/`-separated. */
function resolvePath(dir: string, relative: string): string {
  const parts = (relative.startsWith("/") ? relative : `${dir}/${relative}`).split(/[/\\]+/);
  const out: string[] = [];
  for (const part of parts) {
    if (part === "..") out.pop();
    else if (part !== ".") out.push(part);
  }
  return out.join("/");
}

const EXTERNAL = /^[a-z][a-z0-9+.-]*:/i;

let mermaidIds = 0;

export interface MarkdownPreviewProps extends EditorProps {
  /** Double-clicking a block — its 1-based source line. */
  onSourceLine?: (line: number) => void;
  /** 0-based source line to keep scrolled to the top (scroll sync from a text view). */
  scrollLine?: number;
}

/**
 * The rendered Markdown, styled like VS Code's preview and following the
 * editor's color profile: GFM, KaTeX math, footnotes, GitHub alerts,
 * highlighted code and Mermaid diagrams. Task checkboxes toggle the source;
 * links to other files go through `setMarkdownLinkHandler`; relative images
 * load from disk.
 */
export function MarkdownPreview({ path, content, onChange, readOnly, onSourceLine, scrollLine }: MarkdownPreviewProps) {
  const { editorColors } = useTheme();
  const dark = editorColors.background.l < 50;
  const rootRef = useRef<HTMLDivElement>(null);
  const bodyRef = useRef<HTMLDivElement>(null);

  // Re-rendering on every keystroke is wasteful (Mermaid most of all) — wait for a pause.
  const [source, setSource] = useState(content);
  useEffect(() => {
    const timer = setTimeout(() => setSource(content), 150);
    return () => clearTimeout(timer);
  }, [content]);
  const html = useMemo(() => renderMarkdown(source), [source]);

  useEffect(() => {
    const body = bodyRef.current;
    if (!body) return;
    body.innerHTML = html;
    const dir = dirOf(path);
    body.querySelectorAll("img").forEach((img) => {
      const src = img.getAttribute("src");
      if (src && !EXTERNAL.test(src) && !src.startsWith("//")) img.src = convertFileSrc(resolvePath(dir, decodeURI(src)));
    });
    const diagrams = [...body.querySelectorAll<HTMLElement>(".sp-md-mermaid")];
    if (diagrams.length === 0) return;
    let cancelled = false;
    (async () => {
      const { default: mermaid } = await import("mermaid");
      mermaid.initialize({ startOnLoad: false, theme: dark ? "dark" : "default", securityLevel: "strict" });
      for (const el of diagrams) {
        if (cancelled) return;
        try {
          const { svg } = await mermaid.render(`sp-mermaid-${++mermaidIds}`, el.dataset.source ?? "");
          if (!cancelled) el.innerHTML = svg;
        } catch (err) {
          el.classList.add("sp-md-mermaid--error");
          el.textContent = String(err instanceof Error ? err.message : err);
        }
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [html, path, dark]);

  // Scroll sync: the block for `scrollLine` (interpolated between the blocks around it) at the top.
  useEffect(() => {
    const root = rootRef.current;
    const body = bodyRef.current;
    if (scrollLine === undefined || !root || !body) return;
    const blocks = [...body.querySelectorAll<HTMLElement>("[data-line]")];
    let before: HTMLElement | null = null;
    let after: HTMLElement | null = null;
    for (const el of blocks) {
      const line = Number(el.dataset.line);
      if (line <= scrollLine) before = el;
      else {
        after = el;
        break;
      }
    }
    if (!before) {
      root.scrollTop = 0;
      return;
    }
    const top = (el: HTMLElement) => el.getBoundingClientRect().top - body.getBoundingClientRect().top;
    const fromLine = Number(before.dataset.line);
    let y = top(before);
    if (after) {
      const fraction = (scrollLine - fromLine) / (Number(after.dataset.line) - fromLine);
      y += (top(after) - y) * fraction;
    }
    root.scrollTop = y;
  }, [scrollLine, html]);

  function handleClick(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (target instanceof HTMLInputElement && target.classList.contains("sp-md-task")) {
      e.preventDefault();
      if (!readOnly) onChange(toggleTask(content, Number(target.dataset.line)));
      return;
    }
    const link = target.closest("a");
    const href = link?.getAttribute("href");
    if (!href) return;
    e.preventDefault();
    if (href.startsWith("#")) {
      document.getElementById(decodeURIComponent(href.slice(1)))?.scrollIntoView();
    } else if (EXTERNAL.test(href)) {
      linkHandler(href, "external");
    } else {
      linkHandler(resolvePath(dirOf(path), decodeURI(href.split("#")[0])), "file");
    }
  }

  function handleDoubleClick(e: MouseEvent) {
    const block = (e.target as HTMLElement).closest<HTMLElement>("[data-line]");
    if (block && onSourceLine) onSourceLine(Number(block.dataset.line) + 1);
  }

  const style = {
    "--sp-md-keyword": hslToCss(editorColors.keyword),
    "--sp-md-string": hslToCss(editorColors.string),
    "--sp-md-comment": hslToCss(editorColors.comment),
    "--sp-md-number": hslToCss(editorColors.number),
    "--sp-md-type": hslToCss(editorColors.type),
    "--sp-md-function": hslToCss(editorColors.function),
    "--sp-md-variable": hslToCss(editorColors.variable),
    "--sp-md-code-bg": hslToCss(editorColors.background),
    "--sp-md-code-fg": hslToCss(editorColors.foreground),
  } as CSSProperties;

  return (
    <div ref={rootRef} className="sp-md-preview" data-dark={dark} style={style} onClick={handleClick} onDoubleClick={handleDoubleClick}>
      <div ref={bodyRef} className="sp-md-body" />
    </div>
  );
}

/** Flips the `[ ]`/`[x]` of the task item starting on 0-based `line`. */
function toggleTask(content: string, line: number): string {
  const lines = content.split("\n");
  const text = lines[line];
  if (text === undefined) return content;
  lines[line] = text.replace(/\[([ xX])\]/, (_, mark: string) => (mark === " " ? "[x]" : "[ ]"));
  return lines.join("\n");
}

/**
 * Source and preview side by side, the preview following the editor's
 * scroll — VS Code's "Open Preview to the Side". Double-clicking the preview
 * jumps the editor to that line.
 */
export function MarkdownSplitEditor(props: EditorProps) {
  const viewRef = useRef<EditorView | null>(null);
  const [topLine, setTopLine] = useState(0);

  function handleViewReady(view: EditorView) {
    viewRef.current = view;
    view.scrollDOM.addEventListener("scroll", () => {
      const block = view.lineBlockAtHeight(view.scrollDOM.scrollTop);
      const line = view.state.doc.lineAt(block.from).number - 1;
      // Fractional, so the preview moves smoothly within a long paragraph.
      setTopLine(line + (view.scrollDOM.scrollTop - block.top) / Math.max(block.height, 1));
    });
  }

  function goToLine(line: number) {
    const view = viewRef.current;
    if (view) revealLine(view, line);
  }

  return (
    <div className="sp-md-split">
      <div className="sp-md-split-pane">
        <MarkdownEditor {...props} onViewReady={handleViewReady} />
      </div>
      <div className="sp-md-split-pane">
        <MarkdownPreview {...props} scrollLine={topLine} onSourceLine={goToLine} />
      </div>
    </div>
  );
}
