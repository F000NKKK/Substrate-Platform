import "./ProgressBar.css";

export interface ProgressBarProps {
  /** Done, 0..1; none for a bar that only says something is going on. */
  value?: number | null;
  /** What it's the progress of, for screen readers. */
  label?: string;
  className?: string;
}

/** A thin progress bar in the accent color — a fraction, or indeterminate. */
export function ProgressBar({ value, label, className }: ProgressBarProps) {
  const known = typeof value === "number" && Number.isFinite(value);
  const fraction = known ? Math.min(1, Math.max(0, value)) : 0;
  return (
    <div
      className={["sp-progress", known ? "" : "sp-progress--indeterminate", className].filter(Boolean).join(" ")}
      role="progressbar"
      aria-label={label}
      aria-valuemin={0}
      aria-valuemax={100}
      aria-valuenow={known ? Math.round(fraction * 100) : undefined}
    >
      <div className="sp-progress-fill" style={known ? { width: `${fraction * 100}%` } : undefined} />
    </div>
  );
}
