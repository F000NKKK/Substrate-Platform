//! Substrate Platform — a brand-neutral IDE backend core.
//!
//! Provides the backend pieces every IDE-style desktop tool needs
//! regardless of what it's actually editing: a generic external-process
//! runner ([`task`]) and named multi-step build/run flows on top of it
//! ([`workflow`]), an append-only log sink ([`log`]), a real cross-platform
//! PTY session ([`pty`]), directory browsing/mutation ([`dir`]), and a
//! project/solution model ([`project`]). Nothing here knows about any
//! specific product (no Minecraft/Yog concepts, no UI framework, and no
//! concrete project standards) — that's the whole point: a product like
//! Yog-IDLE depends on this crate the same way any future, unrelated IDE
//! product would, and brings its own UI (Yog-IDLE's is a Tauri + React
//! frontend) and its own project standards/workflow templates.

pub mod dir;
pub mod log;
pub mod project;
pub mod pty;
pub mod shell;
pub mod task;
pub mod workflow;

pub use dir::{DirEntryInfo, EntryKind};
pub use log::{Level, LogLine, LogSink};
pub use project::{Project, ProjectStandard, Solution};
pub use pty::PtySession;
pub use shell::{detect_shells, ShellInfo};
pub use task::{spawn as spawn_task, Task, TaskHandle};
pub use workflow::{run as run_workflow, WorkflowDef, WorkflowFile, WorkflowHandle, WorkflowStep};
