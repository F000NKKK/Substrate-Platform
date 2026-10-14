import { act, createElement } from "react";
import { createRoot } from "react-dom/client";
import { beforeAll, expect, it, vi } from "vitest";
import { ErrorBoundary } from "./ErrorBoundary";

beforeAll(() => {
  (globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
});

it("keeps a crash inside: the message, and Try Again mounts it afresh", async () => {
  vi.spyOn(console, "error").mockImplementation(() => {});
  let fail = true;
  function Flaky() {
    if (fail) throw new Error("boom");
    return createElement("span", null, "fine");
  }
  const host = document.createElement("div");
  await act(async () => createRoot(host).render(createElement(ErrorBoundary, { what: "The Editor", children: createElement(Flaky) })));
  expect(host.textContent).toContain("The Editor failed: boom");
  fail = false;
  await act(async () => host.querySelector("button")!.click());
  expect(host.textContent).toBe("fine");
});
