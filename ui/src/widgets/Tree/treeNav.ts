import type { TreeNode } from "./Tree";
import type { FlatEntry } from "./useTree";

// The pure parts of Tree's keyboard navigation — what a key press moves to —
// kept apart from React so they can be tested on their own.

/** A node's plain text: `text`, else its label when that's a string. */
export function nodeText<T>(node: TreeNode<T>): string {
  return node.text ?? (typeof node.label === "string" ? node.label : "");
}

/**
 * Type-ahead, as in Windows Explorer and Visual Studio's trees: the next
 * visible row after `from` whose text starts with `prefix` (case-insensitive),
 * wrapping around. A prefix that repeats one character (`"aaa"`) cycles
 * through the rows starting with it. `-1` when none does.
 */
export function typeAheadIndex<T>(flat: FlatEntry<T>[], from: number, prefix: string): number {
  if (!prefix || flat.length === 0) return -1;
  const lower = prefix.toLowerCase();
  const cycling = [...lower].every((c) => c === lower[0]);
  // A fresh multi-letter prefix may still match the current row ("ab" after "a" landed on "abc").
  const start = lower.length > 1 && !cycling ? from : from + 1;
  const wanted = cycling ? lower[0] : lower;
  for (let step = 0; step < flat.length; step++) {
    const i = (((start + step) % flat.length) + flat.length) % flat.length;
    if (nodeText(flat[i].node).toLowerCase().startsWith(wanted)) return i;
  }
  return -1;
}

/** Every branch at or under `node` — what `*` expands. */
export function branchIdsUnder<T>(node: TreeNode<T>, out: string[] = []): string[] {
  if (!node.children) return out;
  out.push(node.id);
  for (const child of node.children) branchIdsUnder(child, out);
  return out;
}

/** The ids of the rows from `a` to `b` (either order), inclusive — a Shift range. */
export function rangeIds<T>(flat: FlatEntry<T>[], a: number, b: number): string[] {
  const [lo, hi] = a <= b ? [a, b] : [b, a];
  return flat.slice(Math.max(0, lo), Math.min(flat.length - 1, hi) + 1).map((e) => e.node.id);
}

/** Where a navigation key moves from row `index` (`null`: it isn't one); `page` rows for PageUp/PageDown. */
export function navigationTarget(key: string, index: number, count: number, page: number): number | null {
  if (count === 0) return null;
  const clamp = (i: number) => Math.max(0, Math.min(count - 1, i));
  switch (key) {
    case "ArrowDown":
      return clamp(index + 1);
    case "ArrowUp":
      return clamp(index - 1);
    case "Home":
      return 0;
    case "End":
      return count - 1;
    case "PageDown":
      return clamp(index + page);
    case "PageUp":
      return clamp(index - page);
    default:
      return null;
  }
}
