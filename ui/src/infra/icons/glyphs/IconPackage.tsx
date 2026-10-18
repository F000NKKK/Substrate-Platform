import { createIcon } from "../Icon";

/** A package — a box, its lid's seam down the front (an installable thing). */
export const IconPackage = createIcon(
  <>
    <path d="M12 3 20 7.5v9L12 21l-8-4.5v-9Z" />
    <polyline points="4,7.5 12,12 20,7.5" />
    <line x1="12" y1="12" x2="12" y2="21" />
  </>
);
