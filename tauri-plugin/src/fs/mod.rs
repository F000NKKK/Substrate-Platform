//! Folders and files as an IDE browses and edits them. Writes stay inside the
//! [`Workspace`](substrate_workspace_app::Workspace); reads may go anywhere (a
//! debugger frame in a dependency, a file the user picked). Changes made
//! outside the IDE come back as
//! [`events::FILES_CHANGED`](substrate_host_contract::events::FILES_CHANGED).
//! Feature `fs`.

pub(crate) mod commands;
mod entry;
mod entry_kind_dto;
mod watcher;

pub(crate) use watcher::Watcher;
