import { createContext, useContext, type ReactNode } from "react";

/** An open document, as the product holds it. */
export interface HostDocument {
  /** Stable for the document's lifetime: its center-dock tab's id (the shell's `activeCenterId` when it's in front). */
  id: string;
  path: string;
  /** The editor's text as the product last knew it (`textLoaded`). */
  content: string;
  /** What's on disk, as last read or written. */
  diskContent: string;
  /** `content` holds the file's text — false for a file shown only in a binary view. */
  textLoaded: boolean;
}

/**
 * What the platform's modules need of the product they run in — the folder
 * that's open, its open documents, and the few things only the product can
 * do with them. Source Control and Find in Files use it; a product provides
 * one with `IdeHostProvider`.
 */
export interface IdeHost {
  /** The open folder (the project), or `null`. */
  root: string | null;
  documents: readonly HostDocument[];
  /** Puts `text` into document `id`'s editor (undoable, left unsaved). */
  setText(id: string, text: string): void;
  /** Reads the open documents again — something rewrote them on disk. */
  reloadFromDisk(): Promise<void>;
  /** Asks about unsaved changes before the open folder changes; `false`: cancelled. */
  confirmUnsaved(): Promise<boolean>;
  /** Opens `folder` as the project. */
  openFolder(folder: string): Promise<void>;
  /** Lets the user pick a folder; `null`: cancelled. */
  chooseFolder(title: string): Promise<string | null>;
}

const NO_HOST: IdeHost = {
  root: null,
  documents: [],
  setText: () => {},
  reloadFromDisk: async () => {},
  confirmUnsaved: async () => true,
  openFolder: async () => {},
  chooseFolder: async () => null,
};

const HostContext = createContext<IdeHost>(NO_HOST);

/** Gives the platform's modules the product's `host`. */
export function IdeHostProvider({ host, children }: { host: IdeHost; children?: ReactNode }) {
  return <HostContext.Provider value={host}>{children}</HostContext.Provider>;
}

export function useIdeHost(): IdeHost {
  return useContext(HostContext);
}
