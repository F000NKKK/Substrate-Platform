import { useEffect, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import type { EditorProps } from "./EditorBase";
import "./ImageEditor.css";

const MIME_BY_EXTENSION: Record<string, string> = {
  png: "image/png",
  jpg: "image/jpeg",
  jpeg: "image/jpeg",
  gif: "image/gif",
  webp: "image/webp",
  bmp: "image/bmp",
  ico: "image/x-icon",
  svg: "image/svg+xml",
};

export interface ImageEditorProps extends EditorProps {
  /**
   * Reads the file's raw bytes (e.g. `(path) => invoke("file_read_bytes", { path })`
   * backed by the Rust core's `dir::read_file_bytes`). Omit to display the
   * file straight through Tauri's asset protocol instead — which requires the
   * host to have `assetProtocol` enabled with a scope covering `path`.
   */
  loadBytes?: (path: string) => Promise<ArrayBuffer | Uint8Array | number[]>;
}

/**
 * A sibling branch off `EditorBase`, not a `TextEditor` specialization —
 * images aren't text, so there's no `content`/`onChange` round-trip here,
 * just a read-only view; `content` is ignored, so a host shouldn't read
 * the file as text first (see `isBinaryFile`). By default uses Tauri's
 * asset protocol to display the file directly rather than round-tripping
 * bytes through IPC; pass `loadBytes` to go through IPC anyway.
 */
export function ImageEditor({ path, loadBytes }: ImageEditorProps) {
  const [src, setSrc] = useState<string | null>(loadBytes ? null : convertFileSrc(path));
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!loadBytes) {
      setSrc(convertFileSrc(path));
      return;
    }
    let objectUrl: string | null = null;
    let cancelled = false;
    setSrc(null);
    setError(null);
    loadBytes(path)
      .then((bytes) => {
        if (cancelled) return;
        const data = Array.isArray(bytes) ? new Uint8Array(bytes) : bytes;
        const ext = path.split(".").pop()?.toLowerCase() ?? "";
        objectUrl = URL.createObjectURL(new Blob([data as BlobPart], { type: MIME_BY_EXTENSION[ext] ?? "application/octet-stream" }));
        setSrc(objectUrl);
      })
      .catch((err) => {
        if (!cancelled) setError(String(err));
      });
    return () => {
      cancelled = true;
      if (objectUrl) URL.revokeObjectURL(objectUrl);
    };
  }, [path, loadBytes]);

  return (
    <div className="sp-image-editor">
      {error ? <p className="sp-image-editor-error">{error}</p> : src && <img src={src} alt={path} />}
    </div>
  );
}
