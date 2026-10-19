import { substrate } from "../infra/host/host.generated";
import { invokeCommand } from "../infra/host/invokeCommand";

// The `git_*` commands (`src-tauri/src/git.rs`), typed. Paths in a
// `FileChange` are repo-relative and `/`-separated; `absPath` is the file on
// disk, in the same form as the paths open tabs use.

export type ChangeKind = "modified" | "added" | "deleted" | "renamed" | "copied" | "typeChanged" | "untracked" | "conflicted";

export interface FileChange {
  path: string;
  origPath: string | null;
  kind: ChangeKind;
  absPath: string;
}

export type GitOperation = "merge" | "rebase" | "cherryPick" | "revert";

export interface RepoStatus {
  repoRoot: string;
  /** `null` on a detached HEAD. */
  branch: string | null;
  /** `null` before the first commit. */
  head: string | null;
  upstream: string | null;
  ahead: number;
  behind: number;
  staged: FileChange[];
  unstaged: FileChange[];
  conflicted: FileChange[];
  stashes: number;
  operation: GitOperation | null;
}

export interface CommitInfo {
  hash: string;
  shortHash: string;
  parents: string[];
  author: string;
  email: string;
  /** Unix seconds. */
  date: number;
  refs: string[];
  subject: string;
}

export interface BranchInfo {
  name: string;
  remote: boolean;
  current: boolean;
  upstream: string | null;
  ahead: number;
  behind: number;
  gone: boolean;
  hash: string;
  date: number;
  subject: string;
}

export interface StashInfo {
  index: number;
  message: string;
  date: number;
}

/** One side of a diff: a commit, `"INDEX"`, `"WORKTREE"`, or `""` for "didn't exist". */
export interface DiffSide {
  rev: string;
  path: string;
}

export interface DiffTexts {
  left: string | null;
  right: string | null;
  binary: boolean;
}

export type RemoteOp = "fetch" | "pull" | "push" | "sync";
export type StashMode = "all" | "keepIndex" | "staged";

export interface GlobalConfig {
  version: string | null;
  values: Record<string, string>;
}

export const gitApi = {
  status: (root: string) => invokeCommand<RepoStatus | null>(substrate.git.gitStatus, { root }),
  stage: (root: string, paths: string[]) => invokeCommand<void>(substrate.git.gitStage, { root, paths }),
  unstage: (root: string, paths: string[]) => invokeCommand<void>(substrate.git.gitUnstage, { root, paths }),
  discard: (root: string, items: { path: string; origPath: string | null; kind: ChangeKind; staged: boolean }[]) =>
    invokeCommand<void>(substrate.git.gitDiscard, { root, items }),
  commit: (root: string, message: string, amend: boolean, stageAll: boolean) =>
    invokeCommand<{ hash: string; subject: string }>(substrate.git.gitCommit, { root, message, amend, stageAll }),
  lastMessage: (root: string) => invokeCommand<string>(substrate.git.gitLastMessage, { root }),
  diffTexts: (root: string, left: DiffSide, right: DiffSide) => invokeCommand<DiffTexts>(substrate.git.gitDiffTexts, { root, left, right }),
  headText: (root: string, path: string) => invokeCommand<string | null>(substrate.git.gitHeadText, { root, path }),
  log: (root: string, rev: string, skip: number, max: number) => invokeCommand<CommitInfo[]>(substrate.git.gitLog, { root, rev, skip, max }),
  commitFiles: (root: string, hash: string) => invokeCommand<FileChange[]>(substrate.git.gitCommitFiles, { root, hash }),
  branches: (root: string) => invokeCommand<BranchInfo[]>(substrate.git.gitBranches, { root }),
  switchTo: (root: string, name: string, remote: boolean) => invokeCommand<void>(substrate.git.gitSwitch, { root, name, remote }),
  createBranch: (root: string, name: string, start: string, checkout: boolean) => invokeCommand<void>(substrate.git.gitBranchCreate, { root, name, start, checkout }),
  deleteBranch: (root: string, name: string, force: boolean) => invokeCommand<void>(substrate.git.gitBranchDelete, { root, name, force }),
  remote: (root: string, op: RemoteOp) => invokeCommand<void>(substrate.git.gitRemote, { root, op }),
  integrate: (root: string, op: "merge" | "rebase", branch: string) => invokeCommand<void>(substrate.git.gitIntegrate, { root, op, branch }),
  operation: (root: string, goOn: boolean) => invokeCommand<void>(substrate.git.gitOperation, { root, goOn }),
  clone: (url: string, parent: string, name: string) => invokeCommand<string>(substrate.git.gitClone, { url, parent, name }),
  init: (root: string, branch: string) => invokeCommand<void>(substrate.git.gitInit, { root, branch }),
  stashList: (root: string) => invokeCommand<StashInfo[]>(substrate.git.gitStashList, { root }),
  stashPush: (root: string, message: string, mode: StashMode) => invokeCommand<void>(substrate.git.gitStashPush, { root, message, mode }),
  stashOp: (root: string, index: number, op: "apply" | "pop" | "drop") => invokeCommand<void>(substrate.git.gitStashOp, { root, index, op }),
  configGet: () => invokeCommand<GlobalConfig>(substrate.git.gitConfigGet),
  configSet: (key: string, value: string) => invokeCommand<void>(substrate.git.gitConfigSet, { key, value }),
};

/** The status letter Git Changes shows for a change, Visual Studio's. */
export function changeLetter(kind: ChangeKind): string {
  switch (kind) {
    case "modified":
    case "typeChanged":
      return "M";
    case "added":
      return "A";
    case "deleted":
      return "D";
    case "renamed":
      return "R";
    case "copied":
      return "C";
    case "untracked":
      return "U";
    case "conflicted":
      return "!";
  }
}

/** The change kind spelled out, for tooltips. English, shown through `t`. */
export function changeKindLabel(kind: ChangeKind): string {
  switch (kind) {
    case "modified":
      return "Modified";
    case "typeChanged":
      return "Type changed";
    case "added":
      return "Added";
    case "deleted":
      return "Deleted";
    case "renamed":
      return "Renamed";
    case "copied":
      return "Copied";
    case "untracked":
      return "Untracked";
    case "conflicted":
      return "Conflict";
  }
}

/** A repo-relative path's file name and the folder it's in. */
export function splitRepoPath(path: string): { name: string; dir: string } {
  const slash = path.lastIndexOf("/");
  return slash < 0 ? { name: path, dir: "" } : { name: path.slice(slash + 1), dir: path.slice(0, slash) };
}
