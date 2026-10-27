//! Replace in Files: every match replaced, in open documents' buffers and on disk.

use std::path::Path;

use substrate_message::Message;
use substrate_search_domain::SearchError;
use substrate_search_domain::pattern::{FileMask, Matcher, ReplacementMode};
use substrate_search_domain::project::Layout;
use substrate_search_domain::results::{ReplaceSummary, ReplacedDocument, ReplacedFile};

use crate::ReplaceRequest;
use crate::coverage::Coverage;
use crate::plan::Plan;
use crate::ports::{Decoding, DocumentSource, FileWalker};
use crate::source::Source;

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
    (walker, documents): (&impl FileWalker, &impl DocumentSource),
    may_write: &dyn Fn(&Path) -> Result<(), Message>,
) -> Result<ReplaceSummary, SearchError> {
    let replacer = Replacer {
        matcher: Matcher::new(&request.query)?,
        replacement: &request.replacement,
        mode: request.query.replacement_mode(),
    };
    let mask = FileMask::parse(&request.query.file_mask)?;
    let coverage = Coverage {
        root: request.root.as_deref(),
        scope: request.scope,
        documents: &request.documents,
        layout,
    };
    // Generated code is rewritten by the next build — never replaced in.
    let plan = Plan::new(walker, &coverage, &mask, false);
    let mut summary = ReplaceSummary::default();
    for (path, source) in &plan.items {
        match source {
            Source::Buffer(text) => replacer.in_buffer(path, text, &mut summary),
            Source::Disk => replacer.on_disk(path, (documents, may_write), &mut summary),
        }
    }
    Ok(summary)
}

/// Whether a path may be rewritten: the host's refusal, as a message.
type MayWrite<'a> = &'a dyn Fn(&Path) -> Result<(), Message>;

/// A replace's matcher and replacement.
struct Replacer<'a> {
    matcher: Matcher,
    replacement: &'a str,
    mode: ReplacementMode,
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
        (documents, may_write): (&impl DocumentSource, MayWrite<'_>),
        summary: &mut ReplaceSummary,
    ) {
        let Some(text) = documents.read_text(path, Decoding::Strict) else {
            return;
        };
        let (text, count) = self.matcher.replace_all(&text, self.replacement, self.mode);
        if count == 0 {
            return;
        }
        let written =
            may_write(path).and_then(|()| documents.write_text(path, &text).map_err(Message::from));
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
