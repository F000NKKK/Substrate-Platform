//! Project/solution model — a "project" is a directory plus a recognized
//! "standard" (what kind of thing lives there), and a "solution" groups one
//! or more projects, the same shape as an IDE's own solution/workspace
//! concept. No concrete standards ship here — only the generic machinery a
//! host uses to define its own (e.g. a "rust-crate" standard detected by a
//! `Cargo.toml`).

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

/// On-disk solution file shape (TOML; the file name/extension is the host's
/// choice — [`Solution::load`]/[`Solution::save`] take any path) — distinct from [`Solution`] because `path` is
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
                .map(|p| Project {
                    name: p.name,
                    root: base.join(&p.path),
                    kind: p.kind,
                })
                .collect(),
        })
    }

    /// Writes each project's root as an absolute path — portable relative
    /// paths (surviving the whole tree being moved) are a natural follow-up,
    /// not required for a solution to load back correctly.
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let file = SolutionFile {
            solution: SolutionFileMeta {
                name: self.name.clone(),
            },
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

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("substrate-project-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn standard(id: &str, files: &[&str]) -> ProjectStandard {
        ProjectStandard {
            id: id.to_string(),
            detect_files: files.iter().map(|f| f.to_string()).collect(),
        }
    }

    #[test]
    fn detect_returns_first_standard_whose_files_all_exist() {
        let root = temp_dir("detect");
        std::fs::write(root.join("Cargo.toml"), "").unwrap();
        std::fs::write(root.join("package.json"), "").unwrap();

        let standards = [
            standard("needs-both", &["Cargo.toml", "missing.txt"]),
            standard("rust", &["Cargo.toml"]),
            standard("node", &["package.json"]),
        ];
        assert_eq!(
            ProjectStandard::detect(&root, &standards).as_deref(),
            Some("rust")
        );
        assert_eq!(ProjectStandard::detect(&root, &standards[..1]), None);
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn solution_round_trips_through_save_and_load() {
        let dir = temp_dir("solution");
        let path = dir.join("app.sln.toml");
        let solution = Solution {
            name: "App".to_string(),
            projects: vec![
                Project {
                    name: "core".to_string(),
                    root: dir.join("core"),
                    kind: Some("rust".to_string()),
                },
                Project {
                    name: "docs".to_string(),
                    root: dir.join("docs"),
                    kind: None,
                },
            ],
        };
        solution.save(&path).unwrap();

        let loaded = Solution::load(&path).unwrap();
        assert_eq!(loaded.name, "App");
        let projects: Vec<_> = loaded
            .projects
            .iter()
            .map(|p| (p.name.as_str(), p.root.clone(), p.kind.clone()))
            .collect();
        assert_eq!(
            projects,
            vec![
                ("core", dir.join("core"), Some("rust".to_string())),
                ("docs", dir.join("docs"), None),
            ]
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn solution_load_resolves_relative_paths_against_the_file() {
        let dir = temp_dir("relative");
        let path = dir.join("app.sln.toml");
        std::fs::write(
            &path,
            "[solution]\nname = \"App\"\n\n[[project]]\nname = \"core\"\npath = \"crates/core\"\n",
        )
        .unwrap();

        let loaded = Solution::load(&path).unwrap();
        assert_eq!(loaded.projects[0].root, dir.join("crates/core"));
        assert_eq!(loaded.projects[0].kind, None);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
