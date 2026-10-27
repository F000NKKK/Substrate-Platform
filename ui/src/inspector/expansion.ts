import type { InspectorRow } from "./inspectorRow";

/** Ids of the branches open at first: the root and what is directly under it. */
export function defaultExpansion(root: InspectorRow, depth = 2): ReadonlySet<string> {
  const open = new Set<string>();
  const visit = (row: InspectorRow, remaining: number): void => {
    if (!row.children || remaining <= 0) return;
    open.add(row.id);
    row.children.forEach((child) => visit(child, remaining - 1));
  };
  visit(root, depth);
  return open;
}

/** Ids of every branch with a diagnostic somewhere under it, so a problem is never hidden. */
export function withIssuesOpen(root: InspectorRow, open: ReadonlySet<string>): ReadonlySet<string> {
  const next = new Set(open);
  const visit = (row: InspectorRow): void => {
    if (!row.children || row.issueCount === 0) return;
    next.add(row.id);
    row.children.forEach(visit);
  };
  visit(root);
  return next;
}
