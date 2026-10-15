import { afterEach, describe, expect, it, vi } from "vitest";
import { documentAction, documentHistory, forgetDocumentHistory, pushDocumentStep, recordDocumentChange, setDocumentWriter } from "./documentHistory";

describe("document history", () => {
  let text = "a";
  const edit = (next: string) => {
    recordDocumentChange("/a", text, next);
    text = next;
  };
  afterEach(() => {
    forgetDocumentHistory("/a");
    vi.useRealTimers();
    text = "a";
  });

  it("undoes and redoes a designer's text edits, a burst as one step", () => {
    vi.useFakeTimers();
    setDocumentWriter("/a", (t) => (text = t));
    edit("b");
    vi.advanceTimersByTime(1000);
    // A drag: three changes in quick succession.
    edit("c1");
    edit("c2");
    edit("c3");
    documentHistory("/a").undo();
    expect(text).toBe("b");
    documentHistory("/a").undo();
    expect(text).toBe("a");
    expect(documentHistory("/a").canUndo).toBe(false);
    documentHistory("/a").redo();
    expect(text).toBe("b");
    // A new edit drops what could be redone.
    vi.advanceTimersByTime(1000);
    edit("d");
    expect(documentHistory("/a").canRedo).toBe(false);
  });

  it("takes an action's text and other steps back as one", () => {
    setDocumentWriter("/a", (t) => (text = t));
    let pixels = 1;
    documentAction("/a", () => {
      edit("with a texture");
      pushDocumentStep("/a", { undo: () => (pixels = 1), redo: () => (pixels = 2) });
      pixels = 2;
    });
    documentHistory("/a").undo();
    expect([text, pixels]).toEqual(["a", 1]);
    documentHistory("/a").redo();
    expect([text, pixels]).toEqual(["with a texture", 2]);
  });
});
