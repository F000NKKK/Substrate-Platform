//! The two sides of a file's diff, and what a file holds in the last commit.

use super::run::{absolute, git, repo_of, require_repo};
use crate::message::Message;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Stdio;

/// Files larger than this aren't shown in a diff.
const MAX_DIFF_BYTES: usize = 8 * 1024 * 1024;
/// Bytes looked at for a NUL, which makes a file binary.
const BINARY_SNIFF: usize = 8000;
/// A side's `rev` for what's staged, for the file on disk, and for the last commit.
const INDEX: &str = "INDEX";
const WORKTREE: &str = "WORKTREE";
const HEAD: &str = "HEAD";

/// One side of a diff: `rev` is a commit (anything `git rev-parse` takes),
/// `"INDEX"` for what's staged, `"WORKTREE"` for the file on disk, or empty
/// for nothing (the file didn't exist there).
#[derive(Deserialize)]
pub struct DiffSide {
    pub rev: String,
    /// Repo-relative.
    pub path: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiffTexts {
    pub left: Option<String>,
    pub right: Option<String>,
    /// Either side isn't text (or is too large to show).
    pub binary: bool,
}

/// A blob's text (through its text view or the repository's textconv),
/// `None` if it doesn't exist there; `Err` only for binary content.
fn side_text(top: &Path, side: &DiffSide) -> Result<Option<String>, ()> {
    let bytes = match side.rev.as_str() {
        "" => return Ok(None),
        WORKTREE => {
            let file = absolute(top, &side.path);
            if let Some(view) = crate::text_view::find(Path::new(&file)) {
                return Ok((view.read)(Path::new(&file)));
            }
            match std::fs::read(&file) {
                Ok(bytes) => bytes,
                Err(_) => return Ok(None),
            }
        }
        rev => {
            let spec = if rev == INDEX {
                format!(":{}", side.path)
            } else {
                format!("{rev}:{}", side.path)
            };
            // A file the product reads as text: through the same view as on
            // disk, so both sides of its diff are the same text of it.
            if let Some(view) = crate::text_view::find(Path::new(&side.path)) {
                return Ok(
                    blob(top, &spec).and_then(|bytes| through_view(view, &side.path, &bytes))
                );
            }
            // As the repository shows it: through its textconv, if the path has one.
            match git(top)
                .args(["cat-file", "--textconv", &spec])
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .output()
            {
                Ok(out) if out.status.success() => out.stdout,
                _ => return Ok(None),
            }
        }
    };
    if bytes.len() > MAX_DIFF_BYTES || bytes.iter().take(BINARY_SNIFF).any(|&b| b == 0) {
        return Err(());
    }
    Ok(Some(String::from_utf8_lossy(&bytes).into_owned()))
}

/// The bytes of blob `spec` (`<rev>:<path>`, `:<path>`), `None` if there's none.
fn blob(top: &Path, spec: &str) -> Option<Vec<u8>> {
    let out = git(top)
        .args(["cat-file", "blob", spec])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    out.status.success().then_some(out.stdout)
}

/// Where a blob is put for a text view to read it: a folder of its own, its file's name kept.
const VIEW_DIR: &str = "substrate-diff";
static VIEW_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// `bytes` (a blob of `path`) as `view` reads its file: written under its
/// own name in a scratch folder (views go by the name), read, removed.
fn through_view(view: crate::text_view::TextView, path: &str, bytes: &[u8]) -> Option<String> {
    let seq = VIEW_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("{VIEW_DIR}-{}-{seq}", std::process::id()));
    let file = dir.join(Path::new(path).file_name()?);
    let text = std::fs::create_dir_all(&dir)
        .and_then(|()| std::fs::write(&file, bytes))
        .ok()
        .and_then(|()| (view.read)(&file));
    let _ = std::fs::remove_dir_all(&dir);
    text
}

/// Both sides of a file's diff, for the diff view.
pub fn texts(root: &Path, left: &DiffSide, right: &DiffSide) -> Result<DiffTexts, Message> {
    let top = require_repo(root)?;
    Ok(match (side_text(&top, left), side_text(&top, right)) {
        (Ok(left), Ok(right)) => DiffTexts {
            left,
            right,
            binary: false,
        },
        _ => DiffTexts {
            binary: true,
            ..DiffTexts::default()
        },
    })
}

/// What `path` (absolute) holds in the last commit — the editor's change
/// markers compare against it. `None` for a file outside the repository,
/// not committed yet, or binary.
pub fn head_text(root: &Path, path: &Path) -> Result<Option<String>, Message> {
    let Some((top, _)) = repo_of(root)? else {
        return Ok(None);
    };
    let file = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let Ok(rel) = file.strip_prefix(&top) else {
        return Ok(None);
    };
    let rel: Vec<String> = rel
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    Ok(side_text(
        &top,
        &DiffSide {
            rev: HEAD.into(),
            path: rel.join("/"),
        },
    )
    .unwrap_or(None))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_view::{TextView, register};

    fn upper(path: &Path) -> Option<String> {
        std::fs::read_to_string(path).ok().map(|s| s.to_uppercase())
    }

    fn no_write(_: &Path, _: &str) -> Result<(), String> {
        Ok(())
    }

    #[test]
    fn a_blob_reads_through_the_view_its_file_does() {
        let view = TextView {
            read: upper,
            write: no_write,
        };
        register("difftest", view);
        let text = through_view(view, "dir/a.difftest", b"binary-ish");
        assert_eq!(text.as_deref(), Some("BINARY-ISH"));
    }
}
