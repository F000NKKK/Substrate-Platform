import { createIcon } from "../Icon";

/** Locate — a crosshair (Sync with Active Document, reveal). */
export const IconLocate = createIcon(
  <>
    <circle cx="12" cy="12" r="6" />
    <circle cx="12" cy="12" r="1.5" />
    <path d="M12 2v4M12 18v4M2 12h4M18 12h4" />
  </>
);
