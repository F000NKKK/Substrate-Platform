import { useEffect, type RefObject } from "react";
import { Terminal as XTerm } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { errorText } from "../infra/i18n/errorText";
import { eventKey } from "../infra/keys";
import { copySelection, pasteInto } from "./clipboard";
import { terminalApi } from "./terminalApi";
import "@xterm/xterm/css/xterm.css";

/** The terminal's font size, in pixels (its family is the theme's monospace). */
const FONT_SIZE = 13;

/**
 * How long the shell's output has to stay quiet before the first clear — a
 * clear sent right after start races its startup (profiles, banners: pwsh's
 * especially) and leaves the banner on screen.
 */
const CLEAR_QUIET_MS = 200;

/** Copy and paste: one modifier over Ctrl+C/V, which belong to the shell (SIGINT, its own paste). */
const COPY_KEY = "c";
const PASTE_KEY = "v";

/**
 * The terminals already cleared once after start. Ids are unique per
 * session, but React's StrictMode mounts an effect twice for the same id —
 * without this, both mounts' delayed clears could fire on the live session.
 */
const autoCleared = new Set<string>();

/** Ctrl+Shift+C/V copy and paste in `term` (terminal `id`). */
function attachClipboardKeys(term: XTerm, id: string) {
  term.attachCustomKeyEventHandler((e) => {
    if (e.type !== "keydown" || !e.ctrlKey || !e.shiftKey) return true;
    const key = eventKey(e);
    if (key === COPY_KEY) copySelection(term.getSelection());
    else if (key === PASTE_KEY) pasteInto(id);
    else return true;
    return false;
  });
}

/**
 * Terminal `id`'s first clear, once its output has been quiet for
 * `CLEAR_QUIET_MS` — `schedule` on every output, `cancel` for good (the user
 * typed), `dispose` on unmount.
 */
function quietClear(id: string, clear: () => void) {
  let done = false;
  let timer: number | undefined;
  return {
    schedule() {
      if (done || autoCleared.has(id)) return;
      window.clearTimeout(timer);
      timer = window.setTimeout(() => {
        done = true;
        // Again at fire time: the other StrictMode mount may have won meanwhile.
        if (autoCleared.has(id)) return;
        autoCleared.add(id);
        clear();
      }, CLEAR_QUIET_MS);
    },
    cancel() {
      done = true;
      autoCleared.add(id);
      window.clearTimeout(timer);
    },
    dispose() {
      window.clearTimeout(timer);
    },
  };
}

/** Where a terminal's session puts what the component needs back. */
export interface SessionRefs {
  container: RefObject<HTMLDivElement | null>;
  term: { current: XTerm | null };
  fit: { current: (() => void) | null };
  onExit: { current: () => void };
  onReady: { current: (clear: (() => void) | null) => void };
}

/**
 * Terminal `id`'s shell session in xterm, in `refs.container`: started on
 * mount, killed on unmount — it lives as long as the component.
 */
export function useTerminalSession(id: string, shell: string | undefined, cwd: string | undefined, clearCommand: string, refs: SessionRefs) {
  useEffect(() => {
    if (!refs.container.current) return;
    const term = new XTerm({ fontFamily: "var(--sp-font-mono)", fontSize: FONT_SIZE, theme: { background: "transparent" }, cursorBlink: true });
    refs.term.current = term;
    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(refs.container.current);

    attachClipboardKeys(term, id);
    const fit = () => {
      try {
        fitAddon.fit();
      } catch {
        // Not laid out yet (hidden): measured again once shown.
      }
    };
    refs.fit.current = fit;
    fit();

    // "Clear" makes the shell run its own clear command, never clears
    // xterm's buffer: readline/PSReadLine track the cursor themselves and
    // only resync when they ran the clear — a display-only clear leaves
    // everything typed afterwards drawn out of place.
    const runClear = () => void terminalApi.write(id, clearCommand).catch(() => {});
    refs.onReady.current(runClear);

    const firstClear = quietClear(id, runClear);
    const scheduleClear = firstClear.schedule;

    let disposed = false;
    const unlistenOutput = terminalApi.onOutput((from, data) => {
      if (disposed || from !== id) return;
      term.write(data);
      scheduleClear();
    });
    const unlistenExit = terminalApi.onExit((from) => {
      if (!disposed && from === id) refs.onExit.current();
    });

    terminalApi
      .spawn({ id, cols: term.cols, rows: term.rows, shell, cwd })
      .then(scheduleClear)
      .catch((err) => term.writeln(`\r\n${errorText(err)}`));

    const onData = term.onData((data) => {
      // Typing already: a clear now would erase what the user sees — never.
      firstClear.cancel();
      terminalApi.write(id, data).catch(() => {});
    });

    const resizeObserver = new ResizeObserver(() => {
      fit();
      terminalApi.resize(id, term.cols, term.rows).catch(() => {});
    });
    resizeObserver.observe(refs.container.current);

    return () => {
      disposed = true;
      refs.fit.current = null;
      refs.term.current = null;
      refs.onReady.current(null);
      firstClear.dispose();
      resizeObserver.disconnect();
      onData.dispose();
      void unlistenOutput.then((un) => un());
      void unlistenExit.then((un) => un());
      terminalApi.kill(id).catch(() => {});
      term.dispose();
    };
  }, [id]);
}
