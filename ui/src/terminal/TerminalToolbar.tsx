import { Icon } from "../infra/icons";
import { useT } from "../infra/i18n";
import { ContextMenu, useContextMenu } from "../widgets/ContextMenu";
import { IconButton } from "../widgets/IconButton";
import type { ShellInfo } from "./terminalApi";

/** The Terminal window's toolbar: New (a menu of shells when there are several), Split, Kill, More (Clear). */
export function TerminalToolbar({ shells, hasActive, onAdd, onSplit, onKill, onClear }: {
  shells: ShellInfo[] | null;
  hasActive: boolean;
  onAdd: (shell: ShellInfo) => void;
  onSplit: () => void;
  onKill: () => void;
  onClear: () => void;
}) {
  const t = useT();
  const addMenu = useContextMenu<void>();
  const moreMenu = useContextMenu<void>();
  const add = () => {
    if (shells && shells.length > 1) addMenu.openAtAnchor();
    else if (shells?.[0]) onAdd(shells[0]);
  };
  return (
    <div className="sp-terminal-toolbar">
      <div className="sp-terminal-toolbar-anchor">
        <IconButton title={t("New Terminal")} disabled={!shells?.length} onClick={add}>
          <Icon name="plus" />
        </IconButton>
        <ContextMenu
          target={addMenu.target ? { mode: "anchor" } : null}
          items={(shells ?? []).map((shell) => ({ label: t(shell.label), onSelect: () => onAdd(shell) }))}
          onClose={addMenu.close}
        />
      </div>
      <IconButton title={t("Split Terminal")} disabled={!hasActive} onClick={onSplit}>
        <Icon name="split" />
      </IconButton>
      <IconButton title={t("Kill Terminal")} disabled={!hasActive} onClick={onKill}>
        <Icon name="trash" />
      </IconButton>
      <div className="sp-terminal-toolbar-anchor">
        <IconButton title={t("More Actions")} onClick={() => moreMenu.openAtAnchor()}>
          <Icon name="more" />
        </IconButton>
        <ContextMenu
          target={moreMenu.target ? { mode: "anchor" } : null}
          items={[{ label: t("Clear Terminal"), shortcut: "Ctrl+L", disabled: !hasActive, onSelect: onClear }]}
          onClose={moreMenu.close}
        />
      </div>
    </div>
  );
}
