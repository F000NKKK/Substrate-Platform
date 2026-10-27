import { Toolbar, type ToolbarSection } from "../widgets/Toolbar";
import { useT } from "../infra/i18n";
import type { DapSession } from "./dapSession";
import { useDebugState } from "./useDebugState";

export interface DebugToolbarProps {
  session: DapSession;
  /** Starts the program: the product's launch configuration (the adapter's own arguments). */
  launch: () => Record<string, unknown>;
}

/** The debug toolbar: start or stop, and — by what the program is doing — pause or continue and the three steps. */
export function DebugToolbar({ session, launch }: DebugToolbarProps) {
  const t = useT();
  const { status } = useDebugState(session);
  const idle = status === "idle" || status === "ended";
  const paused = status === "stopped";
  const sections: ToolbarSection[] = [
    [
      idle
        ? { icon: "play", label: t("Start Debugging"), onClick: () => void session.start(launch()) }
        : { icon: "stop", label: t("Stop Debugging"), onClick: () => void session.stop() },
    ],
    [
      paused
        ? { icon: "play", label: t("Continue"), onClick: () => void session.resume() }
        : { icon: "pause", label: t("Pause"), onClick: () => void session.pauseExecution(), disabled: status !== "running" },
      { icon: "arrowRight", label: t("Step Over"), onClick: () => void session.stepOver(), disabled: !paused },
      { icon: "arrowDown", label: t("Step Into"), onClick: () => void session.stepIn(), disabled: !paused },
      { icon: "arrowUp", label: t("Step Out"), onClick: () => void session.stepOut(), disabled: !paused },
    ],
  ];
  return <Toolbar className="sp-debug-toolbar" sections={sections} />;
}
