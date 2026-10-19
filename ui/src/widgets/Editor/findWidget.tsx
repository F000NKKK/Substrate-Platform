import { useEffect, useRef, useState, type KeyboardEvent as ReactKeyboardEvent, type ReactNode } from "react";
import { createRoot, type Root } from "react-dom/client";
import {
  closeSearchPanel,
  findNext,
  findPrevious,
  getSearchQuery,
  openSearchPanel,
  replaceAll,
  replaceNext,
  search,
  searchPanelOpen,
  SearchQuery,
  setSearchQuery,
} from "@codemirror/search";
import { EditorSelection, type EditorState, type Extension } from "@codemirror/state";
import { EditorView, type Panel, type ViewUpdate } from "@codemirror/view";
import { t, useT, withShortcut } from "../../infra/i18n";
import { IconChevronRight, IconClose } from "../../infra/icons";
import { showDialog } from "../Dialog";
import "./findWidget.css";
import { Icon } from "../../infra/icons";

// Visual Studio's Quick Find / Quick Replace: a compact box floating over
// the top-right corner of the editor rather than a bar pushing the text
// down — the chevron on the left unfolds the replace row, the options sit
// inside the search field (Aa, ab, .*), and "3 of 12" tells where you are.
// Typing searches as you go from the cursor; Enter / Shift+Enter (and
// F3 / Shift+F3 in the editor) step through matches, Esc closes it.

const MAX_COUNTED = 9999;

interface MatchInfo {
  /** Every match, up to `MAX_COUNTED`. */
  total: number;
  /** 1-based index of the match the selection is on, or 0. */
  current: number;
  /** The query's regexp doesn't compile. */
  invalid: boolean;
}

function matchInfo(state: EditorState, query: SearchQuery): MatchInfo {
  if (!query.search) return { total: 0, current: 0, invalid: false };
  if (!query.valid) return { total: 0, current: 0, invalid: true };
  const sel = state.selection.main;
  let total = 0;
  let current = 0;
  const cursor = query.getCursor(state);
  for (let next = cursor.next(); !next.done; next = cursor.next()) {
    total++;
    if (next.value.from === sel.from && next.value.to === sel.to) current = total;
    if (total >= MAX_COUNTED) break;
  }
  return { total, current, invalid: false };
}

/** Selects the first match at or after `from` (wrapping), scrolled into view — how typing in the box moves through the document. */
function selectMatchFrom(view: EditorView, query: SearchQuery, from: number) {
  if (!query.search || !query.valid) return;
  const cursor = query.getCursor(view.state, from);
  let found = cursor.next();
  if (found.done) found = query.getCursor(view.state, 0).next();
  if (found.done) return;
  const { from: start, to } = found.value;
  view.dispatch({
    selection: EditorSelection.single(start, to),
    effects: EditorView.scrollIntoView(start, { y: "center" }),
    userEvent: "select.search",
  });
}

/** The single-line selection (or the word at the cursor) — what Ctrl+F starts searching for. */
function seedText(state: EditorState): string {
  const sel = state.selection.main;
  if (!sel.empty) {
    const text = state.sliceDoc(sel.from, sel.to);
    return text.includes("\n") ? "" : text;
  }
  const word = state.wordAt(sel.head);
  return word ? state.sliceDoc(word.from, word.to) : "";
}

interface Controller {
  focus(replace: boolean, seed: string): void;
}

const controllers = new WeakMap<EditorView, Controller>();

function Toggle({ on, title, onClick, children }: { on: boolean; title: string; onClick: () => void; children: ReactNode }) {
  return (
    <button
      type="button"
      className="sp-find-toggle"
      data-on={on || undefined}
      title={title}
      aria-pressed={on}
      onMouseDown={(e) => e.preventDefault()}
      onClick={onClick}
    >
      {children}
    </button>
  );
}

function Action({ title, disabled, onClick, children }: { title: string; disabled?: boolean; onClick: () => void; children: ReactNode }) {
  return (
    <button type="button" className="sp-find-action" title={title} disabled={disabled} onMouseDown={(e) => e.preventDefault()} onClick={onClick}>
      {children}
    </button>
  );
}

function FindWidget({ view, info, register }: { view: EditorView; info: MatchInfo; register: (c: Controller) => void }) {
  const t = useT();
  const query = getSearchQuery(view.state);
  const [expanded, setExpanded] = useState(false);
  const findRef = useRef<HTMLInputElement>(null);
  const replaceRef = useRef<HTMLInputElement>(null);
  // Where typing started — each keystroke re-searches from here, so
  // refining the text doesn't skip past a closer match.
  const origin = useRef(view.state.selection.main.from);

  useEffect(() => {
    register({
      focus(replace, seed) {
        if (replace) setExpanded(true);
        origin.current = view.state.selection.main.from;
        if (seed && seed !== getSearchQuery(view.state).search) setQuery({ search: seed }, false);
        requestAnimationFrame(() => {
          findRef.current?.focus();
          findRef.current?.select();
        });
      },
    });
  });

  function setQuery(change: Partial<ConstructorParameters<typeof SearchQuery>[0]>, jump = true) {
    const current = getSearchQuery(view.state);
    const next = new SearchQuery({
      search: current.search,
      replace: current.replace,
      caseSensitive: current.caseSensitive,
      regexp: current.regexp,
      wholeWord: current.wholeWord,
      ...change,
    });
    view.dispatch({ effects: setSearchQuery.of(next) });
    if (jump && "search" in change) selectMatchFrom(view, next, origin.current);
    else if (jump) selectMatchFrom(view, next, view.state.selection.main.from);
  }

  const next = () => findNext(view);
  const previous = () => findPrevious(view);
  const close = () => {
    closeSearchPanel(view);
    view.focus();
  };

  function onKeyDown(e: ReactKeyboardEvent, field: "find" | "replace") {
    const alt = e.altKey && !e.ctrlKey && !e.metaKey;
    const code = e.nativeEvent.code;
    if (e.key === "Escape") close();
    else if (e.key === "Enter" && field === "find") (e.shiftKey ? previous : next)();
    else if (e.key === "Enter" && field === "replace") (e.ctrlKey || e.metaKey ? replaceAll : replaceNext)(view);
    else if (e.key === "F3") (e.shiftKey ? previous : next)();
    // Visual Studio's option keys, by physical key so any layout works.
    else if (alt && code === "KeyC") setQuery({ caseSensitive: !query.caseSensitive });
    else if (alt && code === "KeyW") setQuery({ wholeWord: !query.wholeWord });
    else if (alt && code === "KeyE") setQuery({ regexp: !query.regexp });
    else if (alt && code === "KeyR" && expanded) replaceNext(view);
    else if (alt && code === "KeyA" && expanded) replaceAll(view);
    else if ((e.ctrlKey || e.metaKey) && !e.altKey && code === "KeyH") setExpanded(true);
    else if ((e.ctrlKey || e.metaKey) && !e.altKey && code === "KeyF") findRef.current?.select();
    else return;
    e.preventDefault();
    e.stopPropagation();
  }

  const status = !query.search
    ? ""
    : info.invalid
      ? t("Invalid pattern")
      : info.total === 0
        ? t("No results")
        : t("{current} of {total}", { current: info.current || "?", total: info.total >= MAX_COUNTED ? `${MAX_COUNTED}+` : info.total });
  const readOnly = view.state.readOnly;

  return (
    <div className="sp-find" data-expanded={expanded || undefined}>
      <button
        type="button"
        className="sp-find-expand"
        title={expanded ? t("Hide replace") : t("Show replace")}
        aria-expanded={expanded}
        disabled={readOnly}
        onMouseDown={(e) => e.preventDefault()}
        onClick={() => setExpanded((v) => !v)}
      >
        <IconChevronRight size={14} />
      </button>
      <div className="sp-find-rows">
        <div className="sp-find-row">
          <div className="sp-find-field" data-invalid={info.invalid || undefined} data-empty={(!!query.search && info.total === 0) || undefined}>
            <input
              data-own-undo
              ref={findRef}
              className="sp-find-input"
              placeholder={t("Find...")}
              aria-label={t("Find")}
              spellCheck={false}
              value={query.search}
              onChange={(e) => setQuery({ search: e.target.value })}
              onKeyDown={(e) => onKeyDown(e, "find")}
              onFocus={() => (origin.current = view.state.selection.main.from)}
            />
            <Toggle on={query.caseSensitive} title={withShortcut("Match case", "alt+c")} onClick={() => setQuery({ caseSensitive: !query.caseSensitive })}>
              Aa
            </Toggle>
            <Toggle on={query.wholeWord} title={withShortcut("Match whole word", "alt+w")} onClick={() => setQuery({ wholeWord: !query.wholeWord })}>
              <span className="sp-find-word">ab</span>
            </Toggle>
            <Toggle on={query.regexp} title={withShortcut("Use regular expressions", "alt+e")} onClick={() => setQuery({ regexp: !query.regexp })}>
              .*
            </Toggle>
          </div>
          <span className="sp-find-status" data-empty={(!!query.search && info.total === 0) || undefined}>
            {status}
          </span>
          <Action title={withShortcut("Find previous", "shift+f3")} disabled={!info.total} onClick={previous}>
            <Icon name="arrowUp" />
          </Action>
          <Action title={withShortcut("Find next", "f3")} disabled={!info.total} onClick={next}>
            <Icon name="arrowDown" />
          </Action>
          <Action title={withShortcut("Close", "esc")} onClick={close}>
            <IconClose />
          </Action>
        </div>
        {expanded && !readOnly && (
          <div className="sp-find-row">
            <div className="sp-find-field">
              <input
                data-own-undo
                ref={replaceRef}
                className="sp-find-input"
                placeholder={t("Replace...")}
                aria-label={t("Replace")}
                spellCheck={false}
                value={query.replace}
                onChange={(e) => setQuery({ replace: e.target.value }, false)}
                onKeyDown={(e) => onKeyDown(e, "replace")}
              />
            </div>
            <Action title={withShortcut("Replace next", "alt+r")} disabled={!info.total} onClick={() => replaceNext(view)}>
              <Icon name="replace" />
            </Action>
            <Action title={withShortcut("Replace all", "alt+a")} disabled={!info.total} onClick={() => replaceAll(view)}>
              <Icon name="replaceAll" />
            </Action>
          </div>
        )}
      </div>
    </div>
  );
}

function createFindPanel(view: EditorView): Panel {
  const dom = document.createElement("div");
  dom.className = "sp-find-panel";
  let root: Root | null = null;
  let info = matchInfo(view.state, getSearchQuery(view.state));
  let pending: { replace: boolean; seed: string } | null = null;
  const register = (c: Controller) => {
    controllers.set(view, c);
    if (pending) {
      c.focus(pending.replace, pending.seed);
      pending = null;
    }
  };
  const render = () => root?.render(<FindWidget view={view} info={info} register={register} />);
  controllers.set(view, {
    focus(replace, seed) {
      pending = { replace, seed };
    },
  });
  return {
    dom,
    top: true,
    mount() {
      root = createRoot(dom);
      render();
    },
    update(update: ViewUpdate) {
      const query = getSearchQuery(update.state);
      const changed = update.docChanged || update.selectionSet || !query.eq(getSearchQuery(update.startState));
      if (changed) info = matchInfo(update.state, query);
      if (changed || update.transactions.length) render();
    },
    destroy() {
      controllers.delete(view);
      // Unmounting synchronously inside CodeMirror's update cycle can race
      // React's own rendering — let it settle first.
      const r = root;
      root = null;
      setTimeout(() => r?.unmount());
    },
  };
}

/** Opens Quick Find (or Quick Replace), seeded with the selection or the word at the cursor, and focuses it. */
export function openQuickFind(view: EditorView, replace = false): boolean {
  const seed = seedText(view.state);
  if (!searchPanelOpen(view.state)) openSearchPanel(view);
  controllers.get(view)?.focus(replace && !view.state.readOnly, seed);
  return true;
}

/** Visual Studio's Go To Line dialog: "Line number (1 - N)", the cursor's line preselected. */
export async function goToLineDialog(view: EditorView): Promise<boolean> {
  const { doc } = view.state;
  const currentLine = doc.lineAt(view.state.selection.main.head).number;
  const label = t("Line number (1 - {lines}):", { lines: doc.lines });
  const result = await showDialog({
    title: t("Go To Line"),
    input: {
      label,
      defaultValue: String(currentLine),
      validate: (value) => {
        const n = Number(value.trim());
        return Number.isInteger(n) && n >= 1 && n <= doc.lines ? null : "";
      },
    },
    buttons: [
      { id: "ok", label: t("OK"), variant: "primary" },
      { id: "cancel", label: t("Cancel"), cancel: true },
    ],
  });
  if (result.button === "ok") {
    const line = view.state.doc.line(Math.min(Number(result.value.trim()), view.state.doc.lines));
    view.dispatch({
      selection: EditorSelection.cursor(line.from),
      effects: EditorView.scrollIntoView(line.from, { y: "center" }),
      userEvent: "select",
    });
  }
  view.focus();
  return true;
}

/** The search extension with the Quick Find box as its panel. */
export const quickFind: Extension = search({ top: true, createPanel: createFindPanel });
