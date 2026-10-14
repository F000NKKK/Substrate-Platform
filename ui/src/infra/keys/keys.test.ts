import { afterEach, describe, expect, it } from "vitest";
import { eventKey, keybindingLabel, matchesShortcut, parseShortcut, registerKeybinding, setKeymapOverrides } from "./keys";

const press = (init: KeyboardEventInit, target: EventTarget = window) => {
  const e = new KeyboardEvent("keydown", { bubbles: true, cancelable: true, ...init });
  target.dispatchEvent(e);
  return e;
};

const cleanups: (() => void)[] = [];
const bind = (keys: string | string[], run: () => void | boolean, id?: string) => cleanups.push(registerKeybinding({ id, keys, run }));

afterEach(() => {
  cleanups.splice(0).forEach((c) => c());
  setKeymapOverrides({});
});

describe("shortcuts", () => {
  it("normalize however they're written", () => {
    expect(parseShortcut("Shift+Ctrl+S")).toEqual(["ctrl+shift+s"]);
    expect(parseShortcut("Cmd+Option+Esc")).toEqual(["ctrl+alt+escape"]);
    expect(parseShortcut("Ctrl+K  V")).toEqual(["ctrl+k", "v"]);
    expect(parseShortcut("Ctrl++")).toEqual(["ctrl++"]);
    expect(parseShortcut("Ctrl+Shift++")).toEqual(["ctrl+shift++"]);
  });

  it("match the physical key in any layout", () => {
    expect(eventKey({ key: "ы", code: "KeyS" })).toBe("s");
    expect(eventKey({ key: "q", code: "KeyA" })).toBe("q"); // AZERTY's own letter
    expect(eventKey({ key: "&", code: "Digit1" })).toBe("1");
    expect(eventKey({ key: "ю", code: "Period" })).toBe(".");
    expect(eventKey({ key: "ArrowUp", code: "ArrowUp" })).toBe("arrowup");
    expect(matchesShortcut(new KeyboardEvent("keydown", { key: "ы", code: "KeyS", ctrlKey: true }), "Ctrl+S")).toBe(true);
    expect(matchesShortcut(new KeyboardEvent("keydown", { key: "s", code: "KeyS", metaKey: true }), "Ctrl+S")).toBe(true);
    expect(matchesShortcut(new KeyboardEvent("keydown", { key: "S", code: "KeyS", ctrlKey: true, shiftKey: true }), "Ctrl+S")).toBe(false);
  });
});

describe("keybindings", () => {
  it("run the latest registration first, and a declining one passes on", () => {
    const ran: string[] = [];
    bind("Ctrl+S", () => void ran.push("shell"));
    bind("Ctrl+S", () => (ran.push("editor"), false));
    const e = press({ key: "s", code: "KeyS", ctrlKey: true });
    expect(ran).toEqual(["editor", "shell"]);
    expect(e.defaultPrevented).toBe(true);
  });

  it("wait for a chord's second step, and an unbound one is an ordinary press", () => {
    const ran: string[] = [];
    bind("Ctrl+K V", () => void ran.push("chord"));
    bind("V", () => void ran.push("v"));
    expect(press({ key: "k", code: "KeyK", ctrlKey: true }).defaultPrevented).toBe(false);
    press({ key: "Control", code: "ControlLeft", ctrlKey: true });
    press({ key: "v", code: "KeyV" });
    expect(ran).toEqual(["chord"]);
    press({ key: "k", code: "KeyK", ctrlKey: true });
    press({ key: "x", code: "KeyX" });
    press({ key: "v", code: "KeyV" });
    expect(ran).toEqual(["chord", "v"]);
  });

  it("leave Ctrl+<character> to a terminal but not function keys", () => {
    const ran: string[] = [];
    bind("Ctrl+W", () => void ran.push("w"));
    bind("Ctrl+F4", () => void ran.push("f4"));
    const terminal = document.createElement("div");
    terminal.setAttribute("data-sp-native-keys", "");
    document.body.append(terminal);
    press({ key: "w", code: "KeyW", ctrlKey: true }, terminal);
    press({ key: "F4", code: "F4", ctrlKey: true }, terminal);
    terminal.remove();
    expect(ran).toEqual(["f4"]);
  });

  it("follow the user's keymap", () => {
    const ran: string[] = [];
    bind("Ctrl+S", () => void ran.push("save"), "file.save");
    expect(keybindingLabel("file.save")).toBe("Ctrl+S");
    setKeymapOverrides({ "file.save": ["Alt+S"] });
    expect(keybindingLabel("file.save")).toBe("Alt+S");
    press({ key: "s", code: "KeyS", ctrlKey: true });
    press({ key: "s", code: "KeyS", altKey: true });
    expect(ran).toEqual(["save"]);
    setKeymapOverrides({ "file.save": [] });
    press({ key: "s", code: "KeyS", ctrlKey: true });
    expect(ran).toEqual(["save"]);
  });
});
