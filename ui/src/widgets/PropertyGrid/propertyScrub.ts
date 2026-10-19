import type { PointerEvent as ReactPointerEvent } from "react";
import { trackPointer } from "../../infra/hooks/trackPointer";
import type { PropertyDescriptor, PropertyValue } from "./propertyTypes";
import { isEditable, roundNumber, SCRUB_FINE, SCRUB_PIXELS, shortNumber } from "./propertyValues";
import type { PropertyEditing } from "./propertyEditing";
import type { PropertyGridState } from "./usePropertyGridState";

/** Scrubbing numbers by their labels. */
export function propertyScrub({ rowsRef, busyId }: PropertyGridState, editing: PropertyEditing) {
  /**
   * Drags a number (or one component of a vector) by its label, as Blender
   * does: a step per few pixels, Shift for a tenth of it; committed once,
   * on release.
   */
  function startScrub(e: ReactPointerEvent<HTMLElement>, p: PropertyDescriptor, component: number | null) {
    if (e.button !== 0 || !isEditable(p) || busyId === p.id) return;
    const base = component === null ? (typeof p.value === "number" ? p.value : null) : Array.isArray(p.value) ? p.value[component] : null;
    if (base === null || base === undefined) return;
    e.preventDefault();
    const startX = e.clientX;
    const step = p.step ?? 1;
    let latest = base;
    const valueAt = (n: number): PropertyValue => {
      if (component === null) return n;
      const v = [...(p.value as readonly number[])];
      v[component] = n;
      return v;
    };
    trackPointer(
      e,
      (ev) => {
        const steps = Math.round((ev.clientX - startX) / SCRUB_PIXELS);
        latest = roundNumber(base + steps * step * (ev.shiftKey ? SCRUB_FINE : 1));
        const out = rowsRef.current?.querySelector<HTMLElement>(`[data-property-id="${CSS.escape(p.id)}"] [data-scrub="${component ?? "value"}"]`);
        if (out instanceof HTMLInputElement) out.value = shortNumber(latest);
        else if (out) out.textContent = shortNumber(latest);
      },
      () => {
        if (latest !== base) void editing.commitChoice(p, valueAt(latest));
      },
    );
  }

  return { startScrub };
}
