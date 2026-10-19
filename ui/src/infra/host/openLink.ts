import { substrate } from "./host.generated";
import { invokeCommand } from "./invokeCommand";

// What the host plugin opens in the user's default app — with the system's
// own environment, which an app packaged as an AppImage doesn't pass on.

/** Opens a web or mail link (`open_url`). */
export function openLink(url: string): Promise<void> {
  return invokeCommand<void>(substrate.open.openUrl, { url });
}

/** Opens a file or folder of the workspace (`open_path`). */
export function openPath(path: string): Promise<void> {
  return invokeCommand<void>(substrate.open.openPath, { path });
}

/** Shows a file or folder of the workspace in the file manager (`reveal_path`). */
export function revealPath(path: string): Promise<void> {
  return invokeCommand<void>(substrate.open.revealPath, { path });
}
