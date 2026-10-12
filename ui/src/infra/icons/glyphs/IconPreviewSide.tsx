import { createIcon } from "../Icon";

/** A split pane with a magnifier on the right half — "Open Preview to the Side". */
export const IconPreviewSide = createIcon(
  <>
    <path d="M12 20H4a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v7" />
    <line x1="12" y1="4" x2="12" y2="20" />
    <circle cx="17" cy="17" r="2.5" />
    <line x1="18.8" y1="18.8" x2="21" y2="21" />
  </>
);
