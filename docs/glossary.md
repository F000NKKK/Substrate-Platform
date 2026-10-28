# Glossary

The ubiquitous language of Substrate-Platform, per bounded context (Craftbook
[§7.1](https://github.com/F000NKKK/craftbook/blob/main/rust/07-architecture-ddd.md#71-ubiquitous-language)).
The code uses these words and the glossary uses the code's; renaming one
renames the other in the same change. Contexts are listed in
[context-map.md](context-map.md).

## Message (shared kernel)

| Term | Meaning | In code |
|---|---|---|
| Message | Text for the user: an English template with `{name}` placeholders, and their values. The template is the key the UI's translation catalogs use. | `message::Message` |
| Template | One message's English text, declared once in its module's `messages!` block. | `messages!`, `*::messages::*` |
| Verbatim | Text that is not ours to translate: a tool's own output. | `Message::verbatim` |
| Error | Why an operation failed, as a typed enum per family; the interface turns it into a `Message` once, on the way out. | `substrate_git_domain::GitError`, `substrate_search_domain::SearchError`, `substrate_lsp_domain::LspError`, `substrate_dap_domain::DapError`, `substrate_workspace_fs::FsError`, `substrate_state_json::StateError`, `substrate_process::ProcessError`, `substrate_project_domain::ProjectError`, `substrate_workspace_domain::WorkspaceError`, `substrate_terminal_domain::TerminalError`, `substrate_run_toml::FileError` |

## Workspace

| Term | Meaning | In code |
|---|---|---|
| Workspace | The folders an IDE may touch: the projects the user opened and files they picked. | `substrate_workspace_app::Workspace` |
| Roots | The workspace's folders at one moment, to check paths with on another thread. | `substrate_workspace_domain::Roots` |
| Current project | The project shown now; new terminals start in it. | `Workspace::current` |
| Directory entry | One file or folder of a listed directory level. | `substrate_workspace_domain::DirectoryEntry` |
| Entry kind | File or directory. | `substrate_workspace_domain::EntryKind` |
| Directory watcher | Watches named folders and delivers changed paths in batches. | `substrate_workspace_fs::DirectoryWatcher` |
| Quiet period | How long a burst of changes may stay quiet before its batch is delivered. | `substrate_workspace_fs::DEFAULT_QUIET` |
| Text view | How a product's binary format is read as text and written from it. | `substrate_workspace_domain::TextView` |
| Text views | A product's text views by extension, built at startup and handed to what reads files as text. | `substrate_workspace_domain::TextViews` |

## Source control

| Term | Meaning | In code |
|---|---|---|
| Repository status | A repository's state for Git Changes: branch, head, upstream, changes, operation. | `substrate_git_domain::status::RepoStatus`, `WorkingTree::status` |
| File change | One changed file: path, original path, kind. | `substrate_git_domain::status::FileChange` |
| Change kind | Modified, added, deleted, renamed, copied, type changed, untracked, conflicted. | `substrate_git_domain::status::ChangeKind` |
| Operation | A merge, rebase, cherry-pick or revert waiting to be finished or aborted. | `substrate_git_domain::status::Operation` |
| Discard | Undo Changes: back to what is staged, or to the last commit. | `substrate_git_app::changes::discard` |
| Commit mode, scope | A new commit or an amend; what is staged or every change. | `substrate_git_domain::commits::{CommitMode, CommitScope}` |
| Resolution | Continue or abort the operation in progress. | `substrate_git_domain::remotes::Resolution`, `substrate_git_app::syncing::resolve` |
| Remote operation | Fetch, pull, push, sync. | `substrate_git_domain::remotes::RemoteOperation`, `substrate_git_app::syncing::synchronize` |
| Integration | Merge a branch into the current one, or rebase onto it. | `substrate_git_domain::remotes::Integration` |
| Branch | A local or remote-tracking branch and its tracking state. | `substrate_git_domain::branches::Branch` |
| Branch source, checkout, delete mode | Switch to a local or remote branch; check a new one out or not; delete only merged or force. | `substrate_git_domain::branches::{BranchSource, BranchCheckout, DeleteMode}` |
| Commit | One commit of a history page. | `substrate_git_domain::commits::Commit` |
| Stash entry | One stash of the stash list. | `substrate_git_domain::stashes::StashEntry` |
| Diff side | One side of a file's diff: a revision, the index or the working tree. | `substrate_git_domain::diffs::DiffSide` |
| Diff texts | Both sides of a file's diff, as text. | `substrate_git_domain::diffs::DiffTexts` |
| Output | Where a long run's lines go (the host's Output window). | `substrate_git_app::Output` |
| Write lock | Held by every write to a repository, so two never race for `index.lock`; one per host, passed to each write. | `substrate_git_app::WriteLock` |

## Search

| Term | Meaning | In code |
|---|---|---|
| Search query | What to look for: pattern, case sensitivity, word match, pattern syntax, file mask, generated code (each an enum, not a flag). | `substrate_search_domain::query::SearchQuery` |
| Matcher | A compiled search query. | `substrate_search_domain::pattern::Matcher` |
| Replacement mode | A replacement taken literally or with `$1` expanded. | `substrate_search_domain::pattern::ReplacementMode` |
| Decoding | Invalid UTF-8 replaced (search) or refused (replace). | `substrate_search_app::ports::Decoding` |
| Walk entry | A file or folder the project walk keeps. | `substrate_search_domain::project::WalkEntry` |
| Find, replace | Find in Files and Replace in Files. | `substrate_search_app::{find, replace}` |
| File mask | Which files a search covers: `*.rs;*.toml`, `!*.lock`. | `substrate_search_domain::pattern::FileMask` |
| Layout | What a product keeps out of the walk and where its generated code lives. | `substrate_search_domain::project::Layout` |
| Scope | Open documents, or the whole project. | `substrate_search_domain::project::Scope` |
| Open document | An editor's buffer, searched instead of its file on disk. | `substrate_search_domain::project::OpenDocument` |
| Line hit | One matching line as Find Results show it. | `substrate_search_domain::results::LineHit` |
| Find summary | The totals of one search. | `substrate_search_domain::results::FindSummary` |
| Replace summary | What Replace All changed. | `substrate_search_domain::results::ReplaceSummary` |

## Run

| Term | Meaning | In code |
|---|---|---|
| Task | One external command, its output streamed into a log. | `substrate_run_domain::Task` |
| Task exit | How a task ended. | `substrate_run_domain::TaskExit` |
| Workflow | Named steps run one after another, stopping at the first failure. | `substrate_run_app::workflow::run`, `substrate_run_app::workflow::Handle` |
| Workflow file | One `workflow.toml`: named workflows, each a definition of steps. | `substrate_run_domain::workflow::File`, `substrate_run_domain::workflow::Definition` |
| Step | Runs one command, or composes another workflow. | `substrate_run_domain::workflow::Step` |
| Outcome | How a workflow run ended: succeeded, failed, cancelled. | `substrate_run_domain::workflow::Outcome` |
| Log sink | An append-only log any feature pushes lines into. | `substrate_run_app::LogSink` |
| Log line, level | One line of a log and its severity (info, warn, error). | `substrate_run_domain::LogLine`, `substrate_run_domain::Level` |

## Terminal

| Term | Meaning | In code |
|---|---|---|
| Pty session | A shell running on a real OS pseudo-terminal. | `substrate_terminal_app::ports::Session` |
| Terminal size | A terminal's columns and rows. | `substrate_terminal_domain::TerminalSize` |
| Shell | One installed shell a terminal can start. | `substrate_terminal_domain::Shell` |
| UTF-8 chunker | Turns arbitrarily split bytes into whole characters. | `substrate_terminal_domain::Utf8Chunker` |

## Language server

| Term | Meaning | In code |
|---|---|---|
| Language | The files that are in it and its server, if any. | `substrate_lsp_domain::Language` |
| Languages | A product's languages, from its TOML list. | `substrate_lsp_domain::Languages` |
| Server definition | How a language server is started. | `substrate_lsp_domain::ServerDefinition` |
| Client | A connection to one running language server. | `substrate_lsp_app::Client` |
| Notification | What the server said without being asked. | `substrate_lsp_domain::Notification` |

## Debug adapter

| Term | Meaning | In code |
|---|---|---|
| Adapter | One debug adapter of the product's list: the languages it debugs and how it is started. | `substrate_dap_domain::Adapter` |
| Adapters | A product's debug adapters, from its TOML list. | `substrate_dap_domain::Adapters` |
| Adapter definition | How a debug adapter is started and what a launch starts from. | `substrate_dap_domain::AdapterDefinition` |
| Client | A connection to one running debug adapter. | `substrate_dap_app::Client` |
| Command | What a request to the adapter asks for (`launch`, `stackTrace`…). | `substrate_dap_domain::Command` |
| Event | What the adapter said without being asked. | `substrate_dap_domain::Event` |
| Seq | The sequence number of a message; a response names its request by it. | `substrate_dap_domain::Seq` |
| Debuggee | The program being debugged; `disconnect` ends it or leaves it running. | `substrate_dap_domain::Debuggee` |

## Project model

| Term | Meaning | In code |
|---|---|---|
| Solution | Projects and how they are built together. | `substrate_project_domain::Solution` |
| Project | A project's contents as its project file describes them. | `substrate_project_domain::Project` |
| Project entry | A project as its solution lists it. | `substrate_project_domain::ProjectEntry` |
| Solution folder | A folder of the solution's tree that is not one on disk. | `substrate_project_domain::SolutionFolder` |
| Configuration | A build configuration on a platform: `Debug|Any CPU`. | `substrate_project_domain::Configuration` |
| Section | Settings a format keeps beyond the model, in order. | `substrate_project_domain::Section` |
| Format | How a solution or project file is read and written; the product's. | `substrate_project_app::ports::SolutionFormat`, `substrate_project_app::ports::ProjectFormat` |
| Project standard | A kind of project, told by marker files. | `substrate_project_domain::ProjectStandard` |

## Installer

| Term | Meaning | In code |
|---|---|---|
| Catalog | What can be installed, checked when made: ids unique in one namespace, every reference names something there, no requirement cycle. A reader (an adapter) fills `CatalogParts`. | `substrate_installer_domain::channel::{Catalog, CatalogParts, CatalogError}` |
| Product | What the installer is for, installed as a whole, with a program that starts it and the workloads it offers. | `substrate_installer_domain::channel::Product` |
| Installer entry | The installer itself as a product of its own channel, updated first. | `substrate_installer_domain::channel::Installer` |
| Workload | A named set of components: required (always), recommended (checked, can be unchecked), optional (unchecked). | `substrate_installer_domain::channel::Workload` |
| Component | One installable part with its group, what it requires and its source. | `substrate_installer_domain::channel::Component` |
| Group | A heading individual components are listed under. | `substrate_installer_domain::channel::Group` |
| Item | What an id names in a catalog: a component, a product or the installer entry. | `substrate_installer_domain::channel::Item`, `substrate_installer_domain::ids::ItemId` |
| Source | Where a component is published, and so how it is installed: GitHub release, git tag, crates.io, rustup toolchain, rustup targets. A closed, `non_exhaustive` enum. | `substrate_installer_domain::sources::Source` |
| System package | A package on the system's own libraries, taken instead of the bundled one where the host has them. | `substrate_installer_domain::sources::SystemPackage` |
| Release | One published version of a source, with its assets. | `substrate_installer_domain::releases::{Release, Asset, Sha256}` |
| Platform tag | An operating system and an architecture, `linux-x86_64`. | `substrate_installer_domain::platforms::PlatformTag` |
| Host | The machine planning is for: its platform and system libraries. | `substrate_installer_domain::platforms::Host` |
| Version | A version, compared leniently (semver, missing parts filled, a leading `v` dropped); not semver is older and ordered as text. | `substrate_installer_domain::versions::Version` |
| Version requirement | A condition on a version in semver's words. | `substrate_installer_domain::versions::VersionRequirement` |
| Selection | The Modify window's checkboxes: workloads, unchecked recommended parts, checked optional and individual components. | `substrate_installer_domain::choice::Selection` |
| Why | Why a component is in a selection (required, recommended, chosen, required by another); decides if its checkbox can be cleared. | `substrate_installer_domain::choice::Why` |
| Change | One thing a plan does: install, update or remove. | `substrate_installer_domain::changes::Change` |
| Plan | The changes, a component after what it requires, and the release each takes. | `substrate_installer_domain::planning::Plan` |
| Mode | When downloads happen: install while downloading, or download all then install. | `substrate_installer_domain::planning::Mode` |
| Managed, system | Who put a found thing there: the installer (it may update and remove it) or someone else (it counts as installed and is left alone). | `substrate_installer_domain::inventory::{Origin, Found}` |
| Installed record | What the installer installed: item, version, path; the set is read and written whole. | `substrate_installer_domain::inventory::{Installed, InstalledSet}` |
| Needs, conflict | What a release needs of the other components, and a need the machine will not meet. | `substrate_installer_domain::compat::{Needs, Conflict, check}` |
| Manifest | The release asset that says its needs; its name is the catalog's. | `substrate_installer_domain::channel::Catalog::manifest` |
| Installer event | One thing that happened in a plan, for a progress display; and how the plan ended. | `substrate_installer_domain::events::{Event, Outcome}` |
| Available | The newest release a host can install of each item, and the sources that failed. | `substrate_installer_app::listing::Available` |
| Situation | What is installed and what is published, which a plan is made against. | `substrate_installer_app::plans::Situation` |
| Installer (use case) | Applies a plan to a catalog on a host through its adapters. | `substrate_installer_app::installing::Installer` |
| Cancel token | Pauses, resumes and cancels an operation from another thread. | `substrate_installer_app::control::CancelToken` |
| Installer ports | What the installer needs from outside: list releases, download, unpack, keep records, run tools, lay out and move files, install programs, detect the system. | `substrate_installer_app::ports::*` |

## IDE state

| Term | Meaning | In code |
|---|---|---|
| Workspace state | A project's remembered state, in the product's private folder of it. | `substrate_state_json::WORKSPACE_FILE` |
| App state | What the app remembers outside any project. | `substrate_state_json::APP_STATE_FILE` |
| Private folder | A product's machine-local folder in a project, kept out of version control. | `substrate_state_json::private_dir` |

## Host

| Term | Meaning | In code |
|---|---|---|
| Host environment | How a started program's environment differs from this process's (an AppImage's variables removed). | `substrate_process::host_environment` |
| Command domain | A group of host commands under a plugin. | `commands::Domain` |
| Event | A message the host sends its UI, by name. | `events` |
| Port | A trait a context's app crate declares and an adapter crate implements. | `substrate_git_app::ports::{WorkingTree, History, Diffs, Branches, Stashes, Remotes, Settings}` |
| Adapter | An infrastructure crate that implements ports with an outside tool. | `substrate_git_gitcli::GitCli` |
| DTO | A `serde` struct at the interface that mirrors a domain type for the wire; the name ends in `Dto`. | `RepoStatusDto`, `DiscardItemDto` |
