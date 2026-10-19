import { ContextMenu, useContextMenu } from "../widgets/ContextMenu";
import { OutputPanel, clearOutputBuffer } from "../widgets/OutputPanel";
import { Spacer } from "../widgets/Layout";
import { Toolbar, ToolbarButton, ToolbarLabel } from "../widgets/Toolbar";
import { useT } from "../infra/i18n";
import { outputSources, setOutputSource, useOutputSource } from "./outputSources";
import "./OutputWindow.css";

/** The Output tool window: "Show output from:" a source, its lines, Clear. */
export function OutputWindow() {
  const t = useT();
  const source = useOutputSource();
  const menu = useContextMenu<void>();
  const all = outputSources();
  const current = all.find((s) => s.id === source) ?? all[0];
  return (
    <div className="sp-output-window">
      <Toolbar>
        <ToolbarLabel>{t("Show output from:")}</ToolbarLabel>
        <div className="sp-output-window-source">
          <ToolbarButton label={t(current.label)} dropdown onClick={() => menu.openAtAnchor()} />
          <ContextMenu
            target={menu.target ? { mode: "anchor" } : null}
            items={all.map((s) => ({ label: t(s.label), checked: s.id === source, onSelect: () => setOutputSource(s.id) }))}
            onClose={menu.close}
          />
        </div>
        <Spacer />
        <ToolbarButton icon="trash" label={t("Clear {source} Output", { source: t(current.label) })} onClick={() => clearOutputBuffer(current.event)} />
      </Toolbar>
      <div className="sp-output-window-body">
        <OutputPanel key={current.event} eventName={current.event} emptyLabel={t(current.emptyLabel)} />
      </div>
    </div>
  );
}
