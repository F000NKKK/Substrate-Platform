import { formatShortcut, useT } from "../../infra/i18n";
import "./KeyHints.css";

export interface KeyHint {
  /** What it does — translated here (`"Orbit the view"`). */
  action: string;
  /** Its keys as shortcuts are written (`"mmb"`, `"shift+mmb"`, `"num1"`), shown in the locale; several are alternatives. */
  keys: string | readonly string[];
}

export interface KeyHintsProps {
  hints: readonly KeyHint[];
  /** A heading over the list — default "Controls". */
  title?: string;
  className?: string;
}

/**
 * How a window is driven, as a list of actions with their keys drawn as
 * key caps — readable at a glance, unlike a sentence of shortcuts; the
 * key names come from `formatShortcut`, so they're localized in one place.
 */
export function KeyHints({ hints, title, className }: KeyHintsProps) {
  const t = useT();
  return (
    <div className={["sp-key-hints", className].filter(Boolean).join(" ")}>
      <div className="sp-key-hints-title">{title ?? t("Controls")}</div>
      <dl className="sp-key-hints-list">
        {hints.map((h) => (
          <div key={h.action} className="sp-key-hints-row">
            <dt>{t(h.action)}</dt>
            <dd>
              {(typeof h.keys === "string" ? [h.keys] : h.keys).map((k, i) => (
                <span key={k} className="sp-key-hints-alt">
                  {i > 0 && <span className="sp-key-hints-or">/</span>}
                  {formatShortcut(k)
                    .split("+")
                    .map((cap, j) => (
                      <kbd key={j}>{cap}</kbd>
                    ))}
                </span>
              ))}
            </dd>
          </div>
        ))}
      </dl>
    </div>
  );
}
