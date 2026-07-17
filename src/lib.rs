//! Substrate Platform — a brand-neutral IDE backend core.
//!
//! Provides the backend pieces every IDE-style desktop tool needs
//! regardless of what it's actually editing: a generic external-process
//! runner ([`task`]), an append-only log sink ([`log`]), and a real
//! cross-platform PTY session ([`pty`]). Nothing here knows about any
//! specific product (no Minecraft/Yog concepts, no UI framework) — that's
//! the whole point: a product like Yog-IDLE depends on this crate the same
//! way any future, unrelated IDE product would, and brings its own UI
//! (Yog-IDLE's is a Tauri + React frontend).

pub mod log;
pub mod pty;
pub mod shell;
pub mod task;

pub use log::{Level, LogLine, LogSink};
pub use pty::PtySession;
pub use shell::{detect_shells, ShellInfo};
pub use task::{spawn as spawn_task, Task, TaskHandle};
