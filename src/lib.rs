//! Substrate Platform's remaining core, on its way into crates of their own:
//! `project` (projects and solutions) and `state` (what the IDE remembers).
//! Everything else is split into the contexts' domain, app and adapter crates
//! (see `docs/context-map.md`).

#[cfg(feature = "project")]
pub mod project;
#[cfg(feature = "state")]
pub mod state;
