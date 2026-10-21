// Zoom factors (1 = the original size): the steps Ctrl+wheel walks, the
// fit to a box, and the percent the user reads and types.

/** The factors Ctrl+wheel walks through. */
export const ZOOM_STEPS = [0.1, 0.25, 0.5, 0.75, 1, 1.5, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64] as const;
export const ZOOM_MIN = ZOOM_STEPS[0];
export const ZOOM_MAX = ZOOM_STEPS[ZOOM_STEPS.length - 1];

/** How far a zoom may sit off a step and still count as on it. */
const STEP_TOLERANCE = 1e-3;
const PERCENT = 100;
/** Percents are shown to a tenth. */
const PERCENT_TENTHS = 10;

export const clampZoom = (zoom: number) => Math.min(ZOOM_MAX, Math.max(ZOOM_MIN, zoom));

/** The step after `zoom` inwards (`dir` > 0) or outwards. */
export function stepZoom(zoom: number, dir: number): number {
  if (dir > 0) return ZOOM_STEPS.find((s) => s > zoom * (1 + STEP_TOLERANCE)) ?? ZOOM_MAX;
  return [...ZOOM_STEPS].reverse().find((s) => s < zoom * (1 - STEP_TOLERANCE)) ?? ZOOM_MIN;
}

/** The largest zoom that shows all of `width`×`height` in the box; whole above 1, so pixels stay even. */
export function fitZoom(width: number, height: number, boxWidth: number, boxHeight: number): number {
  const fit = Math.min(boxWidth / Math.max(width, 1), boxHeight / Math.max(height, 1));
  return clampZoom(fit >= 1 ? Math.floor(fit) : fit);
}

/** `zoom` as a percent of the original: `250%`, `12.5%`. */
export const zoomPercent = (zoom: number) => `${Math.round(zoom * PERCENT * PERCENT_TENTHS) / PERCENT_TENTHS}%`;

/** A typed percent (`250`, `250%`, `12,5 %`) as a zoom; none when it isn't one. */
export function parseZoomPercent(text: string): number | null {
  const n = Number(text.replace("%", "").replace(",", ".").trim());
  return Number.isFinite(n) && n > 0 ? clampZoom(n / PERCENT) : null;
}
