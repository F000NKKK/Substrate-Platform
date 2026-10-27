import { IconButton } from "../../widgets/IconButton";
import { Icon } from "../../infra/icons";
import { ContextMenu, belowButton, useContextMenu } from "../../widgets/ContextMenu";
import { useT } from "../../infra/i18n";
import type { ShellLayout } from "../hooks/useShellLayout";

/** How the document tabs lay out: one scrolling row, or wrapped onto as many rows as they need. */
export type TabRows = "single" | "multiple";
export const TAB_ROWS_KEY = "dock.center.tabRows";
export const isTabRows = (v: unknown): v is TabRows => v === "single" || v === "multiple";

/**
 * The strip's own menu (Visual Studio's "Show all tabs"): every open tab,
 * the active one checked — picking one shows it — and the rows setting.
 */
export function TabListMenu({ layout, ids, rows, onRows }: { layout: ShellLayout; ids: string[]; rows: TabRows; onRows: (rows: TabRows) => void }) {
  const t = useT();
  const menu = useContextMenu();
  const items = [
    ...ids.map((id) => ({
      label: layout.panelsById[id].title,
      checked: id === layout.centerActiveId,
      onSelect: () => layout.setCenterActive(id),
    })),
    {
      label: t("Multiple Rows of Tabs"),
      checked: rows === "multiple",
      separatorBefore: ids.length > 0,
      onSelect: () => onRows(rows === "multiple" ? "single" : "multiple"),
    },
  ];
  return (
    <>
      <IconButton className="sp-center-tab-list" title={t("Show All Tabs")} aria-label={t("Show All Tabs")} onClick={(e) => menu.openAtPoint(undefined, belowButton(e))}>
        <Icon name="more" />
      </IconButton>
      {menu.target && <ContextMenu target={menu.target.mode === "viewport" ? { mode: "viewport", x: menu.target.x, y: menu.target.y } : null} items={items} onClose={menu.close} />}
    </>
  );
}
