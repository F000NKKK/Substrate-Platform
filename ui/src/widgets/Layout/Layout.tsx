import "./Layout.css";

/** Takes the leftover width (or height) of a flex row, pushing what follows to its far end — in a toolbar, a header, a footer. */
export function Spacer() {
  return <span className="sp-spacer" aria-hidden />;
}

/** A short vertical line between groups of controls in a row. */
export function Divider() {
  return <span className="sp-divider" role="separator" aria-orientation="vertical" />;
}
