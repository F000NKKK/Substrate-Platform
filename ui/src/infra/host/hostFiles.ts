import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { DirEntry } from "../../widgets/FileTree/useDirectoryTree";
import { events, substrate } from "./host.generated";
import { invokeCommand } from "./invokeCommand";

/**
 * Folders and files through the host plugin (feature `fs`): writes stay in
 * the workspace, reads go anywhere; `watch` reports changes made outside
 * the IDE through `onChanged`.
 */
export const hostFiles = {
  read: (path: string) => invokeCommand<string>(substrate.fs.fileRead, { path }),
  write: (path: string, contents: string) => invokeCommand<void>(substrate.fs.fileWrite, { path, contents }),
  list: (path: string) => invokeCommand<DirEntry[]>(substrate.fs.dirList, { path }),
  createFile: (path: string) => invokeCommand<void>(substrate.fs.dirCreateFile, { path }),
  createDir: (path: string) => invokeCommand<void>(substrate.fs.dirCreateDir, { path }),
  rename: (from: string, to: string) => invokeCommand<void>(substrate.fs.dirRename, { from, to }),
  copy: (from: string, to: string) => invokeCommand<void>(substrate.fs.dirCopy, { from, to }),
  remove: (path: string) => invokeCommand<void>(substrate.fs.dirRemove, { path }),
  /** A folder or file the user picked joins the workspace. */
  allow: (path: string) => invokeCommand<void>(substrate.fs.allowPath, { path }),
  /** Watches exactly `folders` (replacing what was watched). */
  watch: (folders: string[]) => invokeCommand<void>(substrate.fs.fsWatch, { folders }),
  onChanged: (handler: (paths: string[]) => void): Promise<UnlistenFn> => listen<string[]>(events.filesChanged, (e) => handler(e.payload)),
};
