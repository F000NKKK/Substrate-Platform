// Source Control: Visual Studio's Git windows, menu and status bar over the
// host plugin's `git_*` commands. A product provides `SourceControlProvider`,
// mounts `GitHost`, and lists the windows and documents.

export { GitChangesWindow, GIT_CHANGES_PANEL } from "./GitChangesWindow";
export { GitHost } from "./GitHost";
export { GitMenu } from "./GitMenu";
export { GitStatusBar } from "./GitStatusBar";
export { GitSettingsPage, gitSettingsKeywords } from "./GitSettings";
export { closeGitDocument, isGitDocument, useGitPanels } from "./gitDocuments";
export { onOpenGitSettings, refreshGit, useGit } from "./gitStore";
export { changeKindLabel, changeLetter } from "./gitApi";
export type { ChangeKind, RepoStatus } from "./gitApi";
export { SourceControlProvider } from "./sourceControlHost";
export type { SourceControlFile, SourceControlHost } from "./sourceControlHost";
