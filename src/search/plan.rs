//! What one search or replace goes through: each file, and where its text comes from.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::coverage::Coverage;
use super::source::Source;
use super::{FileMask, OpenDocument, Scope, generated_files, project_files, relative};

/// The files to go through, in order.
pub(crate) struct Plan<'a> {
    pub(crate) root: Option<PathBuf>,
    pub(crate) items: Vec<(PathBuf, Source<'a>)>,
}

impl<'a> Plan<'a> {
    /// The files `coverage` takes in that pass `mask`; generated code only
    /// with `include_generated`.
    pub(crate) fn new(coverage: &Coverage<'a>, mask: &FileMask, include_generated: bool) -> Self {
        let root = coverage.root.map(PathBuf::from);
        let items = match (coverage.scope, &root) {
            (Scope::Documents, _) => documents(root.as_deref(), coverage.documents, mask),
            (Scope::Project, Some(dir)) => {
                let mut files = project_files(dir, mask, coverage.layout);
                if include_generated {
                    files.extend(generated_files(dir, mask, coverage.layout));
                }
                with_buffers(files, coverage.documents)
            }
            (Scope::Project, None) => Vec::new(),
        };
        Self { root, items }
    }

    /// `path` as a result names it: relative to the root, else as it is.
    pub(crate) fn relative_path(&self, path: &Path) -> String {
        self.root.as_deref().map_or_else(
            || path.to_string_lossy().into_owned(),
            |root| relative(root, path),
        )
    }
}

/// The open documents that pass `mask`, read from their buffers.
fn documents<'a>(
    root: Option<&Path>,
    documents: &'a [OpenDocument],
    mask: &FileMask,
) -> Vec<(PathBuf, Source<'a>)> {
    documents
        .iter()
        .filter(|document| {
            let path = Path::new(&document.path);
            mask.matches(&root.map_or(document.path.clone(), |root| relative(root, path)))
        })
        .map(|document| {
            (
                PathBuf::from(&document.path),
                Source::Buffer(&document.text),
            )
        })
        .collect()
}

/// `files`, each read from its open document's buffer where there is one.
fn with_buffers(files: Vec<PathBuf>, documents: &[OpenDocument]) -> Vec<(PathBuf, Source<'_>)> {
    let buffers: HashMap<PathBuf, &str> = documents
        .iter()
        .flat_map(|document| {
            let raw = PathBuf::from(&document.path);
            let canonical = raw.canonicalize().ok();
            std::iter::once((raw, document.text.as_str()))
                .chain(canonical.map(|path| (path, document.text.as_str())))
        })
        .collect();
    files
        .into_iter()
        .map(|path| {
            let source = buffers
                .get(&path)
                .map_or(Source::Disk, |text| Source::Buffer(text));
            (path, source)
        })
        .collect()
}
