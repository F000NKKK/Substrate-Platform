import { afterEach, describe, expect, it, vi } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { forgetDocumentHistory, recordDocumentChange, setDocumentWriter, useDocumentHistory } from "./documentHistory";

describe("document history", () => {
  afterEach(() => {
    forgetDocumentHistory("/a");
    vi.useRealTimers();
  });

  it("undoes and redoes a designer's edits, a burst as one step", () => {
    vi.useFakeTimers();
    let text = "a";
    const write = (t: string) => (text = t);
    setDocumentWriter("/a", write);
    const edit = (next: string) => {
      recordDocumentChange("/a", text, next);
      text = next;
    };
    edit("b");
    vi.advanceTimersByTime(1000);
    // A drag: three changes in quick succession.
    edit("c1");
    edit("c2");
    edit("c3");
    const { result, rerender } = renderHook(() => useDocumentHistory("/a", text));
    expect(result.current.canUndo).toBe(true);
    act(() => result.current.undo());
    rerender();
    expect(text).toBe("b");
    act(() => result.current.undo());
    rerender();
    expect(text).toBe("a");
    expect(result.current.canUndo).toBe(false);
    act(() => result.current.redo());
    rerender();
    expect(text).toBe("b");
    // A new edit drops what could be redone.
    edit("d");
    rerender();
    expect(result.current.canRedo).toBe(false);
  });
});
