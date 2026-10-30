import { useT } from "../../../infra/i18n";
import { fileTypeOf } from "../../../infra/fileTypes";
import { useEffect, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import type { EditorProps } from "../registry/EditorBase";
import { ZoomView } from "../../ZoomView";
import "./ImageEditor.css";

export interface ImageEditorProps extends EditorProps {
  /**
   * Reads the file's raw bytes (e.g. `(path) => invoke("file_read_bytes", { path })`
   * backed by the Rust core's `dir::read_file_bytes`). Omit to display the
   * file straight through Tauri's asset protocol instead — which requires the
   * host to have `assetProtocol` enabled with a scope covering `path`.
   */
  loadBytes?: (path: string) => Promise<ArrayBuffer | Uint8Array | number[]>;
}

/** The size a browser gives an image without its own (an SVG without width/height). */
const UNSIZED: [number, number] = [300, 150];

/**
 * A sibling branch off `EditorBase`, not a `TextEditor` specialization —
 * images aren't text, so there's no `content`/`onChange` round-trip here,
 * just a read-only view, zoomed and panned (`ZoomView`); `content` is ignored, so a host shouldn't read
 * the file as text first (see `isBinaryFile`). By default uses Tauri's
 * asset protocol to display the file directly rather than round-tripping
 * bytes through IPC; pass `loadBytes` to go through IPC anyway.
 */
export function ImageEditor({ path, loadBytes, contentVersion }: ImageEditorProps) {
  // The version in the URL: the webview caches by it, so a file changed on
  // disk (the host bumps `contentVersion`) shows anew.
  const t = useT();
  const [attempt, retry] = useState(0);
  const url = () => `${convertFileSrc(path)}?v=${contentVersion ?? 0}&retry=${attempt}`;
  const [src, setSrc] = useState<string | null>(loadBytes ? null : url());
  const [error, setError] = useState<string | null>(null);
  const [size, setSize] = useState<[number, number] | null>(null);

  useEffect(() => {
    if (!loadBytes) {
      setSrc(url());
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
        objectUrl = URL.createObjectURL(new Blob([data as BlobPart], { type: fileTypeOf(path)?.mime ?? "application/octet-stream" }));
        setSrc(objectUrl);
      })
      .catch((err) => {
        if (!cancelled) setError(String(err));
      });
    return () => {
      cancelled = true;
      if (objectUrl) URL.revokeObjectURL(objectUrl);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [path, loadBytes, contentVersion, attempt]);

  // The image's own size, for the zoom's 100%.
  useEffect(() => {
    setSize(null);
    setError(null);
    if (!src) return;
    const img = new Image();
    img.onload = () => setSize([img.naturalWidth || UNSIZED[0], img.naturalHeight || UNSIZED[1]]);
    img.onerror = () => setError(path);
    img.src = src;
    return () => void (img.onload = img.onerror = null);
  }, [src, path]);

  return (
    <div className="sp-image-editor">
      {error ? (
        <div className="sp-image-editor-error" role="alert">
          <p>{t("The image could not be loaded: {error}", { error })}</p>
          <button type="button" onClick={() => retry(value => value + 1)}>{t("Retry")}</button>
        </div>
      ) : (
        src &&
        size && (
          <ZoomView width={size[0]} height={size[1]}>
            <img src={src} alt={path} draggable={false} />
          </ZoomView>
        )
      )}
    </div>
  );
}
