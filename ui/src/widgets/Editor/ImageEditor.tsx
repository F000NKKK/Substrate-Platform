import { convertFileSrc } from "@tauri-apps/api/core";
import type { EditorProps } from "./EditorBase";
import "./ImageEditor.css";

/**
 * A sibling branch off `EditorBase`, not a `TextEditor` specialization —
 * images aren't text, so there's no `content`/`onChange` round-trip here,
 * just a read-only view. Uses Tauri's asset protocol to display the file
 * directly rather than round-tripping bytes through IPC as base64.
 */
export function ImageEditor({ path }: EditorProps) {
  return (
    <div className="sp-image-editor">
      <img src={convertFileSrc(path)} alt={path} />
    </div>
  );
}
