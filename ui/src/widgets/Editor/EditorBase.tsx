import type { ComponentType } from "react";

/**
 * The contract every file editor implements, whatever it renders underneath
 * (CodeMirror text, an image viewer, a grid) — the host (an `Editor` panel
 * managing open-file tabs) only ever needs this shape, never the concrete
 * editor kind.
 */
export interface EditorProps {
  /** Absolute path — an editor that cares what kind of file this is derives it from here. */
  path: string;
  /** The file's content, as last read from (or committed to) disk. */
  content: string;
  /** Called with the new content whenever the user changes it — the host owns deciding when/whether to persist it. */
  onChange: (next: string) => void;
  readOnly?: boolean;
  /**
   * Bump to make the editor take `content` as its new document — for when
   * the file changed underneath it (reloaded from disk). Without a bump,
   * `content` only seeds the editor at mount; later values are just the
   * editor's own edits flowing back through the host, and are ignored. The
   * reload itself does not fire `onChange`.
   */
  contentVersion?: number;
}

export type EditorComponent = ComponentType<EditorProps>;
