import { describe, expect, it } from "vitest";
import { buildRows } from "./buildRows";
import { defaultExpansion } from "./expansion";
import { parseEdit } from "./editValue";
import { pathId } from "./dataPath";
import type { InspectorRow } from "./inspectorRow";
import { parseShapeDescription } from "./parseShapeDescription";
import type { ShapeDescription } from "./shapeTypes";
import { lightShape, modeShape, modeValue, recursiveShape, motorShape, motorValue, recursiveValue, transformShape, transformValue } from "./substrateFixtures";

function describeShape(json: unknown): ShapeDescription {
  const parsed = parseShapeDescription(json);
  if (!parsed.ok) throw new Error(`fixture does not parse: ${JSON.stringify(parsed.error)}`);
  return parsed.value;
}

const byName = (row: InspectorRow, name: string): InspectorRow => {
  const found = row.children?.find((child) => child.name === name);
  if (!found) throw new Error(`no row ${name}`);
  return found;
};

describe("parseShapeDescription", () => {
  it("reads what substrate-data writes", () => {
    const description = describeShape(transformShape);
    expect([...description.definitions.keys()].sort()).toEqual(["Light", "Mode", "Transform"]);
    expect(description.root).toEqual({ kind: "def", name: "Transform" });
    const transform = description.definitions.get("Transform");
    expect(transform?.kind === "struct" && transform.fields.find((f) => f.name === "scale")?.range).toEqual([0, 10]);
  });

  it("tells a plain enum (no fields) from a tagged one", () => {
    const mode = describeShape(modeShape).definitions.get("Mode");
    const light = describeShape(lightShape).definitions.get("Light");
    expect(mode?.kind === "enum" && [mode.tag, mode.variants[0].fields]).toEqual([null, null]);
    expect(light?.kind === "enum" && [light.tag, light.variants[1].fields?.length]).toEqual(["kind", 2]);
  });

  it("answers an unknown definition with a typed error", () => {
    expect(parseShapeDescription({ root: { kind: "def", name: "Ghost" }, defs: {} })).toEqual({ ok: false, error: { kind: "unknownDefinition", at: "/root", name: "Ghost" } });
  });

  it("finds an unknown definition inside a definition", () => {
    const shape = { root: { kind: "def", name: "A" }, defs: { A: { kind: "struct", fields: [{ name: "b", required: true, shape: { kind: "list", of: { kind: "def", name: "B" } } }] } } };
    expect(parseShapeDescription(shape)).toEqual({ ok: false, error: { kind: "unknownDefinition", at: "/defs/A", name: "B" } });
  });

  it("accepts an empty set of definitions when nothing is referenced", () => {
    expect(parseShapeDescription({ root: { kind: "bool" }, defs: {} }).ok).toBe(true);
  });

  it.each([
    [42, { kind: "malformed", at: "", expected: "an object", found: "number" }],
    [{ root: { kind: "tensor" }, defs: {} }, { kind: "unknownShapeKind", at: "/root", found: "tensor" }],
    [{ root: { kind: "int", min: 0 }, defs: {} }, { kind: "malformed", at: "/root/max", expected: "a number", found: "undefined" }],
    [{ root: { kind: "bool" }, defs: { X: { kind: "union" } } }, { kind: "unknownDefinitionKind", at: "/defs/X/kind", found: "union" }],
    [{ root: { kind: "bool" }, defs: [] }, { kind: "malformed", at: "/defs", expected: "an object", found: "array" }],
    [{ root: { kind: "list" }, defs: {} }, { kind: "malformed", at: "/root/of", expected: "an object", found: "undefined" }],
  ])("rejects %j with a typed error", (json, error) => {
    expect(parseShapeDescription(json)).toEqual({ ok: false, error });
  });
});

describe("the editor backend's transform", () => {
  it("parses and shows the motor, with named and optional wrappers", () => {
    const row = buildRows(describeShape(motorShape), motorValue, { rootName: "transform" });
    const motor = byName(row, "motor");
    expect(motor.typeLabel).toBe("motor");
    expect(motor.doc).toContain("rigid motion");
    expect(motor.children?.map((c) => [c.name, c.typeLabel, c.display.kind])).toEqual([
      ["rotor", "[f32]", "list"],
      ["translation", "[f32]?", "list"],
      ["dual", "[f32]?", "none"],
    ]);
  });
});

describe("buildRows", () => {
  const root = buildRows(describeShape(transformShape), transformValue, { rootName: "Transform" });

  it("lists a struct's fields in the shape's order with their types", () => {
    expect(root.children?.map((c) => [c.name, c.typeLabel])).toEqual([
      ["translation", "[f32]"],
      ["rotation", "[f32]"],
      ["scale", "[f32]"],
      ["mode", "Mode"],
      ["light", "Light"],
      ["lights", "[Light]"],
      ["tags", "{string: i32}"],
      ["parent", "string?"],
      ["visible", "bool"],
      ["count", "u8"],
    ]);
  });

  it("gives every row a stable path id", () => {
    const translation = byName(root, "translation");
    expect(translation.id).toBe(pathId(["translation"]));
    expect(translation.children?.map((c) => c.id)).toEqual(["#/translation/0", "#/translation/1", "#/translation/2"]);
    expect(buildRows(describeShape(transformShape), transformValue).id).toBe("#");
  });

  it("shows list items as editable floats and carries a field's range", () => {
    const scale = byName(root, "scale");
    expect(scale.display).toEqual({ kind: "list", length: 3 });
    expect(scale.children?.[0].editor).toEqual({ kind: "number", range: null });
    expect(byName(root, "count").editor).toEqual({ kind: "integer", min: 0, max: 255 });
  });

  it("shows a plain enum as its variant, editable among all variants", () => {
    const mode = byName(root, "mode");
    expect(mode.display).toEqual({ kind: "variant", name: "fast" });
    expect(mode.editor).toEqual({ kind: "variant", options: ["fast", "slow"], tag: null });
    expect(mode.children).toBeNull();
  });

  it("shows a tagged enum's variant with its payload fields as children, switching only to variants needing nothing", () => {
    const light = byName(root, "light");
    expect(light.display).toEqual({ kind: "variant", name: "point" });
    expect(light.children?.map((c) => c.name)).toEqual(["radius", "color"]);
    expect(light.editor).toEqual({ kind: "variant", options: ["off", "point"], tag: "kind" });
    expect(byName(light, "radius").path).toEqual(["light", "radius"]);
    const lights = byName(root, "lights");
    expect(lights.children?.map((c) => c.display)).toEqual([{ kind: "variant", name: "off" }, { kind: "variant", name: "spot" }]);
    expect(lights.children?.[0].children).toBeNull();
  });

  it("shows a map by key and an absent optional as none", () => {
    expect(byName(byName(root, "tags"), "a").path).toEqual(["tags", "a"]);
    expect(byName(root, "parent").display).toEqual({ kind: "none" });
  });

  it("shows a present optional as its inner value", () => {
    const withParent = buildRows(describeShape(transformShape), { ...(transformValue as object), parent: "root" } as never);
    const parent = byName(withParent, "parent");
    expect(parent.display).toEqual({ kind: "scalar", value: "root" });
    expect(parent.editor).toEqual({ kind: "text" });
  });

  it("builds a plain enum at the root", () => {
    const row = buildRows(describeShape(modeShape), modeValue);
    expect(row.display).toEqual({ kind: "variant", name: "slow" });
  });

  it("follows a recursive definition only as deep as the value", () => {
    const row = buildRows(describeShape(recursiveShape), recursiveValue);
    const next = byName(row, "next");
    expect(next.children?.map((c) => [c.name, c.display])).toEqual([["next", { kind: "none" }], ["v", { kind: "scalar", value: 1 }]]);
  });

  it("reports a value that does not fit as a problem on its row, not an exception", () => {
    const bad = buildRows(describeShape(transformShape), { ...(transformValue as object), count: "seven", mode: "warp", light: 3 } as never);
    expect(byName(bad, "count").problem).toEqual({ kind: "mismatch", expected: "int", found: "string" });
    expect(byName(bad, "mode").problem).toEqual({ kind: "unknownVariant", name: "warp" });
    expect(byName(bad, "light").problem).toEqual({ kind: "mismatch", expected: "map", found: "number" });
  });

  it("marks a required field the value lacks, but not a defaulted one", () => {
    const { count: _count, scale: _scale, ...rest } = transformValue as Record<string, unknown>;
    const row = buildRows(describeShape(transformShape), rest as never);
    expect(byName(row, "count").problem).toEqual({ kind: "missing" });
    expect(byName(row, "scale").display).toEqual({ kind: "default" });
    expect(byName(row, "scale").problem).toBeNull();
  });

  it("reports a missing definition when given a hand-made description", () => {
    const description: ShapeDescription = { root: { kind: "def", name: "Ghost" }, definitions: new Map() };
    expect(buildRows(description, { a: 1 }).problem).toEqual({ kind: "unknownDefinition", name: "Ghost" });
  });

  it("shows the root as missing when there is no value", () => {
    expect(buildRows(describeShape(modeShape), undefined).problem).toEqual({ kind: "missing" });
  });

  it("puts diagnostics on their rows and counts them up the tree", () => {
    const row = buildRows(describeShape(transformShape), transformValue, {
      diagnostics: [{ path: ["translation", 1], message: "NaN" }, { path: ["count"], message: "too big" }],
    });
    expect(byName(byName(row, "translation"), "1").messages).toEqual(["NaN"]);
    expect(byName(row, "translation").issueCount).toBe(1);
    expect(row.issueCount).toBe(2);
    expect(byName(row, "visible").issueCount).toBe(0);
  });

  it("reads an any shape as plain data", () => {
    const row = buildRows({ root: { kind: "any" }, definitions: new Map() }, { a: [1, null], b: "x" });
    expect(row.children?.map((c) => c.name)).toEqual(["a", "b"]);
    expect(row.children?.[0].children?.map((c) => c.display)).toEqual([{ kind: "scalar", value: 1 }, { kind: "scalar", value: null }]);
  });
});

describe("defaultExpansion", () => {
  it("opens the root and its branches, not the leaves of leaves", () => {
    const root = buildRows(describeShape(transformShape), transformValue);
    const open = defaultExpansion(root);
    expect(open.has("#")).toBe(true);
    expect(open.has("#/translation")).toBe(true);
    expect(open.has("#/translation/0")).toBe(false);
    expect(open.has("#/light/color")).toBe(false);
  });
});

describe("parseEdit", () => {
  it("accepts integers within the type and refuses the rest", () => {
    const editor = { kind: "integer", min: 0, max: 255 } as const;
    expect(parseEdit(editor, " 12 ")).toEqual({ ok: true, value: 12 });
    expect(parseEdit(editor, "256")).toEqual({ ok: false, error: { kind: "outOfRange", min: 0, max: 255 } });
    expect(parseEdit(editor, "1.5")).toEqual({ ok: false, error: { kind: "notInteger" } });
    expect(parseEdit(editor, "")).toEqual({ ok: false, error: { kind: "notNumber" } });
  });

  it("checks a float against its range", () => {
    const editor = { kind: "number", range: [0, 10] } as const;
    expect(parseEdit(editor, "2.5")).toEqual({ ok: true, value: 2.5 });
    expect(parseEdit(editor, "11").ok).toBe(false);
    expect(parseEdit(editor, "abc")).toEqual({ ok: false, error: { kind: "notNumber" } });
    expect(parseEdit({ kind: "number", range: null }, "-1e3")).toEqual({ ok: true, value: -1000 });
  });

  it("takes text as it is", () => {
    expect(parseEdit({ kind: "text" }, " a b ")).toEqual({ ok: true, value: " a b " });
  });
});
