# Context map

Substrate-Platform's bounded contexts and how they relate (Craftbook
[§7.2](https://github.com/F000NKKK/craftbook/blob/main/rust/07-architecture-ddd.md#72-bounded-contexts-and-the-context-map)).
The measured gap and the migration plan are in Craftbook's
`adoption/substrate-platform.md`; its words are in [glossary.md](glossary.md).

## Today

`substrate-message` (shared kernel) and `substrate-host-contract` (the
command and event catalog, the generator of the UI's copy) are split out
(wave 5), and source-control is split into `substrate-git-domain`, `substrate-git-app` and `substrate-git-gitcli` (wave 6) and search into `substrate-search-domain`, `-app` and `-ignore` (wave 7), run and process (wave 8), workspace, terminal, language-server and debug-adapter (wave 9). `substrate-installer-domain` and `substrate-installer-app` (the installer context: the engine of an installer like Visual Studio Installer, phase 1, no adapters yet; see [installer.md](installer.md)) are new. `substrate-platform` still holds project and state, each behind a
Cargo feature of its name; `tauri-plugin-substrate` is the interface layer
(Tauri commands, the composition root). The split goes on context by context.

## Contexts

| Context | Today (module, feature) | What it owns | Target crates (layer) |
|---|---|---|---|
| message (shared kernel) | `substrate-message`, every `*/messages.rs` | user-facing text as templates the UI translates | `substrate-message` (shared kernel) |
| workspace | `substrate-workspace-domain`, `substrate-workspace-app`, `substrate-workspace-fs` (wave 9; feature `fs` of the plugin) | the folders the user opened; listing, changing and watching them; product formats read as text | as split: `PathResolver` is the port, `substrate-workspace-fs` the adapter |
| source-control | `substrate-git-domain`, `substrate-git-app`, `substrate-git-gitcli` (wave 6; feature `git` of the plugin) | status, staging, commits, branches, remotes, stashes, diffs, through the `git` executable | as split: the gitcli adapter is the anticorruption layer for git's porcelain |
| search | `substrate-search-domain`, `substrate-search-app`, `substrate-search-ignore` (wave 7; feature `search` of the plugin) | Find and Replace in Files, the project walk | `substrate-search-domain`, `substrate-search-app`, `substrate-search-ignore` (infra) |
| run | `substrate-run-domain`, `substrate-run-app`, `substrate-run-process`, `substrate-run-toml` (wave 8) | tasks streamed into a log, workflows of them | as split: `ProcessLauncher` is the port, `substrate-run-process` the adapter, `substrate-run-toml` the file-format anticorruption layer |
| terminal | `substrate-terminal-domain`, `substrate-terminal-app`, `substrate-terminal-pty` (wave 9) | pty sessions, the installed shells | as split: `PtyOpener` is the port, `substrate-terminal-pty` the adapter |
| language-server | `substrate-lsp-domain`, `substrate-lsp-app`, `substrate-lsp-toml` (wave 9) | the product's languages and their servers, spoken to over LSP | as split: the toml crate is the file-format anticorruption layer, servers and adapters start through `ServiceLauncher` |
| debug-adapter | `substrate-dap-domain`, `substrate-dap-app`, `substrate-dap-toml` (wave 9) | the product's debug adapters, spoken to over DAP | as split: the toml crate is the file-format anticorruption layer, servers and adapters start through `ServiceLauncher` |
| project-model | `project` | solutions and projects as an IDE models them | `substrate-project-domain`, `substrate-project-app` |
| ide-state | `state` | what the IDE remembers between runs | `substrate-state-app`, `substrate-state-json` (infra) |
| installer | `substrate-installer-domain`, `substrate-installer-app` (phase 1; adapters come next, see [installer.md](installer.md)) | the catalog of products, workloads and components and where each is published; releases, selections, plans, compatibility, what is installed; listing, planning and applying through ports | as split: the ports (`ReleaseSource`, `Downloader`, `Unpacker`, `InstalledStore`, `ToolRunner`, `InstallLayout`, `InstallFiles`, `ProductInstaller`, `SystemDetector`) are in the app crate; adapter crates (infra) implement them |
| process | `substrate-process` (wave 8) | starting programs with the system's own environment, opening links and files | `substrate-process` (infra) |
| host-ui-contract | `substrate-host-contract`: `commands`, `events`, `bin/substrate-messages` | the command and event catalog the UI is generated from | `substrate-host-contract` (interface), `tauri-plugin-substrate` (interface) |

`substrate-lock` (`Locked`, `ReadWrite`) and `substrate-framing`
(Content-Length framing) are shared-kernel crates used by several contexts.

## Relations

| From | To | Relation | Why |
|---|---|---|---|
| search | workspace | customer–supplier | reads files through their text view |
| source-control | workspace | customer–supplier | diffs read files through their text view |
| source-control | process | customer–supplier | `git` runs with the system's environment |
| run | process | customer–supplier | tasks run with the system's environment |
| terminal | process | customer–supplier | shells start with the system's environment |
| language-server | run | customer–supplier | a server's log is a `LogSink`; stopping it kills its process group |
| language-server | process | customer–supplier | servers start with the system's environment |
| debug-adapter | run | customer–supplier | an adapter's log is a `LogSink`; stopping it kills its process group |
| debug-adapter | process | customer–supplier | adapters start with the system's environment |
| host-ui-contract | message | published language | the UI's generated copy of every template |
| every context | message | shared kernel | what it says to the user |

`tauri-plugin-substrate` is the composition root and depends on every context.

## Rules kept from `CONTRIBUTING.md`

- Substrate stays generic: no product-specific formats or logic. Project
  formats are product adapters of `SolutionFormat`/`ProjectFormat`; product
  binary formats are product implementations of a text view.
- The wire format the UI relies on (Tauri command names and argument names,
  event names and payloads) changes only together with `ui/`.
