import { useEffect, useRef } from "react";
import { hostFiles } from "./hostFiles";

// Watching for changes made outside the IDE (a terminal, git, another
// editor, a build). Each part of the IDE names the folders it shows — a file
// explorer its listed folders, the editor its open files' folders — and the
// host watches their union.

/** How long the wanted folders settle before the union is sent, in ms. */
const SYNC_DELAY_MS = 50;

/** What each part wants watched. */
const wanted = new Map<string, string[]>();
let timer: ReturnType<typeof setTimeout> | undefined;
/** The union last sent, so an unchanged set isn't sent again. */
let sent = "";

function sync() {
  clearTimeout(timer);
  timer = setTimeout(() => {
    const folders = [...new Set([...wanted.values()].flat())].sort();
    const key = folders.join("\n");
    if (key === sent) return;
    sent = key;
    hostFiles.watch(folders).catch((e) => console.warn("fs_watch:", e));
  }, SYNC_DELAY_MS);
}

/** Watches `folders` while the calling component is mounted (`key` names its part). */
export function useWatchedFolders(key: string, folders: Iterable<string>) {
  const list = [...new Set(folders)].sort();
  const id = list.join("\n");
  useEffect(() => {
    wanted.set(key, id ? id.split("\n") : []);
    sync();
    return () => {
      wanted.delete(key);
      sync();
    };
  }, [key, id]);
}

/** Calls `handler` with each batch of paths changed outside the IDE. */
export function useFilesChanged(handler: (paths: string[]) => void) {
  const ref = useRef(handler);
  ref.current = handler;
  useEffect(() => {
    const unlisten = hostFiles.onChanged((paths) => ref.current(paths));
    return () => {
      void unlisten.then((u) => u());
    };
  }, []);
}
