import { describe, expect, it } from "vitest";
import { scrollMoves } from "./useContextMenu";

function originIn(parent: HTMLElement) {
  const row = document.createElement("div");
  parent.appendChild(row);
  const rect = row.getBoundingClientRect();
  return { node: row, top: rect.top, left: rect.left };
}

describe("context menu scroll dismissal", () => {
  it("ignores a scroll in an unrelated panel (Output following a build)", () => {
    const tree = document.body.appendChild(document.createElement("div"));
    const output = document.body.appendChild(document.createElement("div"));
    expect(scrollMoves(output, originIn(tree))).toBe(false);
  });

  it("ignores a scroll that leaves the origin where it was", () => {
    const tree = document.body.appendChild(document.createElement("div"));
    expect(scrollMoves(tree, originIn(tree))).toBe(false);
  });

  it("closes when a scroll moved the origin", () => {
    const tree = document.body.appendChild(document.createElement("div"));
    const origin = originIn(tree);
    expect(scrollMoves(tree, { ...origin, top: origin.top + 20 })).toBe(true);
    expect(scrollMoves(document, { ...origin, left: origin.left - 5 })).toBe(true);
    expect(scrollMoves(document, null)).toBe(true);
  });

  it("keeps the menu when a refresh replaced the origin", () => {
    const tree = document.body.appendChild(document.createElement("div"));
    const origin = originIn(tree);
    origin.node.remove();
    expect(scrollMoves(tree, origin)).toBe(false);
  });
});
