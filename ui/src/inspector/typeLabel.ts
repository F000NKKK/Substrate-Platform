import type { ShapeNode } from "./shapeTypes";

const WIDTHS = [8, 16, 32, 64, 128] as const;

/** An integer range as the type it is: `u8`, `i32`, or the bare range. */
function integerLabel(min: number, max: number): string {
  for (const bits of WIDTHS) {
    if (min === 0 && max === 2 ** bits - 1) return `u${bits}`;
    if (min === -(2 ** (bits - 1)) && max === 2 ** (bits - 1) - 1) return `i${bits}`;
  }
  return `int(${min}..${max})`;
}

/** A shape in short, for the type column. */
export function typeLabel(shape: ShapeNode): string {
  switch (shape.kind) {
    case "int":
      return integerLabel(shape.min, shape.max);
    case "float":
      return `f${shape.bits}`;
    case "named":
    case "def":
      return shape.name;
    case "list":
      return `[${typeLabel(shape.of)}]`;
    case "map":
      return `{string: ${typeLabel(shape.of)}}`;
    case "optional":
      return `${typeLabel(shape.of)}?`;
    case "ref":
      return `ref<${shape.of.join("|")}>`;
    default:
      return shape.kind;
  }
}
