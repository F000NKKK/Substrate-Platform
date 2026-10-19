import { useEffect, useMemo, useState, type MouseEvent } from "react";
import { openFile } from "../documents/fileOpenBus";
import { errorText } from "../infra/i18n/errorText";
import { useT } from "../infra/i18n/i18n";
import { Icon } from "../infra/icons/IconProvider";
import { Button } from "../widgets/Button/Button";
import { ContextMenu, type ContextMenuItem } from "../widgets/ContextMenu/ContextMenu";
import { useContextMenu } from "../widgets/ContextMenu/useContextMenu";
import { IconButton } from "../widgets/IconButton/IconButton";
import { Spacer } from "../widgets/Layout/Layout";
import { checkoutBranch, localNameOf } from "./BranchPicker";
import { changeKindLabel, changeLetter, gitApi, splitRepoPath, type BranchInfo, type CommitInfo, type FileChange } from "./gitApi";
import { commitDiff, openDiff } from "./gitDocuments";
import { deleteBranch, mergeIntoCurrent, promptNewBranch, rebaseOnto, refreshGit, useGit } from "./gitStore";
import "./git.css";

const PAGE = 200;

/** Commit dates as Visual Studio shows them in the history — the local date and time. */
function formatDate(seconds: number): string {
  return new Date(seconds * 1000).toLocaleString(undefined, { dateStyle: "short", timeStyle: "short" });
}

/**
 * Visual Studio's Git Repository window, as a center document: the local
 * and remote branches on the left (checkout, new branch, merge, rebase,
 * delete), the selected branch's history in the middle, and the selected
 * commit's details and changed files — double-click one for its diff.
 */
export function GitRepositoryWindow() {
  const t = useT();
  const { root, status, branches, revision, loaded } = useGit();
  /** The branch whose history is shown — `null` follows what's checked out. */
  const [selected, setSelected] = useState<string | null>(null);
  const [commits, setCommits] = useState<CommitInfo[]>([]);
  const [more, setMore] = useState(false);
  const [commit, setCommit] = useState<CommitInfo | null>(null);
  const [error, setError] = useState<string | null>(null);
  const head = status?.head ?? null;
  const rev = selected ?? "";

  // A branch that's gone (deleted, pruned) falls back to the current one.
  useEffect(() => {
    if (selected && !branches.some((b) => b.name === selected)) setSelected(null);
  }, [branches, selected]);

  useEffect(() => {
    if (!root || !status || (!head && !selected)) {
      setCommits([]);
      setMore(false);
      return;
    }
    let cancelled = false;
    gitApi
      .log(root, rev, 0, PAGE)
      .then((list) => {
        if (cancelled) return;
        setError(null);
        setCommits(list);
        setMore(list.length === PAGE);
        setCommit((current) => (current ? (list.find((c) => c.hash === current.hash) ?? null) : null));
      })
      .catch((err) => !cancelled && setError(errorText(err)));
    return () => {
      cancelled = true;
    };
    // `revision` — re-read after every refresh (a new commit, a fetch).
  }, [root, rev, revision, head]);

  function loadMore() {
    if (!root) return;
    gitApi
      .log(root, rev, commits.length, PAGE)
      .then((list) => {
        setCommits((prev) => [...prev, ...list]);
        setMore(list.length === PAGE);
      })
      .catch((err) => setError(errorText(err)));
  }

  if (!root) return <div className="sp-git-empty">{t("Open a folder to use Git.")}</div>;
  if (!loaded) return <div className="sp-git-empty">{t("Loading…")}</div>;
  if (!status) return <div className="sp-git-empty">{t("This folder isn't in a Git repository.")}</div>;

  const current = branches.find((b) => b.current);
  const shownName = selected ?? current?.name ?? status.branch ?? "HEAD";
  return (
    <div className="sp-git-repo">
      <BranchList selected={shownName} onSelect={(name) => setSelected(name === current?.name ? null : name)} />
      <div className="sp-git-history">
        <div className="sp-git-pane-header">
          <Icon name="history" size={13} />
          <span className="sp-git-section-title">{t("History - {branch}", { branch: shownName })}</span>
          <Spacer />
          <IconButton title={t("Refresh")} onClick={() => void refreshGit()}>
            <Icon name="restart" />
          </IconButton>
        </div>
        {error && <div className="sp-git-note">{error}</div>}
        <div className="sp-git-commit-list">
          <div className="sp-git-commit-row sp-git-commit-head">
            <span>{t("Commit")}</span>
            <span>{t("Message")}</span>
            <span>{t("Author")}</span>
            <span>{t("Date")}</span>
          </div>
          {commits.map((c) => (
            <CommitRow key={c.hash} commit={c} selected={commit?.hash === c.hash} onSelect={() => setCommit(c)} />
          ))}
          {commits.length === 0 && !error && <div className="sp-git-note">{t("No commits yet.")}</div>}
          {more && (
            <div className="sp-git-load-more">
              <Button onClick={loadMore}>{t("Load More")}</Button>
            </div>
          )}
        </div>
      </div>
      <CommitDetails commit={commit} />
    </div>
  );
}

// ── Branches ─────────────────────────────────────────────────────────────────

function BranchList({ selected, onSelect }: { selected: string; onSelect: (name: string) => void }) {
  const t = useT();
  const { branches } = useGit();
  const local = branches.filter((b) => !b.remote);
  const remotes = useMemo(() => {
    const groups = new Map<string, BranchInfo[]>();
    for (const b of branches) {
      if (!b.remote || b.name.endsWith("/HEAD")) continue;
      const slash = b.name.indexOf("/");
      const remoteName = slash < 0 ? b.name : b.name.slice(0, slash);
      groups.set(remoteName, [...(groups.get(remoteName) ?? []), b]);
    }
    return [...groups.entries()];
  }, [branches]);
  return (
    <div className="sp-git-branches">
      <div className="sp-git-pane-header">
        <Icon name="branch" size={13} />
        <span className="sp-git-section-title">{t("Branches")}</span>
        <Spacer />
        <IconButton title={t("New Branch...")} onClick={() => void promptNewBranch()}>
          <Icon name="plus" />
        </IconButton>
      </div>
      <div className="sp-git-branch-scroll">
        <div className="sp-git-group">{t("Local")}</div>
        {local.map((b) => (
          <BranchRow key={b.name} branch={b} label={b.name} selected={selected === b.name} onSelect={onSelect} />
        ))}
        {remotes.length > 0 && <div className="sp-git-group">{t("Remotes")}</div>}
        {remotes.map(([remoteName, list]) => (
          <div key={remoteName}>
            <div className="sp-git-group sp-git-group-remote">
              <Icon name="folder" size={12} /> {remoteName}
            </div>
            {list.map((b) => (
              <BranchRow key={b.name} branch={b} label={localNameOf(b.name)} selected={selected === b.name} onSelect={onSelect} nested />
            ))}
          </div>
        ))}
      </div>
    </div>
  );
}

function BranchRow({ branch, label, selected, onSelect, nested }: { branch: BranchInfo; label: string; selected: boolean; onSelect: (name: string) => void; nested?: boolean }) {
  const t = useT();
  const { status, busy } = useGit();
  const menu = useContextMenu<void>();
  const currentName = status?.branch ?? null;
  const items: ContextMenuItem[] = [
    { label: t("Checkout"), disabled: branch.current || !!busy, onSelect: () => checkoutBranch(branch) },
    { label: t("New Local Branch From..."), disabled: !!busy, onSelect: () => void promptNewBranch(branch.name) },
    {
      label: t("Merge '{source}' into '{target}'", { source: branch.name, target: currentName ?? "HEAD" }),
      separatorBefore: true,
      disabled: branch.current || !!busy || !currentName,
      onSelect: () => void mergeIntoCurrent(branch.name),
    },
    {
      label: t("Rebase '{source}' onto '{target}'", { source: currentName ?? "HEAD", target: branch.name }),
      disabled: branch.current || !!busy || !currentName,
      onSelect: () => void rebaseOnto(branch.name),
    },
    {
      label: t("Delete"),
      separatorBefore: true,
      destructive: true,
      disabled: branch.current || branch.remote || !!busy,
      onSelect: () => void deleteBranch(branch.name),
    },
  ];
  const tracking = branch.gone ? t("upstream gone") : branch.ahead || branch.behind ? `↓${branch.behind} ↑${branch.ahead}` : "";
  return (
    <div
      className={`sp-git-row${selected ? " sp-git-selected" : ""}${nested ? " sp-git-nested" : ""}`}
      tabIndex={0}
      title={branch.upstream ? `${branch.name} → ${branch.upstream}` : branch.name}
      onClick={() => onSelect(branch.name)}
      onDoubleClick={() => checkoutBranch(branch)}
      onContextMenu={(e: MouseEvent) => {
        onSelect(branch.name);
        menu.openAtPoint(undefined, e);
      }}
    >
      <Icon name={branch.current ? "check" : "branch"} size={12} />
      <span className={`sp-git-name${branch.current ? " sp-git-current" : ""}`}>{label}</span>
      {tracking && <span className="sp-git-dir">{tracking}</span>}
      <ContextMenu target={menu.target} items={items} onClose={menu.close} />
    </div>
  );
}

// ── History ──────────────────────────────────────────────────────────────────

function CommitRow({ commit, selected, onSelect }: { commit: CommitInfo; selected: boolean; onSelect: () => void }) {
  const t = useT();
  const menu = useContextMenu<void>();
  const items: ContextMenuItem[] = [
    { label: t("Copy Commit ID"), onSelect: () => void navigator.clipboard.writeText(commit.hash) },
    { label: t("Copy Message"), onSelect: () => void navigator.clipboard.writeText(commit.subject) },
    { label: t("New Branch..."), separatorBefore: true, onSelect: () => void promptNewBranch(commit.hash) },
  ];
  return (
    <div
      className={`sp-git-commit-row${selected ? " sp-git-selected" : ""}`}
      tabIndex={0}
      onClick={onSelect}
      onContextMenu={(e: MouseEvent) => {
        onSelect();
        menu.openAtPoint(undefined, e);
      }}
    >
      <span className="sp-git-hash">{commit.shortHash}</span>
      <span className="sp-git-subject">
        {commit.refs.map((ref) => (
          <span key={ref} className="sp-git-ref">
            {ref}
          </span>
        ))}
        {commit.subject}
      </span>
      <span className="sp-git-author" title={commit.email}>
        {commit.author}
      </span>
      <span className="sp-git-date">{formatDate(commit.date)}</span>
      <ContextMenu target={menu.target} items={items} onClose={menu.close} />
    </div>
  );
}

function CommitDetails({ commit }: { commit: CommitInfo | null }) {
  const t = useT();
  const { root } = useGit();
  const [files, setFiles] = useState<FileChange[] | null>(null);

  useEffect(() => {
    setFiles(null);
    if (!root || !commit) return;
    let cancelled = false;
    gitApi
      .commitFiles(root, commit.hash)
      .then((list) => !cancelled && setFiles(list))
      .catch(() => !cancelled && setFiles([]));
    return () => {
      cancelled = true;
    };
  }, [root, commit]);

  if (!commit) return <div className="sp-git-details sp-git-empty">{t("Select a commit to see its changes.")}</div>;
  return (
    <div className="sp-git-details">
      <div className="sp-git-pane-header">
        <span className="sp-git-section-title">{t("Commit {hash}", { hash: commit.shortHash })}</span>
      </div>
      <div className="sp-git-details-meta">
        <div className="sp-git-details-subject">{commit.subject}</div>
        <div>
          {commit.author} &lt;{commit.email}&gt;
        </div>
        <div>{formatDate(commit.date)}</div>
        <div className="sp-git-hash" title={commit.hash}>
          {commit.hash}
        </div>
        {commit.parents.length > 0 && <div className="sp-git-dir">{t("Parents: {hashes}", { hashes: commit.parents.map((p) => p.slice(0, 7)).join(", ") })}</div>}
      </div>
      <div className="sp-git-group">{files ? t("Changes ({count})", { count: files.length }) : t("Loading…")}</div>
      <div className="sp-git-details-files">
        {files?.map((file) => (
          <CommitFileRow key={file.path} file={file} commit={commit} />
        ))}
      </div>
    </div>
  );
}

function CommitFileRow({ file, commit }: { file: FileChange; commit: CommitInfo }) {
  const t = useT();
  const menu = useContextMenu<void>();
  const { name, dir } = splitRepoPath(file.path);
  const showDiff = () => openDiff(commitDiff(file, commit));
  const items: ContextMenuItem[] = [
    { label: t("Compare with Previous"), onSelect: showDiff },
    { label: t("Open"), disabled: file.kind === "deleted", onSelect: () => openFile({ path: file.absPath }) },
  ];
  return (
    <div
      className="sp-git-row"
      tabIndex={0}
      title={`${file.origPath ? `${file.origPath} → ` : ""}${file.path}\n${t(changeKindLabel(file.kind))}`}
      onDoubleClick={showDiff}
      onKeyDown={(e) => e.key === "Enter" && showDiff()}
      onContextMenu={(e: MouseEvent) => menu.openAtPoint(undefined, e)}
    >
      <span className={`sp-git-letter sp-git-kind-${file.kind}`}>{changeLetter(file.kind)}</span>
      <span className={`sp-git-name${file.kind === "deleted" ? " sp-git-deleted" : ""}`}>{name}</span>
      {dir && <span className="sp-git-dir">{dir}</span>}
      <ContextMenu target={menu.target} items={items} onClose={menu.close} />
    </div>
  );
}
