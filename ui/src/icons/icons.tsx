import { createIcon } from "./Icon";

/**
 * The platform's default icon set — plain geometric glyphs, not tied to any
 * one product's domain. Products either use these directly or drop in
 * their own ReactNode for `PanelDef.icon`.
 */

export const IconWindow = createIcon(
  <>
    <rect x="3" y="4" width="18" height="16" rx="2" />
    <line x1="3" y1="9" x2="21" y2="9" />
  </>
);

export const IconGrid = createIcon(
  <>
    <rect x="3" y="3" width="7" height="7" rx="1.5" />
    <rect x="14" y="3" width="7" height="7" rx="1.5" />
    <rect x="3" y="14" width="7" height="7" rx="1.5" />
    <rect x="14" y="14" width="7" height="7" rx="1.5" />
  </>
);

export const IconFolder = createIcon(
  <path d="M3 6.5a1.5 1.5 0 0 1 1.5-1.5h4l2 2h9A1.5 1.5 0 0 1 21 8.5v9A1.5 1.5 0 0 1 19.5 19h-15A1.5 1.5 0 0 1 3 17.5z" />
);

export const IconSliders = createIcon(
  <>
    <line x1="5" y1="4" x2="5" y2="20" />
    <circle cx="5" cy="9" r="2" />
    <line x1="12" y1="4" x2="12" y2="20" />
    <circle cx="12" cy="15" r="2" />
    <line x1="19" y1="4" x2="19" y2="20" />
    <circle cx="19" cy="7" r="2" />
  </>
);

export const IconList = createIcon(
  <>
    <line x1="8" y1="6" x2="21" y2="6" />
    <line x1="8" y1="12" x2="21" y2="12" />
    <line x1="8" y1="18" x2="21" y2="18" />
    <line x1="3.5" y1="6" x2="3.5" y2="6.01" />
    <line x1="3.5" y1="12" x2="3.5" y2="12.01" />
    <line x1="3.5" y1="18" x2="3.5" y2="18.01" />
  </>
);

export const IconTerminal = createIcon(
  <>
    <rect x="3" y="4" width="18" height="16" rx="2" />
    <polyline points="7,9.5 10.5,12 7,14.5" />
    <line x1="12.5" y1="14.5" x2="16.5" y2="14.5" />
  </>
);

export const IconPin = createIcon(
  <>
    <path d="M9 4h6l-1 6 3 3v2h-5.5V21l-.5 2-.5-2v-6H5v-2l3-3z" />
  </>
);

export const IconPinOff = createIcon(
  <>
    <path d="M9 4h6l-1 6 3 3v2h-5.5V21l-.5 2-.5-2v-6H5v-2l3-3z" opacity="0.45" />
    <line x1="4" y1="4" x2="20" y2="20" />
  </>
);

export const IconClose = createIcon(
  <>
    <line x1="6" y1="6" x2="18" y2="18" />
    <line x1="18" y1="6" x2="6" y2="18" />
  </>
);

export const IconPalette = createIcon(
  <>
    <path d="M12 3a9 9 0 1 0 0 18c1.1 0 1.5-.7 1.5-1.4 0-.4-.2-.7-.4-1a1.7 1.7 0 0 1-.4-1c0-.8.7-1.4 1.5-1.4H16a4 4 0 0 0 4-4c0-4.4-3.6-9.2-8-9.2Z" />
    <circle cx="7.5" cy="10.5" r="1.1" fill="currentColor" stroke="none" />
    <circle cx="12" cy="7.5" r="1.1" fill="currentColor" stroke="none" />
    <circle cx="16" cy="10" r="1.1" fill="currentColor" stroke="none" />
  </>
);
