import type { ButtonHTMLAttributes, MouseEvent, ReactNode } from "react";
import "./Tab.css";

export type TabOrientation = "horizontal" | "horizontal-dock" | "vertical-left" | "vertical-right" | "list";

export interface TabProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  active?: boolean;
  orientation?: TabOrientation;
  /** Shows a small close glyph; omit for a tab that can't be closed (e.g. a product's permanent main tab). */
  onRequestClose?: () => void;
  /** Shows a pin glyph in place of the close one — clicking it fires `onRequestUnpin`. */
  pinned?: boolean;
  onRequestUnpin?: () => void;
  children: ReactNode;
}

/**
 * The one tab/list-item implementation in the platform — the rotated
 * edge-strip tabs (DockStrip), the horizontal document tabs (CenterDock),
 * and a Settings window's category list (`orientation="list"`) all render
 * through this rather than each hand-rolling their own button/label markup.
 */
export function Tab({ active, orientation = "horizontal", onRequestClose, pinned, onRequestUnpin, className, children, ...props }: TabProps) {
  const classes = ["sp-tab", `sp-tab--${orientation}`, className].filter(Boolean).join(" ");

  function handleCloseClick(e: MouseEvent) {
    e.stopPropagation();
    onRequestClose?.();
  }

  function handleUnpinClick(e: MouseEvent) {
    e.stopPropagation();
    onRequestUnpin?.();
  }

  return (
    <button className={classes} data-active={active} data-pinned={pinned || undefined} {...props}>
      <span className="sp-tab-label">{children}</span>
      {pinned ? (
        <span className="sp-tab-close sp-tab-pin" role="button" aria-label="Unpin tab" title="Unpin" onClick={handleUnpinClick}>
          📌
        </span>
      ) : onRequestClose && (
        <span className="sp-tab-close" role="button" aria-label="Close tab" onClick={handleCloseClick}>
          ×
        </span>
      )}
    </button>
  );
}
