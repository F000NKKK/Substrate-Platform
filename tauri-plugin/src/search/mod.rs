//! Find and Replace in Files: a search streams its files to the UI over a
//! channel; a newer one (or `find_cancel`) stops it. What the product keeps
//! out of the walk is the [`Layout`] it gives the plugin. Feature `search`.

use crate::blocking;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use substrate_platform::message::Message;
use substrate_platform::search::{
    DEFAULT_MAX_LINES, FileResult, FindSummary, Layout, OpenDocument, ReplaceRequest,
    ReplaceSummary, Scope, Search, SearchQuery, WalkEntry, files_containing, find, project_entries,
    replace,
};
use substrate_platform::workspace::{NOTHING_OPEN, Workspace};
use tauri::State;
use tauri::ipc::Channel;

/// How often a running search sends what it found.
const BATCH_EVERY: Duration = Duration::from_millis(100);

/// How many files "Search within file contents" lists at most.
const MAX_CONTAINING_FILES: usize = 5_000;

/// The id of the search allowed to keep running — a new search replaces it,
/// `find_cancel` clears it.
#[derive(Default)]
pub struct Searches {
    current: Arc<AtomicU64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindRequest {
    search_id: u64,
    /// The project root — required for `Project` scope, used for relative paths otherwise.
    root: Option<String>,
    scope: Scope,
    query: SearchQuery,
    #[serde(default)]
    documents: Vec<OpenDocument>,
    max_lines: Option<usize>,
}

/// Files found since the last batch.
#[derive(Clone, Serialize)]
pub struct FindBatch {
    files: Vec<FileResult>,
}

/// `root` checked against the workspace — required for a project search.
fn check_root(workspace: &Workspace, root: Option<&str>, scope: Scope) -> Result<(), Message> {
    match root {
        Some(root) => workspace.check(Path::new(root)),
        None if scope == Scope::Project => Err(Message::new(NOTHING_OPEN)),
        None => Ok(()),
    }
}

/// Find All: streams matching files over `on_batch` and resolves with the
/// totals once done or cancelled.
#[tauri::command]
pub async fn find_in_files(
    workspace: State<'_, Workspace>,
    searches: State<'_, Searches>,
    layout: State<'_, Layout>,
    request: FindRequest,
    on_batch: Channel<FindBatch>,
) -> Result<FindSummary, Message> {
    check_root(&workspace, request.root.as_deref(), request.scope)?;
    let current = Arc::clone(&searches.current);
    current.store(request.search_id, Ordering::SeqCst);
    let layout = layout.inner().clone();
    blocking(move || {
        let id = request.search_id;
        let keep_going = || current.load(Ordering::SeqCst) == id;
        let mut pending = Vec::new();
        let mut last_flush = Instant::now();
        let search = Search {
            root: request.root.as_deref(),
            scope: request.scope,
            query: &request.query,
            documents: &request.documents,
            max_lines: request.max_lines.unwrap_or(DEFAULT_MAX_LINES),
            layout: &layout,
        };
        let summary = find(&search, &keep_going, &mut |file| {
            pending.push(file);
            if last_flush.elapsed() > BATCH_EVERY {
                let _ = on_batch.send(FindBatch {
                    files: std::mem::take(&mut pending),
                });
                last_flush = Instant::now();
            }
        });
        if !pending.is_empty() {
            let _ = on_batch.send(FindBatch { files: pending });
        }
        summary
    })
    .await
}

/// Stops whatever search is running.
#[tauri::command]
pub fn find_cancel(searches: State<'_, Searches>) {
    searches.current.store(0, Ordering::SeqCst);
}

/// Replace All over what a search would cover: open documents' new text
/// comes back for the UI to put in their buffers, other files are rewritten.
#[tauri::command]
pub async fn replace_in_files(
    workspace: State<'_, Workspace>,
    layout: State<'_, Layout>,
    request: ReplaceRequest,
) -> Result<ReplaceSummary, Message> {
    check_root(&workspace, request.root.as_deref(), request.scope)?;
    let roots = workspace.roots();
    let layout = layout.inner().clone();
    blocking(move || replace(&request, &layout, &|path| roots.check(path))).await
}

/// A file explorer's search and filters: the project's files and folders,
/// the ones a lazily listed tree hasn't opened too.
#[tauri::command]
pub async fn project_items(
    workspace: State<'_, Workspace>,
    layout: State<'_, Layout>,
    root: String,
) -> Result<Vec<WalkEntry>, Message> {
    workspace.check(Path::new(&root))?;
    let layout = layout.inner().clone();
    blocking(move || Ok(project_entries(Path::new(&root), &layout))).await
}

/// "Search within file contents": the project files whose text matches.
/// Separate from `find_in_files`, so typing in an explorer never cancels a
/// Find in Files still running.
#[tauri::command]
pub async fn project_files_containing(
    workspace: State<'_, Workspace>,
    layout: State<'_, Layout>,
    root: String,
    query: SearchQuery,
) -> Result<Vec<String>, Message> {
    workspace.check(Path::new(&root))?;
    let layout = layout.inner().clone();
    blocking(move || files_containing(Path::new(&root), &query, MAX_CONTAINING_FILES, &layout))
        .await
}
