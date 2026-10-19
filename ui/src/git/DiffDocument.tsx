import { useEffect, useState } from "react";
import { openFile } from "../documents/fileOpenBus";
import { errorText } from "../infra/i18n/errorText";
import { useT } from "../infra/i18n/i18n";
import { Icon } from "../infra/icons/IconProvider";
import { DiffEditor, type DiffLayout } from "../widgets/DiffEditor/DiffEditor";
import { IconButton } from "../widgets/IconButton/IconButton";
import { Spacer } from "../widgets/Layout/Layout";
import { gitApi, type DiffTexts } from "./gitApi";
import type { DiffRequest } from "./gitDocuments";
import { useGit } from "./gitStore";
import "./git.css";

/** Remembered across diff tabs, like Visual Studio's diff layout toggle. */
let lastLayout: DiffLayout = "sideBySide";

/**
 * One diff tab: `request.left` against `request.right`, re-read whenever the
 * repository is refreshed (a save, a stage, a commit). Read-only — Open File
 * goes to the editable file.
 */
export function DiffDocument({ request }: { request: DiffRequest }) {
  const t = useT();
  const { root, revision } = useGit();
  const [texts, setTexts] = useState<DiffTexts | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [layout, setLayout] = useState<DiffLayout>(lastLayout);

  useEffect(() => {
    if (!root) return;
    let cancelled = false;
    gitApi
      .diffTexts(root, request.left, request.right)
      .then((next) => {
        if (cancelled) return;
        setError(null);
        // Same texts — keep the editor (and its scroll position) as it is.
        setTexts((prev) => (prev && prev.left === next.left && prev.right === next.right && prev.binary === next.binary ? prev : next));
      })
      .catch((err) => !cancelled && setError(errorText(err)));
    return () => {
      cancelled = true;
    };
  }, [root, revision, request]);

  function toggleLayout() {
    const next: DiffLayout = layout === "sideBySide" ? "inline" : "sideBySide";
    lastLayout = next;
    setLayout(next);
  }

  const path = request.right.path || request.left.path;
  const missing = request.right.rev === "" || (request.right.rev === "WORKTREE" && texts?.right === null);
  let body;
  if (error) body = <div className="sp-git-empty">{error}</div>;
  else if (!texts) body = <div className="sp-git-empty">{t("Loading…")}</div>;
  else if (texts.binary) body = <div className="sp-git-empty">{t("Binary files differ.")}</div>;
  else if ((texts.left ?? "") === (texts.right ?? "")) body = <div className="sp-git-empty">{t("No differences.")}</div>;
  else body = <DiffEditor key={layout} original={texts.left ?? ""} modified={texts.right ?? ""} path={path} layout={layout} />;

  return (
    <div className="sp-git-diff">
      <div className="sp-git-diff-header">
        <span className="sp-git-diff-side" title={request.leftLabel}>
          {request.leftLabel}
        </span>
        <span className="sp-git-diff-arrow">↔</span>
        <span className="sp-git-diff-side" title={request.rightLabel}>
          {request.rightLabel}
        </span>
        <span className="sp-git-diff-path" title={path}>
          {path}
        </span>
        <Spacer />
        <IconButton title={layout === "sideBySide" ? t("Inline Diff") : t("Side-by-Side Diff")} onClick={toggleLayout}>
          <Icon name={layout === "sideBySide" ? "list" : "split"} />
        </IconButton>
        <IconButton title={t("Open File")} disabled={missing} onClick={() => openFile({ path: request.absPath })}>
          <Icon name="file" />
        </IconButton>
      </div>
      <div className="sp-git-diff-body">{body}</div>
    </div>
  );
}
