import type { GridCommits } from "./state/dataGridCommits";
import type { GridEditing } from "./state/dataGridEditing";
import type { GridNavigation } from "./state/dataGridNavigation";
import type { GridProps, GridState } from "./state/useDataGridState";
import type { GridView } from "./state/useDataGridView";

/** The grid's parts, as the later ones (keys) use the earlier. */
export interface GridParts<T> {
  props: GridProps<T>;
  state: GridState<T>;
  view: GridView<T>;
  commits: GridCommits;
  nav: GridNavigation;
  editing: GridEditing<T>;
}
