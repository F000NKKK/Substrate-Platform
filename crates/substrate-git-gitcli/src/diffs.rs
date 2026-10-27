//! A file's versions as text, through `git`.

use std::path::Path;

use substrate_git_app::ports::Diffs;
use substrate_git_domain::GitError;
use substrate_git_domain::diffs::{DiffSide, DiffTexts, SideVersion};

use crate::GitCli;
use crate::cli::{repo_of, require_repo};
use crate::diff_text::side_text;

impl Diffs for GitCli {
    fn texts(&self, root: &Path, left: &DiffSide, right: &DiffSide) -> Result<DiffTexts, GitError> {
        let top = require_repo(root)?;
        let (left, right) = (
            side_text(&top, left, &self.views),
            side_text(&top, right, &self.views),
        );
        Ok(match (left, right) {
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

    fn head_text(&self, root: &Path, path: &Path) -> Result<Option<String>, GitError> {
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
        let side = DiffSide {
            version: SideVersion::Revision("HEAD".into()),
            path: rel.join("/"),
        };
        Ok(side_text(&top, &side, &self.views).unwrap_or(None))
    }
}
