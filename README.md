<div align="center">

<a id="top"></a>

# 🧱 Substrate Platform

### A brand-neutral core for building IDE-style desktop tools

[![License: AGPL-3.0](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](LICENSE)
[![Commercial license](https://img.shields.io/badge/commercial-available-blueviolet.svg)](COMMERCIAL-LICENSE.md)
[![Rust 1.99](https://img.shields.io/badge/Rust-1.99-orange.svg)](rust-toolchain.toml)
[![React 19](https://img.shields.io/badge/React-19-61DAFB.svg)](https://react.dev/)
[![CodeMirror 6](https://img.shields.io/badge/CodeMirror-6-D30707.svg)](https://codemirror.net/)

🇺🇸 **English** | 🇷🇺 [Русский](README.ru.md)

[Overview](#-overview) • [Rust Crate](#-rust-crate) • [UI Package](#-ui-package) • [Getting Started](#-getting-started) • [Development](#-development) • [License](#-license)

</div>

---

## 📖 Overview

**Substrate Platform** is the shared foundation for desktop tools that look
and work like an IDE. It knows nothing about any particular product: think of
how the IntelliJ Platform relates to IntelliJ IDEA, PyCharm or Rider. One
core, many products on top.

It comes in two halves:

| Half | Where | What it gives you |
|---|---|---|
| 🦀 **Rust crate** `substrate-platform` | repo root | Process and workflow runner, log sink, PTY terminals, file system helpers, project/solution model. No UI code. |
| ⚛️ **UI package** `substrate-platform-ui` | [`ui/`](ui) | Docking shell, editors, widgets, dialogs, settings, theme and i18n for React. Owns no backend. |

The product brings its own domain logic: its file formats, project
standards, workflow templates and commands. Substrate only provides the
parts every IDE needs anyway.

[Yog-IDLE](https://github.com/F000NKKK/Yog-IDLE) is the first product built
on it (a Tauri app using both halves).

<p align="center"><img src="docs/images/shell.png" alt="The docking shell: menu bar, tabbed center dock with an editor, pinned Project Explorer and Properties, the Output dock, a floating Find and Replace window and the status bar" width="900"></p>

```mermaid
flowchart LR
    subgraph product["Your product"]
        ui["React UI<br/>domain editors, panels"]
        be["Backend commands<br/>(e.g. Tauri)"]
    end
    subgraph sp["Substrate Platform"]
        spui["substrate-platform-ui<br/>shell · editors · widgets"]
        sprs["substrate-platform<br/>terminal · fs · project · state"]
    end
    ui --> spui
    be --> sprs
    ui <-- "invoke / events" --> be
```

## 🦀 Rust Crate

Always there: `workspace` (the folders the user opened; paths outside them are refused), `sync` (locks that outlive a panic), `text_view` (a product's binary formats read and written as text, so search, replace and diffs see what the editor shows; the product builds a `TextViews` value and hands it to the plugin's `Options`). Beside it, [`crates/substrate-process`](crates/substrate-process) starts programs with the system's own environment and opens links and files, and [`crates/substrate-message`](crates/substrate-message) is the shared kernel (`Message`: user-facing text as templates the UI translates, and the `messages!` macro) and [`crates/substrate-host-contract`](crates/substrate-host-contract) is the contract with the UI (`commands`: the host commands by domain, the one list permissions, handlers and the UI's copy come from; `events`; the generator `cargo run -p substrate-host-contract --all-features --bin substrate-messages`). Every other module is a Cargo feature of its name, so a product builds only what it uses (`full` turns on all):

| Feature / module | What it does |
|---|---|
| `run` (no module of this crate) | Running tools, in four crates: [`substrate-run-domain`](crates/substrate-run-domain) (`Task`, `TaskExit`, `LogLine`, `Level`, and the `workflow` types `File`, `Definition`, `Step`, `Outcome`), [`substrate-run-app`](crates/substrate-run-app) (`spawn` runs an external process without blocking and streams stdout/stderr line by line into a `LogSink`; `TaskHandle` has `is_running`, `exit_status`, `wait` and `cancel`, which kills the whole process tree; `workflow::run`: steps run in order and stop at the first failure, a step can call another workflow (cycles are detected), `${var}` is substituted; the `ports::ProcessLauncher` port), [`substrate-run-process`](crates/substrate-run-process) (`SystemLauncher`, and `kill_tree`; the only `unsafe`) and [`substrate-run-toml`](crates/substrate-run-toml) (`parse_workflow`: a workflow file into the domain's `File`). `LogSink` is a cloneable, append-only log with bounded retention. |
| `terminal` | A real cross-platform pseudo-terminal (`PtySession`, via `portable-pty`) with `PtyOptions`; `Utf8Chunker` so output never splits a multi-byte character; `detect_shells()` lists the shells that are installed. |
| `fs` | `fs::dir`: one-level directory listing (folders first, case-insensitive), create/rename/remove, read/write as text or bytes. `fs::watch`: `DirectoryWatcher`, debounced change notifications. |
| `project` | The project and solution model and its API — no format of its own, and it reads no files. `Solution`: solution folders, configurations and platforms, each project's configuration per solution configuration, build dependencies (`build_order`), plus ordered properties and sections for everything else a format keeps; `Project`: properties (per configuration too), target frameworks, items, references. The product implements `SolutionFormat` / `ProjectFormat` for its files (`.sln`, `.slnx`, `.csproj`, its own) and registers them in `Formats`; `ProjectStandard` tells a project's kind from the file list the product gives it. |
| `lsp` | Language servers over the Language Server Protocol. `Languages`: the product's TOML list of languages — extensions, file names and the server that serves each (`command`, `args`, `env`, `initialization_options`); `for_path` picks a file's language. `lsp::Client`: starts a server (stderr into a `LogSink`) or connects to any stream, `initialize`/`shutdown`, requests matched to their responses with a timeout (and `$/cancelRequest`), the server's notifications to a callback, its own requests (`workspace/configuration`, progress, registration) answered; `did_open`/`did_change`/`did_save`/`did_close` with full-text sync. Params and results stay JSON, so a new language is a line in the list. `uri`: paths as `file://` URIs and back. |
| `dap` | Debug adapters over the Debug Adapter Protocol, the debugging counterpart of `lsp`. `Adapters`: the product's TOML list — `[adapter.<id>]` with the `languages` it debugs, `command`, `args`, `env` and `launch_defaults` (what every `launch` starts from); `AdapterId` names an adapter. `dap::Client`: starts an adapter (stderr into a `LogSink`) or connects to any stream, requests numbered with a `Seq` and matched to their responses with a timeout (several can wait at once: many adapters answer `launch` only after `configurationDone`), typed requests (`initialize`, `launch`, `attach`, `set_breakpoints`, `set_exception_breakpoints`, `configuration_done`, `threads`, `stack_trace`, `scopes`, `variables`, `resume`, `step_over`/`step_in`/`step_out`, `pause`, `evaluate`, `disconnect`, `terminate`) with `ThreadId`/`FrameId`/`VariablesReference` arguments, results as the adapter's JSON. What the adapter says unasked is a typed `Event` (`Initialized`, `Stopped`, `Continued`, `Output`, `Terminated`, `Exited`, `Breakpoint`, `Thread`; any other, or one with a body that does not fit, is kept as `Other { name, body }`); its own requests (`runInTerminal`, `startDebugging`) are answered with an error, since the client says in `initialize` that it does not do them. The framing is the one `lsp` uses. |
| `git` (the plugin's feature; no module of this crate) | Source Control over the `git` CLI, in three crates: [`substrate-git-domain`](crates/substrate-git-domain) (status, changes, branches, commits, stashes, diffs, `GitError`), [`substrate-git-app`](crates/substrate-git-app) (the ports, and the `WriteLock` the write operations take) and [`substrate-git-gitcli`](crates/substrate-git-gitcli) (`GitCli`, which runs `git` and parses its porcelain). |
| `search` (the plugin's feature; no module of this crate) | Find and Replace in Files, in [`substrate-search-domain`](crates/substrate-search-domain), [`substrate-search-app`](crates/substrate-search-app) and [`substrate-search-ignore`](crates/substrate-search-ignore): a `.gitignore`-aware walk of the project (or only the open documents' unsaved buffers), results file by file; what the walk leaves out is the product's `Layout`. |
| `state` | What an IDE remembers between runs: a project's and the app's JSON state; reading never fails, writing is atomic. |

The Tauri plugin `tauri-plugin-substrate` ([`tauri-plugin/`](tauri-plugin)) has the same features for its commands (`dap`, `fs`, `git`, `lsp`, `search`, `state`, `terminal`); `lsp`'s languages and `dap`'s adapters are the product's, passed in `Options::languages` and `Options::adapters`. `dap` has four commands (`dap_adapters`, `dap_start`, `dap_request`, `dap_stop`) and two events (`substrate://dap-event`, `substrate://dap-output`); the UI's `ui/src/dap` has the host, the shared `DapSession` (breakpoints per file, launch, run control, call stack, variables), a CodeMirror extension for the breakpoint gutter and the current line, and a small debug toolbar, call stack and variables panel.

The UI's `ui/src/inspector` is a product-neutral data inspector: `DataInspector` shows a value (Substrate Data's JSON form) and the `ShapeDoc` that describes it (`{ root, defs }`, as `substrate_data::json` writes it) as a property tree — structs, lists, maps, optionals, plain and tagged enums, scalars with their types. Props: `shape` (the `ShapeDoc` JSON, parsed with typed `InspectorError`s, or a `ShapeDescription`), `value`, `readOnly` (default `true`), `onEdit(path, value)` (only called when editable: bool, integer, float, string and enum variant, validated against the shape), `diagnostics` (`{ path, message }`, shown on rows and their ancestors) and `rootName`. The pure model (`parseShapeDescription`, `buildRows`) is exported too; the host decides what an edit does.

Which paths a host may touch is deliberately left to the host, one layer up.

### Run a task

```rust
use substrate_run_app::{LogSink, spawn};
use substrate_run_domain::Task;
use substrate_run_process::SystemLauncher;

let sink = LogSink::new();
sink.on_push(|line| println!("{:?} {}", line.level, line.text));

let task = Task::new("cargo").arg("build").cwd("/path/to/project");
let handle = spawn(&SystemLauncher, &task, sink)?;
let exit = handle.wait();
println!("success: {}, code: {:?}", exit.success, exit.code);
```

### Run a workflow

```toml
# workflow.toml
[workflow.build]
description = "Build in release mode"
steps = [{ run = "cargo", args = ["build", "--release"] }]

[workflow.ci]
steps = [
    { run = "cargo", args = ["fmt", "--check"] },
    { workflow = "build" },
    { run = "cargo", args = ["test"], env = { RUST_LOG = "${log}" } },
]
```

```rust
use std::collections::HashMap;
use std::sync::Arc;
use substrate_run_app::LogSink;
use substrate_run_app::workflow::{Request, run};
use substrate_run_process::SystemLauncher;
use substrate_run_toml::parse_workflow;

let file = parse_workflow(&std::fs::read_to_string("workflow.toml")?)?;
let vars = HashMap::from([("log".to_string(), "debug".to_string())]);
let request = Request { file, name: "ci".to_string(), vars, cwd: ".".into() };
let result = run(Arc::new(SystemLauncher), request, LogSink::new()).wait();
assert!(result.is_success());
```

## ⚛️ UI Package

Source only: `main` and `types` point at `src/index.tsx`, and the host's
bundler compiles it. Peer dependency: React 19.

### 🪟 Docking shell
`PlatformShell` with its own dock engine (no third-party docking library):
a tabbed center dock, left/right/bottom tool windows that auto-hide or pin,
drag and drop between edges and the center, floating panels, per-panel
sizes, and a menu bar and status bar slot. With `persistKey` the layout is
saved through the `StateStore` (`localStorage` by default, or the host's own
store, e.g. one file per project). `useShell()` lets any panel show, hide or
activate others.

```tsx
import { PlatformShell, ThemeProvider, StatusBar } from "substrate-platform-ui";

<ThemeProvider>
  <PlatformShell
    main={{ id: "editor", title: "Editor", component: EditorArea }}
    toolWindows={{
      left: [{ id: "files", title: "Files", component: FilesPanel }],
      bottom: [{ id: "output", title: "Output", component: OutputArea }],
    }}
    defaultPinned={{ left: "files" }}
    persistKey="my-app"
    statusBar={<StatusBar /* ... */ />}
  />
</ThemeProvider>
```

### ✍️ Editors
- `resolveEditor(filename)` picks an editor by extension: CodeMirror 6 text
  and code editors (breakpoint gutter, per-language highlighting), Markdown
  with preview (GFM, KaTeX, Mermaid), a `.properties` grid and an image viewer
- `DiffEditor` for side-by-side diffs
- Source Control windows (`GitHost`) and Find and Replace in Files (`FindHost`,
  `FindInFilesWindow`, `FindResultsPanel`) over the plugin's `git` and `search`
- `WorkspaceProvider`: the open folders, and what happens when the window closes

### 🧩 Widgets
| Group | Widgets |
|---|---|
| Chrome | `MenuBar`, `Toolbar`, `RunBar`, `StatusBar`, `Tab`, `WindowControls`, `FloatingWindow`, `SplitPane` |
| Data | `Tree`, `FileTree` + `useDirectoryTree`, `DataGrid`, `PropertyGrid`, `ComponentList`, form fields (`FormPage`, `CommitTextField`, `SelectField`...) |
| Output | `OutputPanel`, `TerminalPanel` (xterm.js on a real PTY), `SearchResults`, `HighlightedText`, `SymbolIcon` |
| Interaction | `Dialog` (`showDialog`, `confirmDialog`, `promptDialog`), `notify`, `QuickPick`, `SearchField`, `ContextMenu` |
| Design | `DesignSurface` (selection, move/resize adorners), `ColorWheel`, `Swatch`, color fields |
| Basics | `Button`, `IconButton`, `Label`, `TextField` |

### 🎨 Infrastructure
- Design tokens as `--sp-*` CSS variables, `ThemeProvider` with accent color
  and editor color schemes, an icon registry, color utilities
- `StateStore` for UI state that survives restarts, selection and keyboard
  shortcut helpers, drag-and-drop helpers, an outline model
- Diagnostics (problems placed in a document), file types, undo history
- i18n with English and Russian built in
- `SettingsWindow` and `ColorPickerWindow`, with ready Appearance, Editor and
  International settings pages

Backend calls are passed in as callbacks or as configurable Tauri command
and event names with defaults, so the package never assumes a backend of
its own.

## 🚀 Getting Started

The crate and the UI package aren't published yet; use path dependencies:

```toml
# Cargo.toml
[dependencies]
substrate-platform = { path = "../Substrate-Platform" }
```

```jsonc
// package.json
"dependencies": {
  "substrate-platform-ui": "file:../Substrate-Platform/ui"
}
```

Run `npm install` in `Substrate-Platform/ui` once so its own dependencies
(CodeMirror, xterm.js...) are in place.

## 🛠️ Development

```sh
cargo build
cargo test
cargo clippy --all-targets
```

The toolchain is pinned to Rust 1.99 in `rust-toolchain.toml`.

Contribution rule: everything here must stay generic. A feature that only
makes sense for one product belongs in that product.

## 📄 License

Dual-licensed: **AGPL-3.0-only** ([LICENSE](LICENSE)), free for any use that
complies with the AGPL, or a **Commercial License** for closed-source use,
free below $1,000/month net profit with a sliding royalty above that. See
[COMMERCIAL-LICENSE.md](COMMERCIAL-LICENSE.md).

---

<div align="center">

**[⬆ Back to Top](#top)**

</div>
