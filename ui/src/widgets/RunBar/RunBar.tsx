import { useEffect } from "react";
import { IconButton } from "../IconButton";
import { Icon } from "../../infra/icons";
import { ContextMenu, useContextMenu } from "../ContextMenu";
import { useStoredState } from "../../infra/state";
import { useT } from "../../infra/i18n";
import "./RunBar.css";

export interface RunTarget {
  name: string;
  /** Human-readable label (e.g. a friendly name from the target's own config), preferred over `name`/`description` for display when present. */
  displayName?: string;
  description?: string;
  /**
   * Targets that are one thing in several variants (a program on several
   * platforms, a loader on several game versions) share a `group`: the bar
   * then offers the group and the variant as two pickers instead of one
   * long list. A group with one target and no `variant` shows no second picker.
   */
  group?: string;
  groupLabel?: string;
  variant?: string;
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
   * An optional extra picker next to the target one — entirely opaque to
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

const label = (t: RunTarget) => t.displayName ?? t.description ?? t.name;
const groupOf = (t: RunTarget) => t.group ?? t.name;

/**
 * A Visual Studio-style run-target picker: choose a target from a dropdown
 * (or a group and its variant, see `RunTarget.group`), then hit the solid
 * green Start arrow to launch it — purely presentational, the product
 * supplies `targets`/`running`/`onRun` from whatever actually resolves and
 * executes them (a workflow engine, a CLI's `run <name>`, anything). Owns
 * its own state, composed into a `Toolbar` rather than merged into it. The
 * chosen target is remembered in the current `StateStore` (`"run.target"`).
 */
export function RunBar({ targets, running, onRun, emptyLabel, modes, selectedMode, onModeChange }: RunBarProps) {
  const [selected, setSelected] = useStoredState<string | null>("run.target", null, (v) => typeof v === "string");
  const t = useT();

  useEffect(() => {
    // No targets (yet — they're often still loading) isn't a reason to
    // forget the remembered choice.
    if (targets.length === 0) return;
    if (selected && targets.some((t) => t.name === selected)) return;
    // Prefer a "run"-named target as the default, the same way VS defaults
    // its Start button to the solution's startup project.
    const preferred = targets.find((t) => t.name.toLowerCase().includes("run")) ?? targets[0];
    setSelected(preferred.name);
  }, [targets, selected, setSelected]);

  const current = targets.find((t) => t.name === selected);
  const grouped = targets.some((t) => t.group);
  const groups = [...new Map(targets.map((t) => [groupOf(t), t])).values()];
  const variants = current ? targets.filter((t) => groupOf(t) === groupOf(current) && t.variant) : [];

  /** Switching group keeps the variant when the new group has it, else takes its last (newest). */
  function pickGroup(group: string) {
    const members = targets.filter((t) => groupOf(t) === group);
    const same = members.find((m) => m.variant && m.variant === current?.variant);
    setSelected((same ?? members[members.length - 1]).name);
  }

  const selectedModeEntry = modes?.find((m) => m.name === selectedMode);
  const nothing = emptyLabel ?? t("No targets");

  return (
    <div className="sp-run-bar">
      <IconButton
        title={current ? t("Run: {target}", { target: label(current) }) : t("Select a target first")}
        disabled={!current || running}
        className="sp-run-bar-play"
        onClick={() => current && onRun(current.name)}
      >
        <Icon name="play" />
      </IconButton>
      {grouped ? (
        <>
          <Picker
            text={current ? (current.groupLabel ?? groupOf(current)) : nothing}
            disabled={targets.length === 0}
            items={groups.map((g) => ({ label: g.groupLabel ?? groupOf(g), checked: current !== undefined && groupOf(g) === groupOf(current), onSelect: () => pickGroup(groupOf(g)) }))}
          />
          {variants.length > 0 && (
            <Picker
              text={current?.variant ?? ""}
              items={variants.map((v) => ({ label: v.variant ?? label(v), checked: v.name === selected, onSelect: () => setSelected(v.name) }))}
            />
          )}
        </>
      ) : (
        <Picker
          text={current ? label(current) : nothing}
          disabled={targets.length === 0}
          items={targets.map((tg) => ({ label: label(tg), checked: tg.name === selected, onSelect: () => setSelected(tg.name) }))}
        />
      )}
      {modes && modes.length > 0 && (
        <Picker
          text={selectedModeEntry?.label ?? selectedModeEntry?.name ?? modes[0]?.label ?? modes[0]?.name ?? ""}
          items={modes.map((m) => ({ label: m.label ?? m.name, checked: m.name === selectedMode, onSelect: () => onModeChange?.(m.name) }))}
        />
      )}
    </div>
  );
}

interface PickerItem {
  label: string;
  checked: boolean;
  onSelect: () => void;
}

/** One dropdown of the bar: its current choice, a menu of the others. */
function Picker({ text, items, disabled }: { text: string; items: PickerItem[]; disabled?: boolean }) {
  const menu = useContextMenu<void>();
  return (
    <div className="sp-run-bar-target-anchor">
      <button type="button" className="sp-run-bar-target" disabled={disabled} onClick={() => menu.openAtAnchor()}>
        <span>{text}</span>
        <Icon name="chevronRight" size={11} className="sp-run-bar-chevron" />
      </button>
      <ContextMenu target={menu.target ? { mode: "anchor" } : null} items={items} onClose={menu.close} />
    </div>
  );
}
