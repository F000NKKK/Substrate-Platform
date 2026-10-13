import { describe, expect, it, vi } from "vitest";

vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn() }));

import { appendOutputLine, clearOutputBuffer, getOutputBuffer, getOutputBufferStart } from "./outputBuffer";

describe("outputBuffer", () => {
  it("keeps a stable absolute start across cap trimming and clear", () => {
    for (const line of ["a", "b", "c", "d"]) appendOutputLine("test", line, 3);
    expect(getOutputBuffer("test")).toEqual(["b", "c", "d"]);
    expect(getOutputBufferStart("test")).toBe(1);
    clearOutputBuffer("test");
    expect(getOutputBuffer("test")).toEqual([]);
    expect(getOutputBufferStart("test")).toBe(4);
    appendOutputLine("test", "e", 3);
    expect(getOutputBufferStart("test") + getOutputBuffer("test").indexOf("e")).toBe(4);
  });
});
