import { afterEach, describe, expect, it } from "vitest";
import { errorText, setLocale } from "./index";
import { messages } from "../host/host.generated";

afterEach(() => setLocale(null));

describe("errorText", () => {
  it("translates a backend message, values filled", () => {
    setLocale("ru");
    expect(errorText({ text: messages.git.cloneTargetNotEmpty, args: { path: "/r" } })).toBe("/r уже существует и не пуста.");
  });

  it("passes a tool's own words through", () => {
    expect(errorText({ text: "{detail}", args: { detail: "fatal: x" } })).toBe("fatal: x");
    expect(errorText(new Error("boom"))).toBe("boom");
    expect(errorText({ message: "service said" })).toBe("service said");
    expect(errorText("plain")).toBe("plain");
  });
});
