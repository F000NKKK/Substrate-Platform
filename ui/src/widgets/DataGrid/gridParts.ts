import type { GridCommits } from "./dataGridCommits";
import type { GridEditing } from "./dataGridEditing";
import type { GridNavigation } from "./dataGridNavigation";
import type { GridProps, GridState } from "./useDataGridState";
import type { GridView } from "./useDataGridView";

/** The grid's parts, as the later ones (keys) use the earlier. */
export interface GridParts<T> {
  props: GridProps<T>;
  state: GridState<T>;
  view: GridView<T>;
  commits: GridCommits;
  nav: GridNavigation;
  editing: GridEditing<T>;
}
