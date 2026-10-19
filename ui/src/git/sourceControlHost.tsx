import { createContext, useContext, type ReactNode } from "react";

/** An open file, as Source Control sees it: where it is and what's on disk. */
export interface SourceControlFile {
  path: string;
  diskContent: string;
}

/**
 * What Source Control needs of the product it runs in — the folder that's
 * open, its open files, and the few things only the product can do with
 * them. A product provides one with `SourceControlProvider`.
 */
export interface SourceControlHost {
  /** The open folder (the repository Git Changes shows), or `null`. */
  root: string | null;
  openFiles: readonly SourceControlFile[];
  /** Reads the open files again — a checkout, pull or stash rewrote them. */
  reloadFromDisk(): Promise<void>;
  /** Asks about unsaved changes before the open folder changes; `false`: cancelled. */
  confirmUnsaved(): Promise<boolean>;
  /** Opens `folder` as the project (a clone, once done). */
  openFolder(folder: string): Promise<void>;
  /** Lets the user pick a folder (where to clone); `null`: cancelled. */
  chooseFolder(title: string): Promise<string | null>;
}

const NO_HOST: SourceControlHost = {
  root: null,
  openFiles: [],
  reloadFromDisk: async () => {},
  confirmUnsaved: async () => true,
  openFolder: async () => {},
  chooseFolder: async () => null,
};

const HostContext = createContext<SourceControlHost>(NO_HOST);

/** Gives Source Control's windows, menu and host the product's `host`. */
export function SourceControlProvider({ host, children }: { host: SourceControlHost; children: ReactNode }) {
  return <HostContext.Provider value={host}>{children}</HostContext.Provider>;
}

export function useSourceControlHost(): SourceControlHost {
  return useContext(HostContext);
}
