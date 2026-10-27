//! Find and Replace in Files, as commands.

use std::cell::Cell;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use substrate_message::Message;
use substrate_platform::workspace::{NOTHING_OPEN, Workspace};
use substrate_search_app::ports::FileWalker;
use substrate_search_app::{
    DEFAULT_MAX_LINES, ReplaceRequest, Search, files_containing, find, replace,
};
use substrate_search_domain::SearchError;
use substrate_search_domain::project::{Layout, OpenDocument, Scope};
use substrate_search_domain::query::SearchQuery;
use substrate_search_ignore::{DiskDocuments, IgnoreWalker};
use tauri::State;
use tauri::ipc::Channel;

use super::wire::entries::WalkEntryDto;
use super::wire::request::{ReplaceRequestDto, SearchQueryDto};
use super::wire::result::{FileResultDto, FindSummaryDto, ReplaceSummaryDto};
use super::{FindBatch, FindRequest, Searches};
use crate::blocking;

/// How often a running search sends what it found.
const BATCH_EVERY: Duration = Duration::from_millis(100);

/// How many files "Search within file contents" lists at most.
const MAX_CONTAINING_FILES: usize = 5_000;

/// `root` checked against the workspace — required for a project search.
fn check_root(workspace: &Workspace, root: Option<&str>, scope: Scope) -> Result<(), Message> {
    match root {
        Some(root) => Ok(workspace.check(Path::new(root))?),
        None if scope == Scope::Project => Err(Message::new(NOTHING_OPEN)),
        None => Ok(()),
    }
}

/// Find All: streams matching files over `on_batch` and resolves with the
/// totals once done or cancelled.
#[tauri::command]
pub(crate) async fn find_in_files(
    workspace: State<'_, Workspace>,
    searches: State<'_, Searches>,
    layout: State<'_, Layout>,
    documents: State<'_, DiskDocuments>,
    request: FindRequest,
    on_batch: Channel<FindBatch>,
) -> Result<FindSummaryDto, Message> {
    let scope = Scope::from(request.scope);
    check_root(&workspace, request.root.as_deref(), scope)?;
    let current = Arc::clone(&searches.current);
    current.store(request.search_id, Ordering::SeqCst);
    let layout = layout.inner().clone();
    let files = documents.inner().clone();
    blocking(move || {
        let id = request.search_id;
        // The UI dropped its end of the channel: nobody to search for.
        let listener_gone = Cell::new(false);
        let keep_going = || current.load(Ordering::SeqCst) == id && !listener_gone.get();
        let mut pending = Vec::new();
        let mut last_flush = Instant::now();
        let query = SearchQuery::from(request.query);
        let open: Vec<OpenDocument> = request.documents.into_iter().map(Into::into).collect();
        let search = Search {
            root: request.root.as_deref(),
            scope,
            query: &query,
            documents: &open,
            max_lines: request.max_lines.unwrap_or(DEFAULT_MAX_LINES),
            layout: &layout,
        };
        let summary = find(&search, (&IgnoreWalker, &files), &keep_going, &mut |file| {
            pending.push(FileResultDto::from(file));
            if last_flush.elapsed() > BATCH_EVERY {
                let batch = FindBatch {
                    files: std::mem::take(&mut pending),
                };
                listener_gone.set(on_batch.send(batch).is_err());
                last_flush = Instant::now();
            }
        });
        if !pending.is_empty() {
            // The summary below still goes back to the caller.
            on_batch.send(FindBatch { files: pending }).ok();
        }
        summary.map(FindSummaryDto::from)
    })
    .await
}

/// Stops whatever search is running.
#[tauri::command]
pub(crate) fn find_cancel(searches: State<'_, Searches>) {
    searches.current.store(0, Ordering::SeqCst);
}

/// Replace All over what a search would cover: open documents' new text
/// comes back for the UI to put in their buffers, other files are rewritten.
#[tauri::command]
pub(crate) async fn replace_in_files(
    workspace: State<'_, Workspace>,
    layout: State<'_, Layout>,
    documents: State<'_, DiskDocuments>,
    request: ReplaceRequestDto,
) -> Result<ReplaceSummaryDto, Message> {
    check_root(&workspace, request.root.as_deref(), request.scope.into())?;
    let roots = workspace.roots();
    let layout = layout.inner().clone();
    let files = documents.inner().clone();
    let request = ReplaceRequest::from(request);
    blocking(move || {
        let summary = replace(&request, &layout, (&IgnoreWalker, &files), &|path| {
            roots.check(path).map_err(Message::from)
        })?;
        Ok::<_, SearchError>(ReplaceSummaryDto::from(summary))
    })
    .await
}

/// A file explorer's search and filters: the project's files and folders,
/// the ones a lazily listed tree hasn't opened too.
#[tauri::command]
pub(crate) async fn project_items(
    workspace: State<'_, Workspace>,
    layout: State<'_, Layout>,
    root: String,
) -> Result<Vec<WalkEntryDto>, Message> {
    workspace.check(Path::new(&root))?;
    let layout = layout.inner().clone();
    blocking(move || {
        let entries = IgnoreWalker.entries(Path::new(&root), &layout);
        Ok::<_, Message>(entries.into_iter().map(WalkEntryDto::from).collect())
    })
    .await
}

/// "Search within file contents": the project files whose text matches.
/// Separate from `find_in_files`, so typing in an explorer never cancels a
/// Find in Files still running.
#[tauri::command]
pub(crate) async fn project_files_containing(
    workspace: State<'_, Workspace>,
    layout: State<'_, Layout>,
    documents: State<'_, DiskDocuments>,
    root: String,
    query: SearchQueryDto,
) -> Result<Vec<String>, Message> {
    workspace.check(Path::new(&root))?;
    let layout = layout.inner().clone();
    let files = documents.inner().clone();
    let query = SearchQuery::from(query);
    blocking(move || {
        files_containing(
            Path::new(&root),
            &query,
            MAX_CONTAINING_FILES,
            &layout,
            (&IgnoreWalker, &files),
        )
    })
    .await
}
