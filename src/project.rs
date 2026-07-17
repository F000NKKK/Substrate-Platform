//! Project/solution model — a "project" is a directory plus a recognized
//! "standard" (what kind of thing lives there), and a "solution" groups one
//! or more projects, the same shape as an IDE's own solution/workspace
//! concept. No concrete standards ship here — only the generic machinery a
//! host uses to define its own (e.g. Yog-IDLE's "yog-rust-mod" standard).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// A recognizable "kind" of project — `detect_files` are paths (relative to
/// a candidate root) whose presence, all together, signals a match.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectStandard {
    pub id: String,
    pub detect_files: Vec<String>,
}

impl ProjectStandard {
    /// The first standard (in `standards`' order) every one of whose
    /// `detect_files` exists under `root` — `None` if nothing matches.
    pub fn detect(root: &Path, standards: &[ProjectStandard]) -> Option<String> {
        standards
            .iter()
            .find(|s| s.detect_files.iter().all(|f| root.join(f).exists()))
            .map(|s| s.id.clone())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub root: PathBuf,
    /// A [`ProjectStandard::id`], if one matched (or was set explicitly).
    pub kind: Option<String>,
}

impl Project {
    /// Where this project's own workflow definitions would live, if it has
    /// any — a host falls back to its built-in default workflow set for
    /// `kind` when this path doesn't exist.
    pub fn workflow_path(&self) -> PathBuf {
        self.root.join("workflow.toml")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Solution {
    pub name: String,
    pub projects: Vec<Project>,
}

/// On-disk `.yogsln` shape — distinct from [`Solution`] because `path` is
/// stored relative to the file itself, not resolved to an absolute [`Project::root`].
#[derive(Debug, Clone, Deserialize, Serialize)]
struct SolutionFile {
    solution: SolutionFileMeta,
    #[serde(rename = "project", default)]
    projects: Vec<ProjectFileEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SolutionFileMeta {
    name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ProjectFileEntry {
    name: String,
    path: String,
    kind: Option<String>,
}

impl Solution {
    pub fn load(path: &Path) -> anyhow::Result<Solution> {
        let contents = std::fs::read_to_string(path)?;
        let file: SolutionFile = toml::from_str(&contents)?;
        // `Path::join` with an absolute `path` replaces the base entirely, so
        // this resolves correctly whether entries were written as relative
        // or absolute.
        let base = path.parent().unwrap_or_else(|| Path::new("."));
        Ok(Solution {
            name: file.solution.name,
            projects: file
                .projects
                .into_iter()
                .map(|p| Project { name: p.name, root: base.join(&p.path), kind: p.kind })
                .collect(),
        })
    }

    /// Writes each project's root as an absolute path — portable relative
    /// paths (surviving the whole tree being moved) are a natural follow-up,
    /// not required for a solution to load back correctly.
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let file = SolutionFile {
            solution: SolutionFileMeta { name: self.name.clone() },
            projects: self
                .projects
                .iter()
                .map(|p| ProjectFileEntry {
                    name: p.name.clone(),
                    path: p.root.to_string_lossy().into_owned(),
                    kind: p.kind.clone(),
                })
                .collect(),
        };
        std::fs::write(path, toml::to_string_pretty(&file)?)?;
        Ok(())
    }
}
