// Source Control: Visual Studio's Git windows, menu and status bar over the
// host plugin's `git_*` commands. A product provides `IdeHostProvider`,
// mounts `GitHost`, and lists the windows and documents.

export { GitChangesWindow, GIT_CHANGES_PANEL } from "./windows/GitChangesWindow";
export { GitHost } from "./GitHost";
export { GitMenu } from "./windows/GitMenu";
export { GitStatusBar } from "./windows/GitStatusBar";
export { GitSettingsPage, gitSettingsKeywords } from "./windows/GitSettings";
export { closeGitDocument, isGitDocument, useGitPanels } from "./store/gitDocuments";
export { onOpenGitSettings, refreshGit, useGit } from "./store/gitStore";
export { changeKindLabel, changeLetter } from "./store/gitApi";
export type { ChangeKind, RepoStatus } from "./store/gitApi";
