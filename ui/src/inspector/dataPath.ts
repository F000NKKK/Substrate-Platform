/** One step into a value: a field or map key, or a list index. */
export type PathSegment = string | number;

/** Where a row's value sits in the inspected value; `[]` is the value itself. */
export type DataPath = readonly PathSegment[];

/** A stable id for a path (a JSON pointer behind `#`): the same path is the same id after every rebuild. */
export function pathId(path: DataPath): string {
  return `#${path.map((segment) => `/${String(segment).replaceAll("~", "~0").replaceAll("/", "~1")}`).join("")}`;
}

export function samePath(left: DataPath, right: DataPath): boolean {
  return left.length === right.length && left.every((segment, index) => segment === right[index]);
}
