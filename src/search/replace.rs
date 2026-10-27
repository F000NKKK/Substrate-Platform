//! Replace in Files: every match replaced, in open documents' buffers and on disk.

use std::path::Path;

use super::coverage::Coverage;
use super::plan::Plan;
use super::source::Source;
use super::{
    Decoding, FileMask, Layout, Matcher, ReplaceRequest, ReplaceSummary, ReplacedDocument,
    ReplacedFile, ReplacementMode, SearchError, read_text,
};
use crate::message::Message;
use crate::text_view::TextViews;

/// Replace All over the same set of files a search would cover: open
/// documents' new text comes back for the frontend to apply to their
/// buffers, every other file is rewritten on disk (non-UTF-8 files are left
/// alone). `may_write` says whether a file may be written; a file it refuses,
/// or that can't be written, is listed in the summary's errors.
///
/// # Errors
///
/// What [`Matcher::new`] and [`FileMask::parse`] refuse in the query.
pub fn replace(
    request: &ReplaceRequest,
    layout: &Layout,
    views: &TextViews,
    may_write: &dyn Fn(&Path) -> Result<(), Message>,
) -> Result<ReplaceSummary, SearchError> {
    let replacer = Replacer {
        matcher: Matcher::new(&request.query)?,
        replacement: &request.replacement,
        views,
        mode: if request.query.regex {
            ReplacementMode::Expand
        } else {
            ReplacementMode::Literal
        },
    };
    let mask = FileMask::parse(&request.query.file_mask)?;
    let coverage = Coverage {
        root: request.root.as_deref(),
        scope: request.scope,
        documents: &request.documents,
        layout,
    };
    // Generated code is rewritten by the next build — never replaced in.
    let plan = Plan::new(&coverage, &mask, false);
    let mut summary = ReplaceSummary::default();
    for (path, source) in &plan.items {
        match source {
            Source::Buffer(text) => replacer.in_buffer(path, text, &mut summary),
            Source::Disk => replacer.on_disk(path, may_write, &mut summary),
        }
    }
    Ok(summary)
}

/// A replace's matcher and replacement.
struct Replacer<'a> {
    matcher: Matcher,
    replacement: &'a str,
    mode: ReplacementMode,
    views: &'a TextViews,
}

impl Replacer<'_> {
    /// Replaces in an open document's buffer: its new text goes to `summary`.
    fn in_buffer(&self, path: &Path, text: &str, summary: &mut ReplaceSummary) {
        let (text, count) = self.matcher.replace_all(text, self.replacement, self.mode);
        if count == 0 {
            return;
        }
        summary.total += count;
        summary.documents.push(ReplacedDocument {
            path: path.to_string_lossy().into_owned(),
            text,
            count,
        });
    }

    /// Replaces in a file on disk and rewrites it.
    fn on_disk(
        &self,
        path: &Path,
        may_write: &dyn Fn(&Path) -> Result<(), Message>,
        summary: &mut ReplaceSummary,
    ) {
        let Some(text) = read_text(path, Decoding::Strict, self.views) else {
            return;
        };
        let (text, count) = self.matcher.replace_all(&text, self.replacement, self.mode);
        if count == 0 {
            return;
        }
        let written = may_write(path)
            .and_then(|()| write_text(path, &text, self.views).map_err(Message::from));
        match written {
            Ok(()) => {
                summary.total += count;
                summary.files.push(ReplacedFile {
                    path: path.to_string_lossy().into_owned(),
                    count,
                });
            }
            Err(err) => summary.errors.push(err),
        }
    }
}

/// Writes `text` to `path`, through its text view if its kind has one.
fn write_text(path: &Path, text: &str, views: &TextViews) -> Result<(), SearchError> {
    match views.find(path) {
        Some(view) => (view.write)(path, text),
        None => std::fs::write(path, text),
    }
    .map_err(|source| SearchError::WriteFailed {
        path: path.to_path_buf(),
        source,
    })
}
