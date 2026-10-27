import { type RefObject, useEffect, useRef, useState } from "react";
import { useIdeHost } from "../infra/host/ideHost";
import { t, useT } from "../infra/i18n/i18n";
import { useShell } from "../shell/ShellContext";
import type { SearchOptions } from "../widgets/SearchField/SearchField";
import { useGlobalState } from "../workspace/WorkspaceProvider";
import { DEFAULTS, OPTIONS_KEY, REPLACEMENT_KEY, type Remembered, bufferText, isRemembered, isString, lookInLabel, seedText } from "./findOptions";
import { LOOK_IN, type FindQuery, type LookIn, type OpenDocument } from "./findStore";
import { FIND_MODE, findDrafts, useFindRequest, type FindWindowMode } from "./findUi";

/** The Find and Replace window's state: what it looks for, where, and with which options. */
export function useFindForm() {
  useT();
  const host = useIdeHost();
  const shell = useShell();
  const { root, documents: tabs } = host;
  const activeTab = tabs.find((tab) => tab.id === shell.activeCenterId);
  const [mode, setModeState] = useState(findDrafts.mode);
  const [remembered, setRemembered] = useGlobalState<Remembered>(OPTIONS_KEY, DEFAULTS, isRemembered);
  const [pattern, setPatternState] = useState(findDrafts.pattern);
  const [replacement, setReplacement] = useGlobalState<string>(REPLACEMENT_KEY, "", isString);
  const [busy, setBusy] = useState(false);
  const findRef = useRef<HTMLInputElement>(null);

  const textTabs = tabs.filter((tab) => tab.textLoaded);
  const hasCurrent = !!activeTab?.textLoaded;
  const available: Record<LookIn, boolean> = { current: hasCurrent, open: textTabs.length > 0, project: !!root };
  // A remembered choice that isn't possible right now falls back to the widest one that is.
  const lookIn: LookIn = available[remembered.lookIn] ? remembered.lookIn : root ? LOOK_IN.project : hasCurrent ? LOOK_IN.current : LOOK_IN.open;
  const options: SearchOptions = { matchCase: remembered.matchCase, wholeWord: remembered.wholeWord, regex: remembered.regex };
  const canRun = pattern !== "" && available[lookIn] && !busy;

  function setMode(next: FindWindowMode) {
    findDrafts.mode = next;
    setModeState(next);
  }

  function setPattern(next: string) {
    findDrafts.pattern = next;
    setPatternState(next);
  }

  useSeedOnRequest(setMode, setPattern, activeTab?.path, findRef);

  function query(): FindQuery {
    return { pattern, ...options, fileMask: remembered.fileMask, includeGenerated: mode === FIND_MODE.find && !!remembered.includeGenerated };
  }

  function documents(): OpenDocument[] {
    const scoped = lookIn === LOOK_IN.current ? textTabs.filter((tab) => tab.id === activeTab?.id) : textTabs;
    return scoped.map((tab) => ({ path: tab.path, text: bufferText(tab) }));
  }

  function caption(verb: string): string {
    const parts = [`${verb} "${pattern}"`];
    if (options.matchCase) parts.push(t("Match case"));
    if (options.wholeWord) parts.push(t("Whole word"));
    if (options.regex) parts.push(t("Regular expressions"));
    parts.push(lookInLabel(lookIn));
    if (remembered.fileMask.trim()) parts.push(`"${remembered.fileMask.trim()}"`);
    return parts.join(", ");
  }

  return { host, shell, root, mode, setMode, pattern, setPattern, replacement, setReplacement, remembered, setRemembered, setBusy, findRef, available, lookIn, options, canRun, query, documents, caption };
}

/** What `useFindForm` gives the window. */
export type FindForm = ReturnType<typeof useFindForm>;

/**
 * Ctrl+Shift+F/H: switch tab, re-seed from the editor, focus. Only on a new
 * request — the window remounts whenever it's moved to another dock.
 */
function useSeedOnRequest(setMode: (mode: FindWindowMode) => void, setPattern: (pattern: string) => void, activePath: string | undefined, findRef: RefObject<HTMLInputElement | null>) {
  const request = useFindRequest();
  useEffect(() => {
    if (!request || request.seq === findDrafts.seq) return;
    findDrafts.seq = request.seq;
    setMode(request.mode);
    const seed = seedText(activePath);
    if (seed) setPattern(seed);
    requestAnimationFrame(() => {
      findRef.current?.focus();
      findRef.current?.select();
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [request?.seq]);
}
