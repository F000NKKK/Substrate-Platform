import { useMemo, useRef, useState } from "react";
import { useStoredState } from "../../../infra/state";
import type { PropertyGridProps } from "./propertyTypes";
import { DEFAULT_DESCRIPTION, DEFAULT_PERSIST_KEY, DEFAULT_SPLIT, layoutItems, MAX_SPLIT, MIN_SPLIT, VIEW_MODE, type ViewMode } from "./propertyValues";

const isViewMode = (v: unknown) => Object.values<unknown>(VIEW_MODE).includes(v);
const isSplit = (v: unknown) => typeof v === "number" && v >= MIN_SPLIT && v <= MAX_SPLIT;
const isNumber = (v: unknown) => typeof v === "number";
const isBool = (v: unknown) => typeof v === "boolean";

/** A text box being edited: which property, its text, and its session (see `sessionRef`). */
export interface EditSession {
  id: string;
  draft: string;
  session: number;
}

/** The grid's state: what's remembered (view, split, collapsed, description), the search, the cursor and the edit in progress. */
export function usePropertyGridState({ objectName, objectKey, properties, persistKey = DEFAULT_PERSIST_KEY, order }: PropertyGridProps) {
  const [mode, setMode] = useStoredState<ViewMode>(`${persistKey}.view`, VIEW_MODE.categorized, isViewMode);
  const [split, setSplit] = useStoredState<number>(`${persistKey}.split`, DEFAULT_SPLIT, isSplit);
  const [collapsedList, setCollapsedList] = useStoredState<string[]>(`${persistKey}.collapsed`, [], Array.isArray);
  const [descriptionVisible, setDescriptionVisible] = useStoredState<boolean>(`${persistKey}.description`, true, isBool);
  const [descriptionHeight, setDescriptionHeight] = useStoredState<number>(`${persistKey}.descriptionHeight`, DEFAULT_DESCRIPTION, isNumber);
  const [query, setQuery] = useState("");
  const [cursor, setCursor] = useState<string | null>(null);
  const [editing, setEditing] = useState<EditSession | null>(null);
  const [error, setError] = useState<{ id: string; message: string } | null>(null);
  const [busyId, setBusyId] = useState<string | null>(null);
  const rowsRef = useRef<HTMLDivElement>(null);
  // Which text-box session is live — a blur arriving after Enter/Esc
  // already ended it (or after the object changed) must not commit again.
  const sessionRef = useRef(0);

  // A different object: drop the old one's edit and error (adjusting state
  // during render, as `useStoredState` does, so the new object never
  // renders a frame with the old one's text box).
  const shownKey = objectKey ?? objectName ?? "";
  const [lastKey, setLastKey] = useState(shownKey);
  if (lastKey !== shownKey) {
    setLastKey(shownKey);
    setEditing(null);
    setError(null);
    sessionRef.current++;
  }

  const collapsed = useMemo(() => new Set(collapsedList), [collapsedList]);
  const searching = query.trim() !== "";
  const items = useMemo(() => layoutItems(properties, mode, query, collapsed, order), [properties, mode, query, collapsed, order]);
  const current = items.find((item) => item.key === cursor) ?? null;

  function toggleCategory(name: string) {
    setCollapsedList((prev) => (prev.includes(name) ? prev.filter((c) => c !== name) : [...prev, name]));
  }

  function focusRows() {
    rowsRef.current?.focus();
  }

  return { mode, setMode, split, setSplit, descriptionVisible, setDescriptionVisible, descriptionHeight, setDescriptionHeight, query, setQuery, cursor, setCursor, editing, setEditing, error, setError, busyId, setBusyId, rowsRef, sessionRef, searching, items, current, toggleCategory, focusRows };
}

export type PropertyGridState = ReturnType<typeof usePropertyGridState>;
