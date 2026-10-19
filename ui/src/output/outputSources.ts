import { useSyncExternalStore } from "react";
import { events } from "../infra/host/host.generated";

/**
 * One of the Output window's sources, like Visual Studio's "Show output
 * from:" — its own event, buffered from startup so switching to it shows
 * everything it has said so far. `label` and `emptyLabel` are English (the
 * i18n keys).
 */
export interface OutputSourceDef {
  id: string;
  label: string;
  event: string;
  /** What the window says while the source has said nothing. */
  emptyLabel: string;
}

/** The platform's own sources' ids: the IDE's messages, and Source Control. */
export const PLATFORM_OUTPUT = { ide: "ide", git: "git" } as const;

const PLATFORM_SOURCES: OutputSourceDef[] = [
  { id: PLATFORM_OUTPUT.ide, label: "General", event: events.ideOutput, emptyLabel: "No messages." },
  { id: PLATFORM_OUTPUT.git, label: "Source Control - Git", event: events.gitOutput, emptyLabel: "Fetch, pull, push and other Git operations show their output here." },
];

let sources: OutputSourceDef[] = PLATFORM_SOURCES;
// Module-level, like `outputBuffer`: the Output window is only mounted while
// shown, but the source has to follow a build or a launch meanwhile.
let current: string = PLATFORM_SOURCES[0].id;
const subscribers = new Set<() => void>();

/** A product's own sources, listed first (call once, before the window shows); the first is current. */
export function registerOutputSources(defs: OutputSourceDef[]): void {
  sources = [...defs, ...sources.filter((s) => !defs.some((d) => d.id === s.id))];
  current = sources[0].id;
}

/** Every source, in the window's order. */
export function outputSources(): readonly OutputSourceDef[] {
  return sources;
}

/** Shows `id` in the Output window. */
export function setOutputSource(id: string): void {
  if (id === current || !sources.some((s) => s.id === id)) return;
  current = id;
  subscribers.forEach((notify) => notify());
}

export function outputSource(): string {
  return current;
}

/** Calls `listener` whenever the shown source changes (to remember it); unsubscribes. */
export function onOutputSourceChange(listener: () => void): () => void {
  subscribers.add(listener);
  return () => {
    subscribers.delete(listener);
  };
}

/** The source the Output window shows. */
export function useOutputSource(): string {
  return useSyncExternalStore(onOutputSourceChange, () => current);
}
