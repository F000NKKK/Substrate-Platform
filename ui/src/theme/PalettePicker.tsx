import { useState } from "react";
import { useTheme } from "./useTheme";
import { AccentColorField } from "./AccentColorField";
import "./PalettePicker.css";

/** A small standalone accent-color popover — most products should surface this inside their Settings window instead (see AccentColorField), but it's kept for quick standalone use. */
export function PalettePicker() {
  const [open, setOpen] = useState(false);

  return (
    <div className="sp-palette-root">
      <button className="sp-palette-trigger" aria-label="Accent color" onClick={() => setOpen((v) => !v)} />
      {open && (
        <div className="sp-palette-popover" role="dialog">
          <AccentColorField />
        </div>
      )}
    </div>
  );
}
