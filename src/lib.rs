//! Substrate Platform — a brand-neutral IDE backend core.
//!
//! Provides the backend pieces every IDE-style desktop tool needs
//! regardless of what it's actually editing: a generic external-process
//! runner ([`task`]) and named multi-step build/run flows on top of it
//! ([`workflow`]), an append-only log sink ([`log`]), a real cross-platform
//! PTY session ([`pty`]), directory browsing/mutation ([`dir`]) and watching ([`watch`]), and a
//! project/solution model ([`project`]). Nothing here knows about any
//! specific product (no product or domain concepts, no UI framework, and no
//! concrete project standards) — that's the whole point: every IDE product
//! depends on this crate the same way, and brings its own UI (e.g. a Tauri +
//! React frontend) and its own project standards/workflow templates.

pub mod dir;
pub mod log;
pub mod process;
pub mod project;
pub mod pty;
pub mod shell;
pub mod task;
pub mod watch;
pub mod workflow;

pub use dir::{DirEntryInfo, EntryKind};
pub use log::{Level, LogLine, LogSink};
pub use project::{Project, ProjectStandard, Solution};
pub use pty::{PtyOptions, PtySession, Utf8Chunker};
pub use shell::{ShellInfo, detect_shells};
pub use watch::DirWatcher;
pub use task::{Task, TaskExit, TaskHandle, spawn as spawn_task};
pub use workflow::{
    WorkflowDef, WorkflowFile, WorkflowHandle, WorkflowResult, WorkflowStep, run as run_workflow,
};
