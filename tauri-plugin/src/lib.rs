//! Substrate Platform's host side as a Tauri plugin: the commands
//! Substrate's UI calls (`plugin:substrate|<command>`) — the same in every
//! product built on it, so none writes them again. A product adds
//! `.plugin(tauri_plugin_substrate::init())` and grants `substrate:default`.
//!
//! The commands run in the [`Workspace`] the plugin manages: whatever they
//! touch must be in a folder the user opened. A product's own commands
//! check their paths against the same one ([`Workspace::check`]).

mod git;

pub use substrate_platform::workspace::Workspace;
use tauri::Manager;
use tauri::plugin::{Builder, TauriPlugin};

/// The plugin's name: its commands are `plugin:substrate|<command>`.
pub use substrate_platform::events::PLUGIN as NAME;

/// Runs `work` off the UI thread.
pub(crate) async fn blocking<T: Send + 'static>(work: impl FnOnce() -> Result<T, substrate_platform::message::Message> + Send + 'static) -> Result<T, substrate_platform::message::Message> {
    tauri::async_runtime::spawn_blocking(work).await.map_err(|e| substrate_platform::message::Message::verbatim(e))?
}

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
    Builder::new(NAME)
        .setup(|app, _api| {
            app.manage(Workspace::default());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            git::git_status,
            git::git_stage,
            git::git_unstage,
            git::git_discard,
            git::git_commit,
            git::git_last_message,
            git::git_diff_texts,
            git::git_head_text,
            git::git_log,
            git::git_commit_files,
            git::git_branches,
            git::git_switch,
            git::git_branch_create,
            git::git_branch_delete,
            git::git_remote,
            git::git_integrate,
            git::git_operation,
            git::git_clone,
            git::git_init,
            git::git_stash_list,
            git::git_stash_push,
            git::git_stash_op,
            git::git_config_get,
            git::git_config_set,
        ])
        .build()
}
