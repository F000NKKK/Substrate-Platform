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

export const IconFolderOpen = createIcon(
  <>
    <path d="M3 6.5a1.5 1.5 0 0 1 1.5-1.5h4l2 2h9A1.5 1.5 0 0 1 21 8.5V9H7.5a1.5 1.5 0 0 0-1.44 1.09L3 19.5z" />
    <path d="M3 6.5v11A1.5 1.5 0 0 0 4.5 19h15l2.94-8.41a1 1 0 0 0-.94-1.34H7.5a1.5 1.5 0 0 0-1.44 1.09L3 19.5" />
  </>
);

export const IconFile = createIcon(
  <>
    <path d="M6.5 3.5h7l4 4v12a1.5 1.5 0 0 1-1.5 1.5h-9.5A1.5 1.5 0 0 1 5 19.5v-14a1.5 1.5 0 0 1 1.5-1.5Z" />
    <polyline points="13.5,3.5 13.5,7.5 17.5,7.5" />
  </>
);

export const IconChevronRight = createIcon(<polyline points="9,5 16,12 9,19" />);

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

export const IconFloat = createIcon(
  <>
    <rect x="4" y="4" width="10" height="8" rx="1.5" />
    <path d="M14 15h4a2 2 0 0 0 2-2v-4" />
    <polyline points="16,4 20,4 20,8" />
    <line x1="20" y1="4" x2="13" y2="11" />
  </>
);

export const IconSettings = createIcon(
  <>
    <circle cx="12" cy="12" r="3" />
    <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.6a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
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
