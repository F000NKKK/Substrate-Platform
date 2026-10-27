import { autocompletion, type CompletionContext, type CompletionResult } from "@codemirror/autocomplete";
import type { Extension } from "@codemirror/state";
import { EditorView, hoverTooltip, keymap, ViewPlugin, type ViewUpdate } from "@codemirror/view";
import { openFile } from "../documents/fileOpenBus";
import {
  markupText,
  toCompletions,
  toOffset,
  toOpenRequest,
  toPosition,
  type LspCompletionResult,
  type LspHover,
  type LspLocation,
  type LspLocationLink,
} from "./convert";
import type { LspSession } from "./lspSession";
import { uriFromPath } from "./uri";
import "./lsp.css";

/** How long typing pauses before the server hears the new text. */
export const CHANGE_DEBOUNCE_MS = 300;

/**
 * Keeps the server's copy of one document: open with the editor, the whole
 * text again after each pause in typing (and before every request, so the
 * server answers about what's on screen), closed with the editor.
 */
class DocumentSync {
  private version = 1;
  private timer: ReturnType<typeof setTimeout> | undefined;
  /** Every notification in order — a change never overtakes the open. */
  private queue: Promise<void>;

  constructor(
    private readonly session: LspSession,
    private readonly path: string,
    private readonly view: EditorView
  ) {
    this.queue = session.didOpen(path, this.version, view.state.doc.toString()).catch(() => {});
  }

  update(update: ViewUpdate) {
    if (!update.docChanged) return;
    clearTimeout(this.timer);
    this.timer = setTimeout(() => void this.flush(), CHANGE_DEBOUNCE_MS);
  }

  /** Sends a pending change now; resolves once the server has everything. */
  flush(): Promise<void> {
    if (this.timer !== undefined) {
      clearTimeout(this.timer);
      this.timer = undefined;
      const version = ++this.version;
      const text = this.view.state.doc.toString();
      this.queue = this.queue.then(() => this.session.didChange(this.path, version, text)).catch(() => {});
    }
    return this.queue;
  }

  destroy() {
    void this.flush().then(() => this.session.didClose(this.path).catch(() => {}));
  }
}

/** What a request about `pos` names: the document and the position. */
function at(view: EditorView, path: string, pos: number) {
  return { textDocument: { uri: uriFromPath(path) }, position: toPosition(view.state.doc, pos) };
}

/**
 * Language server features for the editor of `path`: the document kept in
 * sync, completion, hover, and Go To Definition (F12, Ctrl/Cmd+click) through
 * the file open bus. Diagnostics reach the editor through the Error List's
 * store, like a build's.
 */
export function lspExtension(session: LspSession, path: string): Extension {
  const sync = ViewPlugin.define((view) => new DocumentSync(session, path, view));
  const ready = (view: EditorView) => view.plugin(sync)?.flush() ?? Promise.resolve();

  async function complete(context: CompletionContext): Promise<CompletionResult | null> {
    const view = context.view;
    if (!view) return null;
    const word = context.matchBefore(/[\w$]*/);
    if (!context.explicit && (!word || word.from === word.to)) return null;
    await ready(view);
    try {
      const result = await session.request<LspCompletionResult>("textDocument/completion", at(view, path, context.pos));
      if (context.aborted) return null;
      const { from, options } = toCompletions(view.state.doc, result, word?.from ?? context.pos);
      return options.length ? { from, options, validFor: /^[\w$]*$/ } : null;
    } catch {
      return null;
    }
  }

  const hover = hoverTooltip(async (view, pos) => {
    await ready(view);
    let result: LspHover | null;
    try {
      result = await session.request<LspHover | null>("textDocument/hover", at(view, path, pos));
    } catch {
      return null;
    }
    const text = result ? markupText(result.contents).trim() : "";
    if (!text) return null;
    const range = result?.range;
    return {
      pos: range ? toOffset(view.state.doc, range.start) : pos,
      end: range ? toOffset(view.state.doc, range.end) : pos,
      above: true,
      create: () => {
        const dom = document.createElement("div");
        dom.className = "sp-lsp-hover";
        dom.textContent = text;
        return { dom };
      },
    };
  });

  async function goToDefinition(view: EditorView, pos: number): Promise<void> {
    await ready(view);
    try {
      const result = await session.request<LspLocation | LspLocation[] | LspLocationLink[] | null>(
        "textDocument/definition",
        at(view, path, pos)
      );
      const request = toOpenRequest(result);
      if (request) openFile(request);
    } catch {
      // Nothing to go to.
    }
  }

  return [
    sync,
    autocompletion({ override: [complete] }),
    hover,
    keymap.of([
      {
        key: "F12",
        run: (view) => {
          void goToDefinition(view, view.state.selection.main.head);
          return true;
        },
      },
    ]),
    EditorView.domEventHandlers({
      mousedown: (event, view) => {
        if (!(event.ctrlKey || event.metaKey) || event.button !== 0) return false;
        const pos = view.posAtCoords({ x: event.clientX, y: event.clientY });
        if (pos === null) return false;
        event.preventDefault();
        void goToDefinition(view, pos);
        return true;
      },
    }),
  ];
}
