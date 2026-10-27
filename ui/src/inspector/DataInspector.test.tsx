import { act, createElement } from "react";
import { createRoot } from "react-dom/client";
import { afterEach, beforeAll, describe, expect, it, vi } from "vitest";
import { setLocale } from "../infra/i18n";
import { DataInspector, type DataInspectorProps } from "./DataInspector";
import type { DataValue } from "./dataValue";
import { lightShape, lightValue, motorShape, motorValue, transformShape, transformValue } from "./substrateFixtures";

beforeAll(() => {
  (globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
});
afterEach(() => {
  setLocale(null);
  document.body.innerHTML = "";
});

function mount(props: DataInspectorProps): HTMLElement {
  const host = document.createElement("div");
  document.body.append(host);
  act(() => createRoot(host).render(createElement(DataInspector, props)));
  return host;
}

function rowNamed(host: HTMLElement, name: string): HTMLElement {
  const row = [...host.querySelectorAll<HTMLElement>("[role=treeitem]")].find((item) => item.querySelector(".sp-inspector-name")?.textContent === name);
  if (!row) throw new Error(`no row ${name}`);
  return row;
}

function type(input: HTMLInputElement, text: string): void {
  const setter = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value")?.set;
  act(() => {
    setter?.call(input, text);
    input.dispatchEvent(new Event("input", { bubbles: true }));
  });
}

describe("DataInspector", () => {
  it("renders a Transform-like value as a tree of typed rows", () => {
    const host = mount({ shape: transformShape, value: transformValue, rootName: "Transform" });
    expect(host.querySelector("[role=tree]")).not.toBeNull();
    const translation = rowNamed(host, "translation");
    expect(translation.textContent).toContain("[f32]");
    expect(translation.textContent).toContain("3 items");
    expect(rowNamed(host, "0").textContent).toContain("1");
    expect(rowNamed(host, "mode").textContent).toContain("fast");
    expect(rowNamed(host, "parent").textContent).toContain("none");
    expect(rowNamed(host, "visible").textContent).toContain("true");
  });

  it("renders the editor backend's real transform component", () => {
    const host = mount({ shape: motorShape, value: motorValue, rootName: "transform" });
    expect(rowNamed(host, "motor").textContent).toContain("motor");
    expect(rowNamed(host, "rotor").textContent).toContain("4 items");
    expect(rowNamed(host, "dual").textContent).toContain("none");
  });

  it("shows a tagged enum's variant and its payload", () => {
    const host = mount({ shape: lightShape, value: lightValue, rootName: "light" });
    expect(rowNamed(host, "light").textContent).toContain("spot");
    expect(rowNamed(host, "angle").textContent).toContain("1");
  });

  it("is read-only by default, whatever onEdit is", () => {
    const host = mount({ shape: transformShape, value: transformValue, onEdit: vi.fn() });
    expect(host.querySelector("input, select")).toBeNull();
  });

  it("is read-only when there is nobody to take the edit", () => {
    const host = mount({ shape: transformShape, value: transformValue, readOnly: false });
    expect(host.querySelector("input, select")).toBeNull();
  });

  it("edits a scalar and reports a typed path and a JSON value", () => {
    const onEdit = vi.fn();
    const host = mount({ shape: transformShape, value: transformValue, onEdit, readOnly: false });
    const count = rowNamed(host, "count").querySelector("input")!;
    type(count, "9");
    act(() => count.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true })));
    expect(onEdit).toHaveBeenCalledWith(["count"], 9);
    const first = rowNamed(host, "0").querySelector("input")!;
    type(first, "2.5");
    act(() => first.dispatchEvent(new FocusEvent("focusout", { bubbles: true })));
    expect(onEdit).toHaveBeenCalledWith(["translation", 0], 2.5);
  });

  it("does not report an invalid number and says why", () => {
    const onEdit = vi.fn();
    const host = mount({ shape: transformShape, value: transformValue, onEdit, readOnly: false });
    const count = rowNamed(host, "count").querySelector("input")!;
    type(count, "300");
    act(() => count.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true })));
    expect(onEdit).not.toHaveBeenCalled();
    expect(count.getAttribute("aria-invalid")).toBe("true");
    expect(count.title).toBe("Enter a value from 0 to 255.");
  });

  it("toggles a bool and switches enum variants", () => {
    const onEdit = vi.fn();
    const host = mount({ shape: transformShape, value: transformValue, onEdit, readOnly: false });
    act(() => rowNamed(host, "visible").querySelector("input")!.click());
    expect(onEdit).toHaveBeenLastCalledWith(["visible"], false);
    const mode = rowNamed(host, "mode").querySelector("select")!;
    act(() => {
      mode.value = "slow";
      mode.dispatchEvent(new Event("change", { bubbles: true }));
    });
    expect(onEdit).toHaveBeenLastCalledWith(["mode"], "slow");
    const light = rowNamed(host, "light").querySelector("select")!;
    act(() => {
      light.value = "off";
      light.dispatchEvent(new Event("change", { bubbles: true }));
    });
    expect(onEdit).toHaveBeenLastCalledWith(["light"], { kind: "off" });
  });

  it("shows a bad shape as an error, not a crash", () => {
    const host = mount({ shape: { root: { kind: "def", name: "Ghost" }, defs: {} }, value: {} as DataValue });
    expect(host.querySelector("[role=alert]")?.textContent).toBe("/root: no definition named Ghost.");
  });

  it("shows diagnostics on their rows, and opens the branches that hold them", () => {
    const host = mount({ shape: transformShape, value: transformValue, diagnostics: [{ path: ["light", "color", 2], message: "out of gamut" }] });
    expect(rowNamed(host, "light").querySelector("[role=img]")?.getAttribute("aria-label")).toBe("1 problems inside");
    const labels = [...host.querySelectorAll("[role=img]")].map((mark) => mark.getAttribute("aria-label"));
    expect(labels).toContain("out of gamut");
  });

  it("shows a mismatching value with the problem", () => {
    const host = mount({ shape: transformShape, value: { ...(transformValue as object), count: "seven" } as DataValue });
    expect(rowNamed(host, "count").textContent).toContain("Expected int, found string.");
  });

  it("speaks Russian", () => {
    setLocale("ru");
    const host = mount({ shape: transformShape, value: transformValue });
    expect(rowNamed(host, "translation").textContent).toContain("3 элемента");
    expect(rowNamed(host, "parent").textContent).toContain("нет");
    expect(host.querySelector(".sp-inspector")?.getAttribute("aria-label")).toBe("Инспектор данных");
  });

  it("collapses and expands with the keyboard like any tree", () => {
    const host = mount({ shape: transformShape, value: transformValue });
    const color = rowNamed(host, "color");
    expect(color.getAttribute("aria-expanded")).toBe("false");
    act(() => color.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowRight", bubbles: true })));
    expect(rowNamed(host, "color").getAttribute("aria-expanded")).toBe("true");
  });
});
