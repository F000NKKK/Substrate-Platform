import { useEffect } from "react";
import { useKeybinding } from "../infra/keys/keys";
import { useShell } from "../shell/ShellContext";
import { FIND_IN_FILES_PANEL, FIND_MODE, openFindInFiles, useFindRequest } from "./findUi";

/**
 * Find in Files (Ctrl+Shift+F) and Replace in Files (Ctrl+Shift+H): their
 * keybindings, and bringing up the Find and Replace tool window (floating
 * at first, dockable like any other).
 */
export function FindHost() {
  const request = useFindRequest();
  const shell = useShell();
  useKeybinding("Ctrl+Shift+F", () => openFindInFiles(FIND_MODE.find), { id: "edit.findInFiles" });
  useKeybinding("Ctrl+Shift+H", () => openFindInFiles(FIND_MODE.replace), { id: "edit.replaceInFiles" });

  useEffect(() => {
    if (request) shell.showToolWindow(FIND_IN_FILES_PANEL);
    // Only on a new request.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [request?.seq]);

  return null;
}
