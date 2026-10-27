# Context map

Substrate-Platform's bounded contexts and how they relate (Craftbook
[§7.2](https://github.com/F000NKKK/craftbook/blob/main/rust/07-architecture-ddd.md#72-bounded-contexts-and-the-context-map)).
The measured gap and the migration plan are in Craftbook's
`adoption/substrate-platform.md`; its words are in [glossary.md](glossary.md).

## Today

`substrate-message` (shared kernel) and `substrate-host-contract` (the
command and event catalog, the generator of the UI's copy) are split out
(wave 5). `substrate-platform` still holds every other context, each behind a
Cargo feature of its name; `tauri-plugin-substrate` is the interface layer
(Tauri commands, the composition root). The split goes on context by context.

## Contexts

| Context | Today (module, feature) | What it owns | Target crates (layer) |
|---|---|---|---|
| message (shared kernel) | `substrate-message`, every `*/messages.rs` | user-facing text as templates the UI translates | `substrate-message` (shared kernel) |
| workspace | `workspace`, `fs`, `text_view` | the folders the user opened; listing, changing and watching them; product formats read as text | `substrate-workspace-domain`, `substrate-workspace-fs` (infra) |
| source-control | `git` | status, staging, commits, branches, remotes, stashes, diffs, through the `git` executable | `substrate-git-domain`, `substrate-git-app`, `substrate-git-gitcli` (infra; the anticorruption layer for git's porcelain) |
| search | `search` | Find and Replace in Files, the project walk | `substrate-search-domain`, `substrate-search-app`, `substrate-search-ignore` (infra) |
| run | `run` | tasks streamed into a log, workflows of them | `substrate-run-domain`, `substrate-run-app`, `substrate-run-process`, `substrate-run-toml` (infra) |
| terminal | `terminal` | pty sessions, the installed shells | `substrate-terminal-domain`, `substrate-terminal-pty` (infra) |
| language-server | `lsp` | the product's languages and their servers, spoken to over LSP | not planned yet: added to the adoption page with the split |
| debug-adapter | `dap` | the product's debug adapters, spoken to over DAP | not planned yet: added to the adoption page with the split |
| project-model | `project` | solutions and projects as an IDE models them | `substrate-project-domain`, `substrate-project-app` |
| ide-state | `state` | what the IDE remembers between runs | `substrate-state-app`, `substrate-state-json` (infra) |
| host | `process` | starting programs with the system's own environment, opening links and files | `substrate-host-app`, `substrate-host-os` (infra) |
| host-ui-contract | `substrate-host-contract`: `commands`, `events`, `bin/substrate-messages` | the command and event catalog the UI is generated from | `substrate-host-contract` (interface), `tauri-plugin-substrate` (interface) |

`sync` (`Locked`, `ReadWrite`) belongs to no context; it dissolves into the
call sites when the crates split.

## Relations

| From | To | Relation | Why |
|---|---|---|---|
| search | workspace | customer–supplier | reads files through their text view |
| source-control | workspace | customer–supplier | diffs read files through their text view |
| source-control | host | customer–supplier | `git` runs with the system's environment |
| run | host | customer–supplier | tasks run with the system's environment |
| terminal | host | customer–supplier | shells start with the system's environment |
| language-server | run | customer–supplier | a server's log is a `LogSink`; stopping it kills its process group |
| language-server | host | customer–supplier | servers start with the system's environment |
| debug-adapter | run | customer–supplier | an adapter's log is a `LogSink`; stopping it kills its process group |
| debug-adapter | host | customer–supplier | adapters start with the system's environment |
| host-ui-contract | message | published language | the UI's generated copy of every template |
| every context | message | shared kernel | what it says to the user |

`tauri-plugin-substrate` is the composition root and depends on every context.

## Rules kept from `CONTRIBUTING.md`

- Substrate stays generic: no product-specific formats or logic. Project
  formats are product adapters of `SolutionFormat`/`ProjectFormat`; product
  binary formats are product implementations of a text view.
- The wire format the UI relies on (Tauri command names and argument names,
  event names and payloads) changes only together with `ui/`.
