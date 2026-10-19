import type { KeyboardEvent, PointerEvent as ReactPointerEvent } from "react";
import { trackPointer } from "../../infra/hooks/trackPointer";
import { MAX_DESCRIPTION, MAX_SPLIT, MIN_DESCRIPTION, MIN_SPLIT } from "./propertyValues";
import type { PropertyEditing } from "./propertyEditing";
import type { PropertyGridState } from "./usePropertyGridState";

const clamp = (n: number, min: number, max: number) => Math.max(min, Math.min(max, n));

/** The grid's keyboard (rows, categories, Enter/F2) and its drags (splitter, description pane). */
export function propertyGridInput(grid: PropertyGridState, editing: PropertyEditing) {
  const { items, current, searching, rowsRef, setCursor, toggleCategory, setSplit, descriptionHeight, setDescriptionHeight } = grid;

  function moveTo(index: number) {
    const target = items[clamp(index, 0, items.length - 1)];
    if (!target) return;
    setCursor(target.key);
    rowsRef.current?.querySelector(`[data-key="${CSS.escape(target.key)}"]`)?.scrollIntoView({ block: "nearest" });
  }

  /** Arrow keys, Home/End, Left/Right to fold, Enter/F2 to edit — `false` when the key isn't the grid's. */
  function onKey(key: string): boolean {
    const index = current ? items.indexOf(current) : -1;
    if (key === "ArrowDown") moveTo(index + 1);
    else if (key === "ArrowUp") moveTo(index < 0 ? 0 : index - 1);
    else if (key === "Home") moveTo(0);
    else if (key === "End") moveTo(items.length - 1);
    else if (key === "ArrowLeft" || key === "ArrowRight") {
      if (current?.type !== "category" || searching || current.collapsed === (key === "ArrowLeft")) return false;
      toggleCategory(current.name);
    } else if (key === "Enter" || key === "F2") {
      if (current?.type === "category") toggleCategory(current.name);
      else if (current) editing.beginEdit(current.property);
    } else return false;
    return true;
  }

  function handleKeyDown(e: KeyboardEvent<HTMLDivElement>) {
    // Keys typed into a text box or dropdown are theirs.
    if (e.target === e.currentTarget && onKey(e.key)) e.preventDefault();
  }

  /** Drags the name/value splitter — the split is a fraction of the grid's width, so it survives resizing the tool window. */
  function startSplitDrag(e: ReactPointerEvent<HTMLSpanElement>) {
    const rows = rowsRef.current;
    if (!rows || e.button !== 0) return;
    e.preventDefault();
    e.stopPropagation();
    const left = rows.getBoundingClientRect().left;
    const width = rows.clientWidth;
    trackPointer(e, (ev) => setSplit(clamp((ev.clientX - left) / width, MIN_SPLIT, MAX_SPLIT)));
  }

  /** Drags the description pane's top edge — taller upward. */
  function startDescriptionDrag(e: ReactPointerEvent<HTMLDivElement>) {
    if (e.button !== 0) return;
    e.preventDefault();
    const startY = e.clientY;
    const startHeight = descriptionHeight;
    trackPointer(e, (ev) => setDescriptionHeight(clamp(startHeight - (ev.clientY - startY), MIN_DESCRIPTION, MAX_DESCRIPTION)));
  }

  return { handleKeyDown, startSplitDrag, startDescriptionDrag };
}

export type PropertyGridInput = ReturnType<typeof propertyGridInput>;
