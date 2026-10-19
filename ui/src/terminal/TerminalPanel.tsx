import { useEffect, useRef, useState } from "react";
import { TerminalInstance } from "./TerminalInstance";
import { TerminalSidebar } from "./TerminalSidebar";
import { TerminalToolbar } from "./TerminalToolbar";
import { terminalApi, type ShellInfo } from "./terminalApi";
import { useTerminalTabs } from "./useTerminalTabs";
import "./TerminalPanel.css";

/**
 * The Terminal window, as Visual Studio Code's: a toolbar (new, split,
 * kill, more) and the terminals listed on the right. Each is a shell on a
 * pty through the host plugin; hidden ones keep running (scrollback too),
 * so switching is instant. The terminals and their splits are remembered
 * per state scope (the open project) and reopened as fresh shells.
 */
export function TerminalPanel() {
  const [shells, setShells] = useState<ShellInfo[] | null>(null);
  const { tabs, activeId, setActiveId, addTerminal, splitTerminal, renameTerminal, closeTerminal } = useTerminalTabs(shells);
  const clearHandles = useRef<Record<string, () => void>>({});
  const activeGroupId = tabs.find((tab) => tab.id === activeId)?.groupId;
  // Groups in first-appearance order, each group's members together.
  const groupIds = [...new Set(tabs.map((tab) => tab.groupId))];
  const orderedTabs = groupIds.flatMap((gid) => tabs.filter((tab) => tab.groupId === gid));

  useEffect(() => {
    terminalApi.shells().then(setShells, () => setShells([]));
  }, []);

  return (
    <div className="sp-terminal-manager">
      <TerminalToolbar
        shells={shells}
        hasActive={!!activeId}
        onAdd={(shell) => addTerminal(shell)}
        onSplit={splitTerminal}
        onKill={() => activeId && closeTerminal(activeId)}
        onClear={() => activeId && clearHandles.current[activeId]?.()}
      />
      <div className="sp-terminal-body">
        <div className="sp-terminal-views">
          {groupIds.map((groupId) => (
            <div key={groupId} className="sp-terminal-group" data-active={groupId === activeGroupId || undefined}>
              {tabs
                .filter((tab) => tab.groupId === groupId)
                .map((tab) => {
                  const shell = shells?.find((s) => s.id === tab.shellId);
                  return (
                    <div key={tab.id} className="sp-terminal-pane" data-focused={tab.id === activeId || undefined} onClick={() => setActiveId(tab.id)}>
                      <TerminalInstance
                        id={tab.id}
                        shell={shell?.command}
                        cwd={tab.cwd}
                        clearCommand={shell?.clearCommand ?? ""}
                        active={groupId === activeGroupId}
                        focused={tab.id === activeId}
                        onExit={() => closeTerminal(tab.id)}
                        onReady={(clear) => {
                          if (clear) clearHandles.current[tab.id] = clear;
                          else delete clearHandles.current[tab.id];
                        }}
                      />
                    </div>
                  );
                })}
            </div>
          ))}
        </div>
        <TerminalSidebar tabs={orderedTabs} shells={shells} activeId={activeId} onActivate={setActiveId} onRename={renameTerminal} onClose={closeTerminal} />
      </div>
    </div>
  );
}
