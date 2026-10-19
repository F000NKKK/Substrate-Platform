//! Substrate Platform — a brand-neutral IDE backend core.
//!
//! Provides the backend pieces every IDE-style desktop tool needs
//! regardless of what it's actually editing. Always there: user-facing
//! [`message`]s, the [`events`] the UI hears, started programs' environment
//! ([`process`]) and the folders the user opened ([`workspace`]). Each
//! module behind a feature of its name: `run` (processes streamed into a
//! log, multi-step workflows), `terminal` (pty sessions, shells), `fs`
//! (folders and files, watching them), `project` (projects and solutions),
//! `git` (Source Control). Nothing here knows about any
//! specific product (no product or domain concepts, no UI framework, and no
//! concrete project standards) — that's the whole point: every IDE product
//! depends on this crate the same way, and brings its own UI (e.g. a Tauri +
//! React frontend) and its own project standards/workflow templates.

pub mod commands;
pub mod events;
pub mod message;
pub mod process;
pub mod workspace;

#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "git")]
pub mod git;
#[cfg(feature = "project")]
pub mod project;
#[cfg(feature = "run")]
pub mod run;
#[cfg(feature = "terminal")]
pub mod terminal;
