import { useState, type KeyboardEvent, type ReactNode } from "react";
import { IconButton } from "../IconButton";
import { IconClose } from "../../infra/icons";
import { useDraggable } from "../../infra/hooks";
import { ViewportPortal } from "../../infra/portal";
import { useT } from "../../infra/i18n";
import "./FloatingWindow.css";

export interface FloatingWindowProps {
  title: string;
  /** A small glyph before the title. */
  icon?: ReactNode;
  /** The close button and Escape. */
  onClose: () => void;
  /** Width in px (default 460); the height follows the content. */
  width?: number;
  /** Where it first appears — default: horizontally centered, near the top. */
  initialPosition?: { x: number; y: number };
  /** Controls in the title bar, before the close button. */
  headerExtra?: ReactNode;
  className?: string;
  children: ReactNode;
}

/**
 * A modeless tool window floating over the IDE — draggable by its title
 * bar, closed with its close button or Escape — in the same family as
 * Quick Find (raised surface, strong border, shadow). For dialogs that work
 * alongside the editor, like Visual Studio's Find and Replace window.
 */
export function FloatingWindow({ title, icon, onClose, width = 460, initialPosition, headerExtra, className, children }: FloatingWindowProps) {
  const t = useT();
  const [initial] = useState(() => initialPosition ?? { x: Math.max(8, Math.round((window.innerWidth - width) / 2)), y: 96 });
  const { pos, handlers } = useDraggable(initial);
  // Kept on screen however far it was dragged (or the window shrank).
  const left = Math.min(Math.max(pos.x, 8 - width + 80), window.innerWidth - 80);
  const top = Math.min(Math.max(pos.y, 0), window.innerHeight - 36);

  function onKeyDown(e: KeyboardEvent) {
    if (e.key === "Escape" && !e.defaultPrevented) {
      e.preventDefault();
      e.stopPropagation();
      onClose();
    }
  }

  return (
    <ViewportPortal>
      <div
        className={["sp-floating-window", className].filter(Boolean).join(" ")}
        role="dialog"
        aria-label={title}
        style={{ left, top, width }}
        onKeyDown={onKeyDown}
      >
        <div className="sp-floating-window-header" {...handlers}>
          {icon && <span className="sp-floating-window-icon">{icon}</span>}
          <span className="sp-floating-window-title">{title}</span>
          {headerExtra}
          <IconButton aria-label={t("Close")} title={t("Close")} onClick={onClose}>
            <IconClose />
          </IconButton>
        </div>
        <div className="sp-floating-window-body">{children}</div>
      </div>
    </ViewportPortal>
  );
}
