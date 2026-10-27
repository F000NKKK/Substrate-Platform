import { expect, it } from "vitest";
import { offeredGroups, type RunTarget } from "./RunBar";

const target = (group: string, variant: string): RunTarget => ({ name: `${group}-${variant}`, group, variant });
const targets = [
  target("fabric", "1.19.2"),
  target("fabric", "1.20.1"),
  target("forge", "1.19.2"),
  target("forge", "1.20.1"),
  target("neoforge", "1.20.1"),
];

it("offers only the groups that have the current variant", () => {
  const offered = offeredGroups(targets, targets[2]).map((t) => t.group);
  expect(offered).toEqual(["fabric", "forge"]);
});

it("offers every group on a variant they all have, or with none chosen", () => {
  expect(offeredGroups(targets, targets[1]).map((t) => t.group)).toEqual(["fabric", "forge", "neoforge"]);
  expect(offeredGroups(targets, undefined).map((t) => t.group)).toEqual(["fabric", "forge", "neoforge"]);
});
