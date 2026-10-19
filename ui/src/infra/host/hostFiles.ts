import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { DirEntry } from "../../widgets/FileTree/useDirectoryTree";
import { events } from "./host.generated";
import { invokeSubstrate } from "./invokeSubstrate";

/**
 * Folders and files through the host plugin (feature `fs`): writes stay in
 * the workspace, reads go anywhere; `watch` reports changes made outside
 * the IDE through `onChanged`.
 */
export const hostFiles = {
  read: (path: string) => invokeSubstrate<string>("file_read", { path }),
  write: (path: string, contents: string) => invokeSubstrate<void>("file_write", { path, contents }),
  list: (path: string) => invokeSubstrate<DirEntry[]>("dir_list", { path }),
  createFile: (path: string) => invokeSubstrate<void>("dir_create_file", { path }),
  createDir: (path: string) => invokeSubstrate<void>("dir_create_dir", { path }),
  rename: (from: string, to: string) => invokeSubstrate<void>("dir_rename", { from, to }),
  copy: (from: string, to: string) => invokeSubstrate<void>("dir_copy", { from, to }),
  remove: (path: string) => invokeSubstrate<void>("dir_remove", { path }),
  /** A folder or file the user picked joins the workspace. */
  allow: (path: string) => invokeSubstrate<void>("allow_path", { path }),
  /** Watches exactly `folders` (replacing what was watched). */
  watch: (folders: string[]) => invokeSubstrate<void>("fs_watch", { folders }),
  onChanged: (handler: (paths: string[]) => void): Promise<UnlistenFn> => listen<string[]>(events.filesChanged, (e) => handler(e.payload)),
};
