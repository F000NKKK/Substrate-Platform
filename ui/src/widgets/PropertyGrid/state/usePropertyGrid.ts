import { propertyEditing } from "./propertyEditing";
import { propertyGridInput } from "./propertyGridInput";
import { propertyScrub } from "./propertyScrub";
import type { PropertyGridProps } from "./propertyTypes";
import { usePropertyGridState } from "./usePropertyGridState";

/** Everything the grid's parts share: its state, editing, and input. */
export function usePropertyGrid(props: PropertyGridProps) {
  const state = usePropertyGridState(props);
  const editing = propertyEditing(state);
  return { ...state, ...editing, ...propertyGridInput(state, editing), ...propertyScrub(state, editing) };
}

export type PropertyGridModel = ReturnType<typeof usePropertyGrid>;
