# Substrate Platform

A brand-neutral IDE core — the shared foundation for building desktop,
IDE-style tools. It has no knowledge of any specific product; the same
relationship as JetBrains' IntelliJ Platform to IntelliJ IDEA/PyCharm/
WebStorm/Rider — one core, many products built on top.

It ships in two halves:

- a **Rust crate** (`substrate-platform`, this repo's root) — the backend
  pieces every IDE needs, with no UI code at all;
- a **React/TypeScript UI package** (`substrate-platform-ui`, in [`ui/`](ui))
  — the IDE chrome, docking shell and generic panels, written against the
  Rust core's shapes but not tied to any one host.

[Yog-IDLE](https://github.com/F000NKKK/Yog-IDLE) is the first product built
on both (a Tauri app depending on the crate via a path dependency and on the
UI package via `file:../Substrate-Platform/ui`).

## Rust crate

| Module | What it does |
| --- | --- |
| `task` | Spawn an external process non-blocking, streaming stdout/stderr line-by-line into a `LogSink`. `TaskHandle` offers `is_running`, `exit_status`, blocking `wait` and `cancel`. |
| `workflow` | Named, multi-step flows from TOML (`workflow.toml`): steps run strictly in order, stop at the first failure, can compose other workflows (cycles are detected) and get `${var}` substitution. `WorkflowHandle` offers `result`, `wait` and `cancel`. |
| `log` | A cloneable, append-only `LogSink` — drain it, or stream lines live via `on_push`. Retention is bounded (`DEFAULT_MAX_LINES`, nothing retained while streaming). |
| `pty` | A real, cross-platform PTY session via `portable-pty` (`PtySession`), with `PtyOptions` for shell/args/cwd/env and `spawn_utf8` / `Utf8Chunker` for output that never splits a multi-byte character. |
| `shell` | Detects which shells are actually installed (`detect_shells`). |
| `dir` | One-level directory listing (dirs first, case-insensitive), create/rename/remove, and file read/write as text or raw bytes. |
| `project` | `ProjectStandard` detection, `Project` and `Solution` (a TOML solution file, load/save). Hosts define their own concrete standards. |

Path-scoping (which files a host may touch) is deliberately the host's job,
one layer up.

## UI package (`ui/`)

Source-only (`main`/`types` point at `src/index.tsx`); the consuming app's
bundler compiles it. Peer-depends on React 19.

- **Docking shell** — `PlatformShell` plus its own dock engine in
  `src/dock` (no third-party docking library): a tabbed center dock,
  left/right/bottom tool-window docks with auto-hide flyouts and pinning,
  drag-and-drop between edges/center, floating panels, per-panel sizes and
  optional `localStorage` persistence of the whole layout.
- **Editors** — `resolveEditor(filename)` maps extensions to CodeMirror 6
  based editors (`TextEditor`, `CodeEditor` with a breakpoint gutter, and
  per-language variants), a lossless `.properties` grid editor and an
  `ImageEditor` (`isBinaryFile` tells a host to skip its text read).
- **Widgets** — menu bar, toolbar, run bar, status bar, tabs, tree /
  lazily-loaded `FileTree` (`useDirectoryTree`), `DataGrid`, context menus,
  `OutputPanel`, an xterm.js `TerminalPanel`, buttons, fields and color
  pickers.
- **Infra** — design tokens (`--sp-*` CSS variables), theme provider with
  accent and editor color schemes, icon registry, color utilities, hooks.
- **Windows / pages** — settings window, color picker window, appearance
  settings page.

Hosts inject their own backend calls as callbacks or configurable Tauri
command/event names (with defaults) — the package owns no backend.

## Development

```sh
cargo build
cargo test
cargo clippy --all-targets
```

The toolchain is pinned to Rust 1.99 via `rust-toolchain.toml`.

## License

Dual-licensed: **AGPL-3.0-only** (see [LICENSE](LICENSE)) — free for any
use, provided you comply with the AGPL — or a **Commercial License** for
closed-source use, free below $1,000/mo net profit and a sliding royalty
above that. See [COMMERCIAL-LICENSE.md](COMMERCIAL-LICENSE.md) for details.
