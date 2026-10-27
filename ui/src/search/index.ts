// Find and Replace in Files: Visual Studio's Find and Replace window and
// Find Results over the host plugin's `search` commands. A product provides
// `IdeHostProvider`, mounts `FindHost`, and declares the two tool windows.

export { FindHost } from "./FindHost";
export { FindInFilesWindow } from "./FindInFilesWindow";
export { bufferText } from "./findOptions";
export { FindResultsPanel } from "./FindResultsPanel";
export { FIND_IN_FILES_PANEL, FIND_MODE, FIND_RESULTS_PANEL, openFindInFiles, useFindRequest } from "./findUi";
export type { FindRequest, FindWindowMode } from "./findUi";
export type { OpenDocument } from "./findStore";
