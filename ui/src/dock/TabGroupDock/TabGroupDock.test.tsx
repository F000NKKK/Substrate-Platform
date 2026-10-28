import { act, createElement } from "react";
import { createRoot } from "react-dom/client";
import { beforeAll, expect, it, vi } from "vitest";
import { TabGroupDock } from "./TabGroupDock";
import type { ShellLayout } from "../hooks/useShellLayout";

beforeAll(() => {
  (globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
});

function layoutOf(pinned: string[]) {
  const panel = (id: string) => ({ id, title: id, component: () => createElement("p", null, `body of ${id}`) });
  const layout = {
    idsByAnchor: () => ["a", "b"],
    pinnedInAnchor: () => pinned,
    panelsById: { a: panel("a"), b: panel("b") },
    anchorSize: () => 200,
    setAnchorSize: vi.fn(),
    hide: vi.fn(),
    pin: vi.fn(),
    close: vi.fn(),
    dockTo: vi.fn(),
    floatAt: vi.fn(),
  };
  return layout as unknown as ShellLayout & typeof layout;
}

async function mount(layout: ShellLayout) {
  const host = document.createElement("div");
  await act(async () => createRoot(host).render(createElement(TabGroupDock, { anchor: "bottom", layout })));
  return host;
}

it("lists every panel of the edge as a tab and shows only the pinned one", async () => {
  const host = await mount(layoutOf(["b"]));
  expect(host.querySelectorAll('[role="tab"]').length).toBe(2);
  expect(host.textContent).toContain("body of b");
  expect(host.textContent).not.toContain("body of a");
});

it("steps the others back when several are pinned", async () => {
  const layout = layoutOf(["a", "b"]);
  await mount(layout);
  expect(layout.hide).toHaveBeenCalledTimes(1);
});

it("pins the tab that is clicked", async () => {
  const layout = layoutOf(["b"]);
  const host = await mount(layout);
  await act(async () => (host.querySelectorAll('[role="tab"]')[0] as HTMLElement).click());
  expect(layout.pin).toHaveBeenCalledWith("a");
});
