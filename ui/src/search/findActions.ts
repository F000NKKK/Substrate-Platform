import { invokeCommand } from "../infra/host/invokeCommand";
import { substrate } from "../infra/host/host.generated";
import type { IdeHost } from "../infra/host/ideHost";
import { t, tn } from "../infra/i18n/i18n";
import { reportError } from "../output/reportError";
import { confirmDialog } from "../widgets/Dialog/Dialog";
import { editorSetText } from "../widgets/Editor/code/editorCommands";
import { getEditorView } from "../widgets/Editor/registry/editorRegistry";
import { notify } from "../widgets/Notifications/Notifications";
import { lookInLabel } from "./findOptions";
import { LOOK_IN, runFind, scopeOf, showReplaceResults, type ReplaceSummary } from "./findStore";
import { FIND_RESULTS_PANEL } from "./findUi";
import type { FindForm } from "./useFindForm";

/** Find All: every hit into Find Results. */
export function findAll(form: FindForm): void {
  if (!form.canRun) return;
  form.shell.showToolWindow(FIND_RESULTS_PANEL);
  void runFind({ caption: form.caption(t("Find all")), root: form.root, lookIn: form.lookIn, query: form.query(), documents: form.documents() });
}

/**
 * Replace All, after a confirmation: open documents in their editors
 * (undoable, left unsaved), every other file on disk.
 */
export async function replaceAll(form: FindForm): Promise<void> {
  if (!form.canRun || !(await confirmReplace(form))) return;
  form.setBusy(true);
  try {
    const summary = await invokeCommand<ReplaceSummary>(substrate.search.replaceInFiles, {
      request: { root: form.root, scope: scopeOf(form.lookIn), query: form.query(), replacement: form.replacement, documents: form.documents() },
    });
    applyToDocuments(form.host, summary);
    if (summary.files.length > 0) await form.host.reloadFromDisk();
    summary.errors.forEach((err) => reportError(t("Replace in Files couldn't write {error}", { error: err })));
    form.shell.showToolWindow(FIND_RESULTS_PANEL);
    showReplaceResults(form.caption(t("Replace all")) + ` → "${form.replacement}"`, form.root, summary);
    notifyReplaced(summary);
  } catch (err) {
    reportError(t("Replace in Files failed: {error}", { error: String(err) }));
  } finally {
    form.setBusy(false);
  }
}

function confirmReplace({ pattern, replacement, lookIn }: FindForm): Promise<boolean> {
  const where = lookInLabel(lookIn);
  return confirmDialog({
    title: t("Replace All"),
    message:
      t('Replace all occurrences of "{find}" with "{replace}" in {where}?', { find: pattern, replace: replacement, where }) +
      (lookIn === LOOK_IN.project ? "\n\n" + t("Files that aren't open are changed on disk; that can't be undone with Undo.") : ""),
    confirmLabel: t("Replace All"),
  });
}

/** The replaced documents' text into their editors, or into the host's documents where no editor is up. */
function applyToDocuments(host: IdeHost, summary: ReplaceSummary): void {
  for (const doc of summary.documents) {
    const view = getEditorView(doc.path);
    if (view) editorSetText(view, doc.text);
    else {
      const tab = host.documents.find((tab) => tab.path === doc.path);
      if (tab) host.setText(tab.id, doc.text);
    }
  }
}

function notifyReplaced(summary: ReplaceSummary): void {
  const files = summary.documents.length + summary.files.length;
  notify({
    message:
      summary.total === 0
        ? t("No occurrences were found.")
        : tn(summary.total, "{count} occurrence replaced", "{count} occurrences replaced") + ", " + tn(files, "{count} file", "{count} files"),
  });
}
