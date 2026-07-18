import { useEffect, useState } from "react";
import { IconButton } from "../IconButton";
import { Icon } from "../../infra/icons";
import { ContextMenu, useContextMenu } from "../ContextMenu";
import "./RunBar.css";

export interface RunTarget {
  name: string;
  /** Human-readable label (e.g. a mod's `name = "..."` inside `[run.X]`), preferred over `name`/`description` for display when present. */
  displayName?: string;
  description?: string;
}

/** One entry in the optional mode picker — see `RunBarProps.modes`. */
export interface RunMode {
  name: string;
  label?: string;
}

export interface RunBarProps {
  targets: RunTarget[];
  running: boolean;
  onRun: (name: string) => void;
  emptyLabel?: string;
  /**
   * An optional second picker next to the target one — entirely opaque to
   * `RunBar`: it has no idea what a mode *means* (debug vs. release, a
   * build configuration, anything else a host product invents), it just
   * renders named options and reports which one is selected. Omit
   * entirely for a plain target-only run bar.
   */
  modes?: RunMode[];
  /** Controlled: which `modes[].name` is currently selected. */
  selectedMode?: string;
  onModeChange?: (name: string) => void;
}

/**
 * A Visual Studio-style run-target picker: choose a target from a dropdown,
 * then hit the solid green Start arrow to launch it — purely presentational,
 * the product supplies `targets`/`running`/`onRun` from whatever actually
 * resolves and executes them (a workflow engine, `yog run <name>`, anything).
 * Owns its own state/behavior entirely separately from `Toolbar`; meant to
 * be composed into one via its `leading` slot rather than merged into it,
 * mirroring VS's startup-item selector + Start button sitting in the same
 * toolbar row as its other actions.
 */
export function RunBar({ targets, running, onRun, emptyLabel = "No targets", modes, selectedMode, onModeChange }: RunBarProps) {
  const [selected, setSelected] = useState<string | null>(null);
  const menu = useContextMenu<void>();
  const modeMenu = useContextMenu<void>();

  useEffect(() => {
    if (targets.length === 0) {
      setSelected(null);
      return;
    }
    if (selected && targets.some((t) => t.name === selected)) return;
    // Prefer a "run"-named target as the default, the same way VS defaults
    // its Start button to the solution's startup project.
    const preferred = targets.find((t) => t.name.toLowerCase().includes("run")) ?? targets[0];
    setSelected(preferred.name);
  }, [targets, selected]);

  const selectedTarget = targets.find((t) => t.name === selected);
  const selectedModeEntry = modes?.find((m) => m.name === selectedMode);

  return (
    <div className="sp-run-bar">
      <IconButton
        size={26}
        title={selectedTarget ? `Run: ${selectedTarget.name}` : "Select a target first"}
        disabled={!selected || running}
        className="sp-run-bar-play"
        onClick={() => selected && onRun(selected)}
      >
        <Icon name="play" size={15} />
      </IconButton>
      <div className="sp-run-bar-target-anchor">
        <button type="button" className="sp-run-bar-target" disabled={targets.length === 0} onClick={() => menu.openAtAnchor()}>
          <span>{selectedTarget?.displayName ?? selectedTarget?.description ?? selectedTarget?.name ?? emptyLabel}</span>
          <Icon name="chevronRight" size={11} className="sp-run-bar-chevron" />
        </button>
        <ContextMenu
          target={menu.target ? { mode: "anchor" } : null}
          items={targets.map((t) => ({
            label: t.displayName ?? t.description ?? t.name,
            checked: t.name === selected,
            onSelect: () => setSelected(t.name),
          }))}
          onClose={menu.close}
        />
      </div>
      {modes && modes.length > 0 && (
        <div className="sp-run-bar-target-anchor">
          <button type="button" className="sp-run-bar-target" onClick={() => modeMenu.openAtAnchor()}>
            <span>{selectedModeEntry?.label ?? selectedModeEntry?.name ?? modes[0]?.label ?? modes[0]?.name}</span>
            <Icon name="chevronRight" size={11} className="sp-run-bar-chevron" />
          </button>
          <ContextMenu
            target={modeMenu.target ? { mode: "anchor" } : null}
            items={modes.map((m) => ({
              label: m.label ?? m.name,
              checked: m.name === selectedMode,
              onSelect: () => onModeChange?.(m.name),
            }))}
            onClose={modeMenu.close}
          />
        </div>
      )}
    </div>
  );
}
