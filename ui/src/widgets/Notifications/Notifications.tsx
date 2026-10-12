import { useEffect, useSyncExternalStore } from "react";
import { IconButton } from "../IconButton";
import { Icon } from "../../infra/icons";
import { ViewportPortal } from "../../infra/portal";
import { useT } from "../../infra/i18n";
import "./Notifications.css";

export type NotificationLevel = "info" | "warn" | "error";

export interface NotificationOptions {
  level?: NotificationLevel;
  message: string;
  /** Milliseconds before it dismisses itself — default 4s for info, 8s for warn/error; `0` keeps it until closed. */
  timeout?: number;
}

interface Notification {
  id: number;
  level: NotificationLevel;
  message: string;
  timeout: number;
}

/** Oldest toasts beyond this are dropped, so a burst of failures can't bury the window. */
const MAX_VISIBLE = 5;

// Module-level, like `showDialog`'s queue: anything can raise a notification
// (an async callback, a context provider), and `NotificationHost` renders them.
let notifications: Notification[] = [];
let nextId = 0;
const subscribers = new Set<() => void>();

function setNotifications(next: Notification[]) {
  notifications = next;
  subscribers.forEach((notify) => notify());
}

/** Shows a transient toast in the window's bottom-right corner — for failures (and the occasional notice) the user should see without having the Output panel open. Requires a mounted `NotificationHost` (`PlatformShell` mounts one). */
export function notify({ level = "info", message, timeout }: NotificationOptions): void {
  const entry = { id: nextId++, level, message, timeout: timeout ?? (level === "info" ? 4000 : 8000) };
  setNotifications([...notifications, entry].slice(-MAX_VISIBLE));
}

function dismiss(id: number) {
  setNotifications(notifications.filter((n) => n.id !== id));
}

function subscribe(onChange: () => void) {
  subscribers.add(onChange);
  return () => {
    subscribers.delete(onChange);
  };
}

/** Renders `notify`'s toasts — mount exactly one per app (`PlatformShell` already does). */
export function NotificationHost() {
  const list = useSyncExternalStore(subscribe, () => notifications);
  if (list.length === 0) return null;
  return (
    <ViewportPortal>
      <div className="sp-notifications">
        {list.map((n) => (
          <Toast key={n.id} notification={n} />
        ))}
      </div>
    </ViewportPortal>
  );
}

function Toast({ notification }: { notification: Notification }) {
  const t = useT();
  const { id, level, message, timeout } = notification;
  useEffect(() => {
    if (timeout <= 0) return;
    const timer = window.setTimeout(() => dismiss(id), timeout);
    return () => window.clearTimeout(timer);
  }, [id, timeout]);

  return (
    <div className={`sp-notification sp-notification--${level}`} role={level === "error" ? "alert" : "status"}>
      <span className="sp-notification-message">{message}</span>
      <IconButton size={20} aria-label={t("Dismiss")} onClick={() => dismiss(id)}>
        <Icon name="close" size={12} />
      </IconButton>
    </div>
  );
}
