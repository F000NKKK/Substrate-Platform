import { act } from "react";
import { createRoot } from "react-dom/client";
import { afterEach, beforeAll, expect, it, vi } from "vitest";
import { hostFiles } from "../../../infra/host";
import { ImageEditor } from "./ImageEditor";

vi.mock("../../../infra/host", () => ({ hostFiles: { allow: vi.fn() } }));
vi.mock("@tauri-apps/api/core", () => ({ convertFileSrc: (path: string) => `asset://${path}` }));
vi.mock("../../ZoomView", () => ({ ZoomView: ({ children }: { children: React.ReactNode }) => children }));

beforeAll(() => {
  (globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });

it("waits for asset permission before reading an old cached PNG and permits retry", async () => {
  let permit!: () => void;
  const loaded: string[] = [];
  vi.stubGlobal("Image", class {
    onload: (() => void) | null = null;
    onerror: (() => void) | null = null;
    naturalWidth = 32; naturalHeight = 32;
    set src(value: string) { loaded.push(value); this.onload?.(); }
  });
  const allow = vi.mocked(hostFiles.allow);
  allow.mockRejectedValueOnce(new Error("temporarily unavailable"));
  allow.mockImplementationOnce(() => new Promise<void>(resolve => { permit = resolve; }));
  const host = document.createElement("div"), root = createRoot(host);
  const path = "/Документы/Example/.meridian-idle/thumbnails/old-v2.png";
  await act(async () => root.render(<ImageEditor path={path} content="" onChange={() => {}} />));
  expect(host.querySelector('[role="alert"]')?.textContent).toContain("temporarily unavailable");
  expect(loaded).toEqual([]);
  await act(async () => host.querySelector("button")!.click());
  expect(loaded).toEqual([]);
  await act(async () => permit());
  expect(loaded).toEqual([`asset://${path}?v=0&retry=1`]);
  expect(host.querySelector("img")?.getAttribute("src")).toContain("old-v2.png");
  expect(host.querySelector('[role="alert"]')).toBeNull();
  await act(async () => root.unmount());
});
