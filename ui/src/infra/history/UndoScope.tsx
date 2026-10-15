import type { ReactNode } from "react";

/**
 * Marks a window (Options, Project Properties) as an undo scope: while the
 * focus is in it, Ctrl+Z and Ctrl+Y act on history `id` (see
 * `documentHistory`'s `undoable`) rather than the active document's.
 * `display: contents` keeps it out of the layout.
 */
export function UndoScope({ id, children }: { id: string; children: ReactNode }) {
  return (
    <div data-undo-scope={id} style={{ display: "contents" }}>
      {children}
    </div>
  );
}
