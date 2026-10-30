import { act } from "react";
import { createRoot } from "react-dom/client";
import { afterEach, beforeAll, expect, it, vi } from "vitest";
import { forgetDocumentHistory, recordDocumentChange } from "../../../infra/history";
import { MultiViewEditor } from "./MultiViewEditor";
import type { EditorProvider } from "../registry/registry";

beforeAll(() => {
  (globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
});
afterEach(() => forgetDocumentHistory("/designer"));

it("routes designer undo and redo while leaving text, read-only and owned history alone", async () => {
  const host = document.createElement("div"), root = createRoot(host);
  const write = vi.fn();
  const provider: EditorProvider = {
    id: "design", label: "Designer", fileTypes: [], kind: "designer",
    component: () => <div tabIndex={0}><span>Canvas</span><div data-own-undo><input /></div><div className="cm-editor"><textarea /></div></div>,
  };
  const render = async (kind: EditorProvider["kind"] = "designer", readOnly = false) => {
    await act(async () => root.render(<MultiViewEditor path="/designer" content="after" onChange={write} providers={[{ ...provider, kind }]} readOnly={readOnly} />));
  };
  const press = async (selector: string, key: string, shiftKey = false) => {
    const event = new KeyboardEvent("keydown", { key, ctrlKey: true, shiftKey, bubbles: true, cancelable: true });
    await act(async () => host.querySelector(selector)!.dispatchEvent(event));
    return event.defaultPrevented;
  };
  try {
    await render();
    recordDocumentChange("/designer", "before", "after");
    expect(await press("span", "z")).toBe(true);
    expect(write).toHaveBeenLastCalledWith("before");
    expect(await press("span", "z", true)).toBe(true);
    expect(write).toHaveBeenLastCalledWith("after");
    write.mockClear();
    expect(await press("input", "z")).toBe(false);
    expect(await press("textarea", "z")).toBe(false);
    await render("text");
    expect(await press("span", "z")).toBe(false);
    await render("designer", true);
    expect(await press("span", "z")).toBe(false);
    expect(write).not.toHaveBeenCalled();
  } finally { await act(async () => root.unmount()); }
});
