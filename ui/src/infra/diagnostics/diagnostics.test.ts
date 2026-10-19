import { describe, expect, it } from "vitest";
import { createRustcParser } from "./rustc";
import { clearDiagnostics, getDiagnostics, setDiagnostics } from "./store";

function parseAll(text: string, options = {}) {
  const parser = createRustcParser(options);
  return [...text.split("\n").flatMap((l) => parser.push(l)), ...parser.flush()];
}

describe("createRustcParser", () => {
  it("reads human-format errors and warnings with their location and lint", () => {
    const out = parseAll(
      [
        "   Compiling my_app v0.1.0 (/p/build)",
        "error[E0425]: cannot find value `x` in this scope",
        " --> ../src/lib.rs:3:5",
        "  |",
        "3 |     x",
        "  |     ^ not found in this scope",
        "",
        "warning: unused variable: `y`",
        "  --> ../src/foo.rs:10:9",
        "   |",
        "   = note: `#[warn(unused_variables)]` on by default",
        "",
        "warning: `my_app` (lib) generated 1 warning",
        "error: could not compile `my_app` (lib) due to 1 previous error; 1 warning emitted",
      ].join("\n"),
      { resolvePath: (p: string) => `/p/build/${p}` }
    );
    expect(out).toEqual([
      {
        severity: "error",
        message: "cannot find value `x` in this scope",
        code: "E0425",
        codeHref: "https://doc.rust-lang.org/error_codes/E0425.html",
        file: "/p/build/../src/lib.rs",
        line: 3,
        column: 5,
        project: "my_app",
        tool: "rustc",
        origin: "build",
      },
      {
        severity: "warning",
        message: "unused variable: `y`",
        code: "unused_variables",
        codeHref: undefined,
        file: "/p/build/../src/foo.rs",
        line: 10,
        column: 9,
        project: "my_app",
        tool: "rustc",
        origin: "build",
      },
    ]);
  });

  it("keeps the first location, not a note's", () => {
    const out = parseAll(["warning: unused import: `a`", " --> src/a.rs:1:5", "note: the lint level is defined here", " --> src/lib.rs:1:9"].join("\n"));
    expect(out).toHaveLength(1);
    expect(out[0]).toMatchObject({ file: "src/a.rs", line: 1 });
  });

  it("reads cargo JSON messages", () => {
    const line = JSON.stringify({
      reason: "compiler-message",
      target: { name: "my_app" },
      message: {
        message: "this `return` is needless",
        level: "warning",
        code: { code: "clippy::needless_return" },
        spans: [{ file_name: "src/lib.rs", line_start: 7, column_start: 2, is_primary: true }],
      },
    });
    expect(parseAll(line)).toEqual([
      expect.objectContaining({
        severity: "warning",
        code: "clippy::needless_return",
        codeHref: "https://rust-lang.github.io/rust-clippy/master/index.html#needless_return",
        tool: "clippy",
        file: "src/lib.rs",
        line: 7,
        project: "my_app",
      }),
    ]);
  });

  it("skips JSON tallies", () => {
    const line = JSON.stringify({ $message_type: "diagnostic", message: "aborting due to 2 previous errors", level: "error", spans: [] });
    expect(parseAll(line)).toEqual([]);
  });
});

describe("diagnostics store", () => {
  it("replaces sets and clears by origin", () => {
    setDiagnostics("build", [{ severity: "error", message: "a", origin: "build" }]);
    setDiagnostics("live:x", [{ severity: "warning", message: "b", origin: "live" }]);
    expect(getDiagnostics().map((d) => d.message)).toEqual(["a", "b"]);
    clearDiagnostics({ origin: "build" });
    expect(getDiagnostics().map((d) => d.message)).toEqual(["b"]);
    setDiagnostics("live:x", []);
    expect(getDiagnostics()).toEqual([]);
  });
});
