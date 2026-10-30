// Generic filesystem actions and confirmation dialogs shared by product explorers.
import { useState } from "react";
import { hostFiles } from "../../infra/host";
import { errorText, useT } from "../../infra/i18n";
import { notify } from "../Notifications";
import type { DirEntry } from "../FileTree/useDirectoryTree";
import { FileActionDialog } from "./FileActionDialog";
import { setFileClipboard, useFileClipboard } from "./clipboard";

export interface FileActionEntry extends DirEntry {
  /** Sidecars that move/copy/delete with this item; the suffix is appended to its new name. */
  companions?: readonly { path: string; suffix: string }[];
}
type Pending = { kind: "delete"; entries: readonly FileActionEntry[] }
  | { kind: "rename"; entry: FileActionEntry }
  | { kind: "file"; name: string; contents: string }
  | { kind: "folder" };
export function joinFilePath(folder: string, name: string) { return folder.replace(/[\\/]$/, "") + (folder.includes("\\") ? "\\" : "/") + name; }
function validName(name: string) {
  return name !== "." && name !== ".." && name.length > 0 && !/[/\\:*?"<>|\u0000-\u001f]/.test(name)
    && !/[. ]$/.test(name) && !/^(con|prn|aux|nul|com[1-9]|lpt[1-9])(\.|$)/i.test(name);
}
function contained(path: string, parent: string) { return path === parent || path.startsWith(parent.replace(/[\\/]$/, "") + (parent.includes("\\") ? "\\" : "/")); }
function topEntries(entries: readonly FileActionEntry[]) { return entries.filter(entry => !entries.some(other => other !== entry && other.kind === "dir" && contained(entry.path, other.path))); }

export function useFileActions(folder: string, changed: () => void) {
  const t = useT(), clipboard = useFileClipboard();
  const [pending, setPending] = useState<Pending | null>(null);
  const [busy, setBusy] = useState(false);
  async function perform(action: () => Promise<void>) {
    if (busy) return;
    setBusy(true);
    try { await action(); }
    catch (error) { notify({ level: "error", message: errorText(error) }); }
    finally { setBusy(false); changed(); }
  }
  async function paste(destination = folder) {
    if (!clipboard) return;
    await perform(async () => {
      const taken = new Set((await hostFiles.list(destination)).map(entry => entry.name.toLowerCase()));
      for (const entry of topEntries(clipboard.entries)) {
        if (clipboard.operation === "cut" && joinFilePath(destination, entry.name) === entry.path) continue;
        let name = entry.name, number = 1;
        while (taken.has(name.toLowerCase())) {
          const dot = entry.kind === "file" ? entry.name.lastIndexOf(".") : -1;
          const stem = dot > 0 ? entry.name.slice(0, dot) : entry.name, extension = dot > 0 ? entry.name.slice(dot) : "";
          name = `${stem} (${number++})${extension}`;
        }
        const target = joinFilePath(destination, name);
        const operation = clipboard.operation === "cut" ? hostFiles.rename : hostFiles.copy;
        await operation(entry.path, target);
        for (const companion of entry.companions ?? []) await operation(companion.path, target + companion.suffix);
        taken.add(name.toLowerCase());
      }
      if (clipboard.operation === "cut") setFileClipboard(null);
    });
  }
  async function submit(value: string) {
    if (!pending) return;
    if (pending.kind === "delete") {
      for (const entry of topEntries(pending.entries)) {
        await hostFiles.remove(entry.path);
        for (const companion of entry.companions ?? []) await hostFiles.remove(companion.path);
      }
    } else {
      const name = value.trim();
      if (!validName(name)) throw new Error(t("Enter a valid portable file name."));
      const entries = await hostFiles.list(folder);
      if (entries.some(entry => entry.name.toLowerCase() === name.toLowerCase() && !(pending.kind === "rename" && entry.path === pending.entry.path)))
        throw new Error(t('"{name}" already exists here.', { name }));
      const target = joinFilePath(folder, name);
      if (pending.kind === "rename") {
        if (target !== pending.entry.path) {
          await hostFiles.rename(pending.entry.path, target);
          for (const companion of pending.entry.companions ?? []) await hostFiles.rename(companion.path, target + companion.suffix);
        }
      } else if (pending.kind === "folder") await hostFiles.createDir(target);
      else {
        await hostFiles.createFile(target);
        await hostFiles.write(target, pending.contents);
      }
    }
    changed();
  }
  const title = pending?.kind === "delete" ? t("Delete selected items") : pending?.kind === "rename" ? t("Rename") : pending?.kind === "folder" ? t("New Folder") : t("New File");
  return {
    busy, canPaste: clipboard !== null,
    remove: (entries: readonly FileActionEntry[]) => { if (entries.length) setPending({ kind: "delete", entries: [...entries] }); },
    rename: (entry: FileActionEntry) => setPending({ kind: "rename", entry }),
    newFile: (name = "file.txt", contents = "") => setPending({ kind: "file", name, contents }),
    newFolder: () => setPending({ kind: "folder" }),
    copy: (entries: readonly FileActionEntry[]) => setFileClipboard({ entries: [...entries], operation: "copy" }),
    cut: (entries: readonly FileActionEntry[]) => setFileClipboard({ entries: [...entries], operation: "cut" }),
    paste,
    dialog: pending && <FileActionDialog key={pending.kind} title={title}
      description={pending.kind === "delete" ? t("Delete these items and their sidecar files from disk? {names}", { names: pending.entries.map(entry => entry.name).join(", ") }) : undefined}
      initial={pending.kind === "delete" ? undefined : pending.kind === "rename" ? pending.entry.name : pending.kind === "file" ? pending.name : "New folder"}
      submit={submit} close={() => setPending(null)} />,
  };
}
