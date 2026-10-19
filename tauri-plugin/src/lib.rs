//! Substrate Platform's host side as a Tauri plugin: the commands
//! Substrate's UI calls (`plugin:substrate|<command>`) — the same in every
//! product built on it, so none writes them again. A product adds
//! `.plugin(tauri_plugin_substrate::init())` and grants `substrate:default`.
//!
//! The commands run in the [`Workspace`] the plugin manages: whatever they
//! touch must be in a folder the user opened. A product's own commands
//! check their paths against the same one ([`Workspace::check`]).

#[cfg(feature = "fs")]
mod fs;
#[cfg(feature = "git")]
mod git;
mod open;
#[cfg(feature = "search")]
mod search;
#[cfg(feature = "terminal")]
mod terminal;

#[cfg(any(feature = "git", feature = "search"))]
use substrate_platform::message::Message;
pub use substrate_platform::workspace::Workspace;
use tauri::Manager;
use tauri::plugin::{Builder, TauriPlugin};

/// The plugin's name: its commands are `plugin:substrate|<command>`.
pub use substrate_platform::commands::SUBSTRATE_PLUGIN as NAME;

/// Runs `work` off the UI thread.
#[cfg(any(feature = "git", feature = "search"))]
pub(crate) async fn blocking<T: Send + 'static>(work: impl FnOnce() -> Result<T, Message> + Send + 'static) -> Result<T, Message> {
    tauri::async_runtime::spawn_blocking(work).await.map_err(Message::verbatim)?
}

/// What a product tells the plugin about itself.
#[derive(Default)]
pub struct Options {
    /// What a search keeps out of the walk, and where generated code lives.
    #[cfg(feature = "search")]
    pub search: substrate_platform::search::Layout,
}

/// The plugin with no product options.
pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
    init_with(Options::default())
}

/// The plugin, with the commands of each feature turned on (Substrate's
/// domains, `substrate_platform::commands`, list the same ones).
pub fn init_with<R: tauri::Runtime>(options: Options) -> TauriPlugin<R> {
    Builder::new(NAME)
        .setup(move |app, _api| {
            app.manage(Workspace::default());
            #[cfg(feature = "terminal")]
            app.manage(terminal::Terminals::default());
            #[cfg(feature = "fs")]
            app.manage(fs::Watcher::default());
            #[cfg(feature = "search")]
            {
                app.manage(search::Searches::default());
                app.manage(options.search);
            }
            #[cfg(not(feature = "search"))]
            let _ = options;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open::open_url,
            open::open_path,
            open::reveal_path,
            #[cfg(feature = "search")]
            search::find_in_files,
            #[cfg(feature = "search")]
            search::find_cancel,
            #[cfg(feature = "search")]
            search::replace_in_files,
            #[cfg(feature = "search")]
            search::project_items,
            #[cfg(feature = "search")]
            search::project_files_containing,
            #[cfg(feature = "fs")]
            fs::dir_list,
            #[cfg(feature = "fs")]
            fs::dir_create_file,
            #[cfg(feature = "fs")]
            fs::dir_create_dir,
            #[cfg(feature = "fs")]
            fs::dir_rename,
            #[cfg(feature = "fs")]
            fs::dir_remove,
            #[cfg(feature = "fs")]
            fs::dir_copy,
            #[cfg(feature = "fs")]
            fs::file_read,
            #[cfg(feature = "fs")]
            fs::file_write,
            #[cfg(feature = "fs")]
            fs::allow_path,
            #[cfg(feature = "fs")]
            fs::fs_watch,
            #[cfg(feature = "terminal")]
            terminal::terminal_shells,
            #[cfg(feature = "terminal")]
            terminal::terminal_spawn,
            #[cfg(feature = "terminal")]
            terminal::terminal_write,
            #[cfg(feature = "terminal")]
            terminal::terminal_resize,
            #[cfg(feature = "terminal")]
            terminal::terminal_kill,
            #[cfg(feature = "git")]
            git::changes::git_status,
            #[cfg(feature = "git")]
            git::changes::git_stage,
            #[cfg(feature = "git")]
            git::changes::git_unstage,
            #[cfg(feature = "git")]
            git::changes::git_discard,
            #[cfg(feature = "git")]
            git::changes::git_commit,
            #[cfg(feature = "git")]
            git::changes::git_last_message,
            #[cfg(feature = "git")]
            git::changes::git_diff_texts,
            #[cfg(feature = "git")]
            git::changes::git_head_text,
            #[cfg(feature = "git")]
            git::history::git_log,
            #[cfg(feature = "git")]
            git::history::git_commit_files,
            #[cfg(feature = "git")]
            git::history::git_branches,
            #[cfg(feature = "git")]
            git::history::git_switch,
            #[cfg(feature = "git")]
            git::history::git_branch_create,
            #[cfg(feature = "git")]
            git::history::git_branch_delete,
            #[cfg(feature = "git")]
            git::remote::git_remote,
            #[cfg(feature = "git")]
            git::remote::git_integrate,
            #[cfg(feature = "git")]
            git::remote::git_operation,
            #[cfg(feature = "git")]
            git::remote::git_clone,
            #[cfg(feature = "git")]
            git::remote::git_init,
            #[cfg(feature = "git")]
            git::stash::git_stash_list,
            #[cfg(feature = "git")]
            git::stash::git_stash_push,
            #[cfg(feature = "git")]
            git::stash::git_stash_op,
            #[cfg(feature = "git")]
            git::stash::git_config_get,
            #[cfg(feature = "git")]
            git::stash::git_config_set,
        ])
        .build()
}
