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

## Workspace

| Term | Meaning | In code |
|---|---|---|
| Workspace | The folders an IDE may touch: the projects the user opened and files they picked. | `workspace::Workspace` |
| Roots | The workspace's folders at one moment, to check paths with on another thread. | `workspace::Roots` |
| Current project | The project shown now; new terminals start in it. | `Workspace::current` |
| Directory entry | One file or folder of a listed directory level. | `fs::DirectoryEntry` |
| Entry kind | File or directory. | `fs::EntryKind` |
| Directory watcher | Watches named folders and delivers changed paths in batches. | `fs::DirectoryWatcher` |
| Quiet period | How long a burst of changes may stay quiet before its batch is delivered. | `fs::watch::DEFAULT_QUIET` |
| Text view | How a product's binary format is read as text and written from it, by extension. | `text_view::TextView` |

## Source control

| Term | Meaning | In code |
|---|---|---|
| Repository status | A repository's state for Git Changes: branch, head, upstream, changes, operation. | `git::status::RepoStatus`, `git::status::read` |
| File change | One changed file: path, original path, kind. | `git::status::FileChange` |
| Change kind | Modified, added, deleted, renamed, copied, type changed, untracked, conflicted. | `git::status::ChangeKind` |
| Operation | A merge, rebase, cherry-pick or revert waiting to be finished or aborted. | `git::status::Operation` |
| Discard | Undo Changes: back to what is staged, or to the last commit. | `git::status::discard` |
| Commit mode, scope | A new commit or an amend; what is staged or every change. | `git::status::CommitMode`, `git::status::CommitScope` |
| Resolution | Continue or abort the operation in progress. | `git::remote::Resolution`, `git::remote::resolve` |
| Remote operation | Fetch, pull, push, sync. | `git::remote::RemoteOperation`, `git::remote::synchronize` |
| Integration | Merge a branch into the current one, or rebase onto it. | `git::remote::Integration` |
| Branch | A local or remote-tracking branch and its tracking state. | `git::branch::Branch` |
| Branch source, checkout, delete mode | Switch to a local or remote branch; check a new one out or not; delete only merged or force. | `git::branch::{BranchSource, BranchCheckout, DeleteMode}` |
| Commit | One commit of a history page. | `git::history::Commit` |
| Stash entry | One stash of the stash list. | `git::stash::StashEntry` |
| Diff side | One side of a file's diff: a revision, the index or the working tree. | `git::diff::DiffSide` |
| Diff texts | Both sides of a file's diff, as text. | `git::diff::DiffTexts` |
| Output | Where a long run's lines go (the host's Output window). | `git::Output` |

## Search

| Term | Meaning | In code |
|---|---|---|
| Search query | What to look for: pattern, match case, whole word, regex, file mask. | `search::SearchQuery` |
| Matcher | A compiled search query. | `search::Matcher` |
| Replacement mode | A replacement taken literally or with `$1` expanded. | `search::ReplacementMode` |
| Decoding | Invalid UTF-8 replaced (search) or refused (replace). | `search::Decoding` |
| Walk entry | A file or folder the project walk keeps. | `search::WalkEntry` |
| Find, replace | Find in Files and Replace in Files. | `search::find`, `search::replace` |
| File mask | Which files a search covers: `*.rs;*.toml`, `!*.lock`. | `search::FileMask` |
| Layout | What a product keeps out of the walk and where its generated code lives. | `search::Layout` |
| Scope | Open documents, or the whole project. | `search::Scope` |
| Open document | An editor's buffer, searched instead of its file on disk. | `search::OpenDocument` |
| Line hit | One matching line as Find Results show it. | `search::LineHit` |
| Find summary | The totals of one search. | `search::FindSummary` |
| Replace summary | What Replace All changed. | `search::ReplaceSummary` |

## Run

| Term | Meaning | In code |
|---|---|---|
| Task | One external command, its output streamed into a log. | `run::Task` |
| Task exit | How a task ended. | `run::TaskExit` |
| Workflow | Named steps run one after another, stopping at the first failure. | `run::workflow::run`, `run::workflow::Handle` |
| Workflow file | One `workflow.toml`: named workflows, each a definition of steps. | `run::workflow::File`, `run::workflow::Definition` |
| Step | Runs one command, or composes another workflow. | `run::workflow::Step` |
| Outcome | How a workflow run ended: succeeded, failed, cancelled. | `run::workflow::Outcome` |
| Log sink | An append-only log any feature pushes lines into. | `run::LogSink` |
| Log line, level | One line of a log and its severity (info, warn, error). | `run::LogLine`, `run::Level` |

## Terminal

| Term | Meaning | In code |
|---|---|---|
| Pty session | A shell running on a real OS pseudo-terminal. | `terminal::PtySession` |
| Shell | One installed shell a terminal can start. | `terminal::Shell` |
| UTF-8 chunker | Turns arbitrarily split bytes into whole characters. | `terminal::Utf8Chunker` |

## Language server

| Term | Meaning | In code |
|---|---|---|
| Language | The files that are in it and its server, if any. | `lsp::Language` |
| Languages | A product's languages, from its TOML list. | `lsp::Languages` |
| Server definition | How a language server is started. | `lsp::ServerDefinition` |
| Client | A connection to one running language server. | `lsp::Client` |
| Notification | What the server said without being asked. | `lsp::Notification` |

## Project model

| Term | Meaning | In code |
|---|---|---|
| Solution | Projects and how they are built together. | `project::Solution` |
| Project | A project's contents as its project file describes them. | `project::Project` |
| Project entry | A project as its solution lists it. | `project::ProjectEntry` |
| Solution folder | A folder of the solution's tree that is not one on disk. | `project::SolutionFolder` |
| Configuration | A build configuration on a platform: `Debug|Any CPU`. | `project::Configuration` |
| Section | Settings a format keeps beyond the model, in order. | `project::Section` |
| Format | How a solution or project file is read and written; the product's. | `project::SolutionFormat`, `project::ProjectFormat` |
| Project standard | A kind of project, told by marker files. | `project::ProjectStandard` |

## IDE state

| Term | Meaning | In code |
|---|---|---|
| Workspace state | A project's remembered state, in the product's private folder of it. | `state::WORKSPACE_FILE` |
| App state | What the app remembers outside any project. | `state::APP_STATE_FILE` |
| Private folder | A product's machine-local folder in a project, kept out of version control. | `state::private_dir` |

## Host

| Term | Meaning | In code |
|---|---|---|
| Host environment | How a started program's environment differs from this process's (an AppImage's variables removed). | `process::host_environment` |
| Command domain | A group of host commands under a plugin. | `commands::Domain` |
| Event | A message the host sends its UI, by name. | `events` |
