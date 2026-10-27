//! Substrate Platform — a brand-neutral IDE backend core.
//!
//! Provides the backend pieces every IDE-style desktop tool needs
//! regardless of what it's actually editing. Always there: the folders the user
//! opened ([`workspace`]). Starting programs, running tools and their
//! workflows are their own crates (`substrate-process`, `substrate-run-*`).
//! Each module behind a feature of its name: `terminal` (pty sessions, shells), `fs`
//! (folders and files, watching them), `project` (projects and solutions),
//! `lsp` (language servers), `dap` (debug
//! adapters), `state` (what the IDE remembers). Nothing
//! here knows about any specific product (no product or domain concepts, no
//! UI framework, and no concrete project standards) — that's the whole point: every IDE product
//! depends on this crate the same way, and brings its own UI (e.g. a Tauri +
//! React frontend) and its own project standards/workflow templates.

pub mod sync;
pub mod text_view;
pub mod workspace;

#[cfg(any(feature = "dap", feature = "lsp"))]
mod framing;

#[cfg(feature = "dap")]
pub mod dap;
#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "lsp")]
pub mod lsp;
#[cfg(feature = "project")]
pub mod project;
#[cfg(feature = "state")]
pub mod state;
#[cfg(feature = "terminal")]
pub mod terminal;
