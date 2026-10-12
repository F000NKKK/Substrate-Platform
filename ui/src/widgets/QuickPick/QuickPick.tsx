import { useEffect, useLayoutEffect, useRef, useState, type KeyboardEvent, type ReactNode } from "react";
import { ViewportPortal } from "../../infra/portal";
import { HighlightedText } from "../HighlightedText";
import { IconSearch } from "../../infra/icons";
import "./QuickPick.css";

export interface QuickPickItem {
  key: string;
  icon?: ReactNode;
  label: string;
  /** Character offsets into `label` to highlight (a fuzzy match). */
  highlights?: readonly number[];
  /** Dimmed, right after the label (a symbol's container). */
  description?: string;
  /** Dimmed, at the row's end (a file path and line). */
  detail?: string;
}

export interface QuickPickFilter {
  id: string;
  label: string;
  /** The query prefix that switches to this filter (`"f:"`), shown as a hint. */
  prefix?: string;
}

export interface QuickPickProps<T extends QuickPickItem> {
  query: string;
  onQueryChange: (query: string) => void;
  placeholder?: string;
  /** Mode tabs under the field (All, Files, Types, ...). */
  filters?: readonly QuickPickFilter[];
  activeFilter?: string;
  onFilterChange?: (id: string) => void;
  items: readonly T[];
  /** Enter or a click — `e.ctrlKey` etc. tell how. */
  onAccept: (item: T, how: { ctrl: boolean; shift: boolean }) => void;
  /** Enter with no items (e.g. a `:42` go-to-line query); returns whether it did something. */
  onAcceptQuery?: () => boolean;
  /** The highlighted row changed (keyboard or hover) — e.g. to preview it. */
  onActiveChange?: (item: T | null) => void;
  onClose: () => void;
  /** Shown instead of the list when there are no items. */
  emptyText?: ReactNode;
  /** A thin progress bar under the field while results are on their way. */
  busy?: boolean;
  /** A line under the list (hints, counts). */
  footer?: ReactNode;
  width?: number;
}

/**
 * Visual Studio's Code Search / VS Code's Quick Open: a popup at the top
 * center with a search field, optional mode tabs, and a keyboard-driven
 * result list — Up/Down/PageUp/PageDown move, Enter accepts, Escape or a
 * click outside closes, Tab / Shift+Tab cycle the filters.
 */
export function QuickPick<T extends QuickPickItem>({
  query,
  onQueryChange,
  placeholder,
  filters,
  activeFilter,
  onFilterChange,
  items,
  onAccept,
  onAcceptQuery,
  onActiveChange,
  onClose,
  emptyText,
  busy,
  footer,
  width = 640,
}: QuickPickProps<T>) {
  const [active, setActive] = useState(0);
  const listRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const index = items.length === 0 ? -1 : Math.min(active, items.length - 1);

  // A new result set starts at its top.
  useEffect(() => setActive(0), [items]);

  const activeItem = index >= 0 ? items[index] : null;
  useEffect(() => onActiveChange?.(activeItem), [activeItem, onActiveChange]);

  useLayoutEffect(() => {
    const row = listRef.current?.querySelector<HTMLElement>(`[data-index="${index}"]`);
    row?.scrollIntoView({ block: "nearest" });
  }, [index]);

  // Whatever had focus before (an editor) gets it back when this closes without opening anything.
  useEffect(() => {
    const previous = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    inputRef.current?.focus();
    return () => {
      if (document.activeElement === document.body || !document.activeElement) previous?.focus();
    };
  }, []);

  function move(delta: number) {
    if (items.length === 0) return;
    setActive((i) => Math.max(0, Math.min(items.length - 1, Math.min(i, items.length - 1) + delta)));
  }

  function cycleFilter(delta: number) {
    if (!filters?.length || !onFilterChange) return;
    const at = Math.max(0, filters.findIndex((f) => f.id === activeFilter));
    onFilterChange(filters[(at + delta + filters.length) % filters.length].id);
  }

  function onKeyDown(e: KeyboardEvent) {
    const page = 10;
    let handled = true;
    if (e.key === "ArrowDown") move(1);
    else if (e.key === "ArrowUp") move(-1);
    else if (e.key === "PageDown") move(page);
    else if (e.key === "PageUp") move(-page);
    else if (e.key === "Tab" && filters?.length) cycleFilter(e.shiftKey ? -1 : 1);
    else if (e.key === "Escape") onClose();
    else if (e.key === "Enter") {
      if (activeItem) onAccept(activeItem, { ctrl: e.ctrlKey || e.metaKey, shift: e.shiftKey });
      else if (!onAcceptQuery?.()) handled = false;
    } else handled = false;
    if (handled) {
      e.preventDefault();
      e.stopPropagation();
    }
  }

  return (
    <ViewportPortal>
      <div className="sp-quickpick-backdrop" onPointerDown={onClose} />
      <div className="sp-quickpick" role="dialog" style={{ width }} onKeyDown={onKeyDown}>
        <div className="sp-quickpick-field">
          <IconSearch size={15} className="sp-quickpick-search-icon" />
          <input
            ref={inputRef}
            className="sp-quickpick-input"
            value={query}
            placeholder={placeholder}
            spellCheck={false}
            autoComplete="off"
            role="combobox"
            aria-expanded="true"
            aria-controls="sp-quickpick-list"
            aria-activedescendant={index >= 0 ? `sp-quickpick-item-${index}` : undefined}
            onChange={(e) => onQueryChange(e.target.value)}
          />
        </div>
        <div className="sp-quickpick-progress" data-busy={busy || undefined} />
        {filters && filters.length > 0 && (
          <div className="sp-quickpick-filters" role="tablist">
            {filters.map((f) => (
              <button
                key={f.id}
                type="button"
                role="tab"
                aria-selected={f.id === activeFilter}
                className="sp-quickpick-filter"
                data-active={f.id === activeFilter || undefined}
                title={f.prefix}
                onMouseDown={(e) => e.preventDefault()}
                onClick={() => onFilterChange?.(f.id)}
              >
                {f.label}
                {f.prefix && <span className="sp-quickpick-filter-prefix">{f.prefix}</span>}
              </button>
            ))}
          </div>
        )}
        <div className="sp-quickpick-list" id="sp-quickpick-list" role="listbox" ref={listRef}>
          {items.length === 0 && emptyText && <div className="sp-quickpick-empty">{emptyText}</div>}
          {items.map((item, i) => (
            <div
              key={item.key}
              id={`sp-quickpick-item-${i}`}
              data-index={i}
              role="option"
              aria-selected={i === index}
              className="sp-quickpick-item"
              data-active={i === index || undefined}
              onMouseMove={() => i !== index && setActive(i)}
              onMouseDown={(e) => e.preventDefault()}
              onClick={(e) => onAccept(item, { ctrl: e.ctrlKey || e.metaKey, shift: e.shiftKey })}
            >
              {item.icon && <span className="sp-quickpick-item-icon">{item.icon}</span>}
              <HighlightedText className="sp-quickpick-item-label" text={item.label} positions={item.highlights} />
              {item.description && <span className="sp-quickpick-item-description">{item.description}</span>}
              {item.detail && <span className="sp-quickpick-item-detail">{item.detail}</span>}
            </div>
          ))}
        </div>
        {footer && <div className="sp-quickpick-footer">{footer}</div>}
      </div>
    </ViewportPortal>
  );
}
