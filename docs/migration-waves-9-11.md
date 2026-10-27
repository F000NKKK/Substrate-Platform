# Migrating from the `substrate-platform` crate (waves 9–11)

The root crate `substrate-platform` and its Cargo features are gone: every
module is now a context split into domain, app and adapter crates under
[`crates/`](../crates) (layers and allowed edges: [`craftbook.toml`](../craftbook.toml),
vocabulary: [context-map.md](context-map.md)). There is no re-export facade.
A product depends on the crates it uses, by path:

```toml
substrate-workspace-fs = { path = "../Substrate-Platform/crates/substrate-workspace-fs" }
```

The crates start at `0.1.0`. `substrate-host-contract` and
`tauri-plugin-substrate` keep their names; their Cargo features changed target
(see the end of this page).

Waves 5–8 (earlier) already moved `run`, `process`, `git`, `search` and
`message`; this page lists everything the root crate still held.

## Old path to new path

| Old (`substrate_platform::…`) | New |
|---|---|
| `sync::Locked`, `sync::ReadWrite` | `substrate_lock::{Locked, ReadWrite}` |
| `framing::{write_frame, read_frame}` | `substrate_framing::{write_frame, read_frame}` |
| `text_view::{TextView, TextViews}` | `substrate_workspace_domain::{TextView, TextViews}` |
| `workspace::Workspace` | `substrate_workspace_app::Workspace`, built with `Workspace::new(Arc<dyn PathResolver>)`; the disk adapter is `substrate_workspace_fs::DiskPaths` |
| `Workspace::roots()` | `Workspace::boundary()` (returns a `Boundary`, `Boundary::check`) |
| `workspace::{Roots, WorkspaceError}` | `substrate_workspace_domain::{Roots, WorkspaceError}`; `WorkspaceError::Unreadable` is a struct variant `{ reason }` |
| `fs::dir::{DirectoryEntry, EntryKind}` | `substrate_workspace_domain::{DirectoryEntry, EntryKind}` (no serde: the plugin has its own DTOs) |
| `fs::dir::{list, create_file, create, rename, remove, read_file, read_file_bytes, write_file, write_file_bytes, copy}` | `substrate_workspace_fs::dir::…` |
| `fs::{copy_new, read_text, FsError}`, `fs::watch::{DirectoryWatcher, DEFAULT_QUIET}` | `substrate_workspace_fs::{copy_new, read_text, FsError, DirectoryWatcher, DEFAULT_QUIET}` |
| `terminal::{TerminalSize, Shell, SessionOptions, Utf8Chunker}`, `terminal::PtyError` | `substrate_terminal_domain::{TerminalSize, Shell, SessionOptions, Utf8Chunker, TerminalError}` (`Shell::new(program, label)`) |
| `terminal::PtySession` | the port `substrate_terminal_app::ports::{PtyOpener, Session}`; `substrate_terminal_app::spawn_utf8`; the adapter `substrate_terminal_pty::PortablePty` |
| `terminal::{detect_shells, default_shell}` | `substrate_terminal_app::detect_shells` (port `ShellProbe`), `substrate_terminal_pty::{SystemShells, default_shell}` |
| `lsp::{Language, Languages, ServerDefinition, Notification, LspError}`, `lsp::uri` | `substrate_lsp_domain::…`; `Languages::get` is now `Languages::find`; `ServerDefinition::initialization_options` is `Option<serde_json::Value>` |
| `lsp::Client` | `substrate_lsp_app::Client`; `Client::start(launcher, &server, cwd, log, on_notification)` takes a `&dyn ServiceLauncher` (use `substrate_run_process::SystemLauncher`) |
| `Languages` from TOML | `substrate_lsp_toml::parse_languages` |
| `dap::{AdapterId, Adapter, AdapterDefinition, Adapters, Command, Debuggee, EvaluateContext, Event, StopReason, ChangeReason, OutputCategory, FrameId, Seq, ThreadId, VariablesReference, DapError}` | `substrate_dap_domain::…` (the ids and `Event` have no serde) |
| `dap::Client` | `substrate_dap_app::Client`, started with a `ServiceLauncher` |
| `dap::Event` as JSON | `substrate_dap_app::wire::{EventDto, decode_event}` (and `StopReasonDto`, `ChangeReasonDto`, `OutputCategoryDto`) |
| `Adapters` from TOML | `substrate_dap_toml::parse_adapters` |
| `project::{Solution, Project, ProjectEntry, SolutionFolder, Item, Reference, Configuration, ConfigurationMapping, ConfiguredProperties, Section, ProjectStandard, ProjectError}` | `substrate_project_domain::…` (no serde derives: a format maps its file to the model itself) |
| `project::{SolutionFormat, ProjectFormat}` | `substrate_project_app::ports::{SolutionFormat, ProjectFormat}` |
| `project::Formats` | `substrate_project_app::Formats` |
| `ProjectStandard` from a file | `substrate_project_toml::parse_standards` (`[[standard]]` tables with `id` and `detect_files`, in detection order); a bad file is `ProjectError::InvalidStandards` |
| `project::messages`, `state::messages` | `substrate_project_domain::messages`, `substrate_state_json::messages`. New template: `project.INVALID_STANDARDS` (the UI's generated copy and the Russian catalog gained it) |
| `state::{read_json, write_json, private_dir, StateError, APP_STATE_FILE, WORKSPACE_FILE}` | `substrate_state_json::…` |
| `run::lock` helper inside `substrate-run-app` | `substrate_lock` |

New in wave 9: `substrate_run_app::ports::{Service, ServiceLauncher}` (a
long-running child with piped stdio, for language servers and debug adapters),
implemented by `substrate_run_process::SystemLauncher`.

## Cargo features

| Crate | Before | After |
|---|---|---|
| `substrate-platform` | `project`, `state`, `full`, … | removed |
| `substrate-host-contract` | features forwarded to `substrate-platform` | `dap`, `fs`, `git`, `lsp`, `project`, `search`, `state`, `terminal` pull in the matching domain crates (`substrate-dap-domain`, `substrate-workspace-fs`, `substrate-git-domain`, `substrate-lsp-domain`, `substrate-project-domain`, `substrate-search-domain`, `substrate-state-json`, `substrate-terminal-domain`); `full` is all of them |
| `tauri-plugin-substrate` | `dap`, `fs`, `git`, `lsp`, `search`, `state`, `terminal` forwarding to `substrate-platform` | same names, backed by the crates above; `tauri_plugin_substrate::Workspace` is still re-exported |

## Wave 11

- `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes
  and is the CI static gate; the workspace's clippy lints are `deny`.
- `substrate_project_domain::Solution::mapping(project, configuration)` moved
  to `ProjectEntry::mapping(&self, configuration)`.
- `substrate_lsp_app::Client` builds its JSON-RPC messages with the params
  moved in rather than copied; its public signatures are unchanged.
- `substrate_dap_domain::adapters` is flat: `Adapter`, `AdapterDefinition`,
  `AdapterId` and `Adapters` are still exported from the crate root.
- `substrate_run_domain::workflow::substitute` is generic over the map's
  hasher.
- `Message::arg` is `#[must_use]`.
- serde derives were already out of every domain crate after waves 5–10 (the
  DTOs live in `tauri-plugin-substrate`'s `wire` modules,
  `substrate_dap_app::wire` and the `*-toml` crates); wave 11 found nothing
  more to move.

## Gates

`cargo llvm-cov --package substrate-platform` is gone; CI measures per crate
(see `.github/workflows/quality.yml`).
