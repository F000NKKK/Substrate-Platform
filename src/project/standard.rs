//! What kind of project a folder holds, told by the files in it — which
//! the product lists (Substrate reads no folder for it).

use serde::{Deserialize, Serialize};

/// A marker's wildcard: any run of characters within one name (`*.csproj`).
const ANY: char = '*';

/// Between the names of a relative path.
const SEPARATOR: char = '/';

/// A recognizable kind of project: `detect_files` are paths relative to a
/// candidate root (`/`-separated) whose presence, all together, signals a
/// match. A name may hold `*` (`*.csproj`, `src/*.sln`).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectStandard {
    /// Its id.
    pub id: String,
    /// The paths whose presence, all together, signals it.
    pub detect_files: Vec<String>,
}

impl ProjectStandard {
    /// The first standard (in `standards`' order) every one of whose
    /// `detect_files` is among `files` — the folder's files, relative to it
    /// and `/`-separated, as deep as the product lists them. `None` if
    /// nothing matches.
    pub fn detect(files: &[String], standards: &[ProjectStandard]) -> Option<String> {
        standards
            .iter()
            .find(|s| {
                s.detect_files
                    .iter()
                    .all(|marker| files.iter().any(|f| matches(marker, f)))
            })
            .map(|s| s.id.clone())
    }
}

/// Whether `path` fits `pattern`, whose `*`s stand for any run of
/// characters within one name.
fn matches(pattern: &str, path: &str) -> bool {
    let (patterns, names): (Vec<&str>, Vec<&str>) = (
        pattern.split(SEPARATOR).collect(),
        path.split(SEPARATOR).collect(),
    );
    patterns.len() == names.len() && patterns.iter().zip(&names).all(|(p, n)| name_matches(p, n))
}

/// Whether `name` fits `pattern` (one name each).
fn name_matches(pattern: &str, name: &str) -> bool {
    let mut parts = pattern.split(ANY);
    let first = parts.next().unwrap_or_default();
    let Some(mut rest) = name.strip_prefix(first) else {
        return false;
    };
    let parts: Vec<&str> = parts.collect();
    let Some((last, middle)) = parts.split_last() else {
        return rest.is_empty();
    };
    for part in middle {
        match rest.find(part) {
            Some(at) => rest = &rest[at + part.len()..],
            None => return false,
        }
    }
    rest.ends_with(last)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn standard(id: &str, files: &[&str]) -> ProjectStandard {
        ProjectStandard {
            id: id.to_string(),
            detect_files: files.iter().map(|f| f.to_string()).collect(),
        }
    }

    fn files(names: &[&str]) -> Vec<String> {
        names.iter().map(|n| n.to_string()).collect()
    }

    #[test]
    fn detect_returns_first_standard_whose_files_all_exist() {
        let here = files(&["Cargo.toml", "package.json"]);
        let standards = [
            standard("needs-both", &["Cargo.toml", "missing.txt"]),
            standard("rust", &["Cargo.toml"]),
            standard("node", &["package.json"]),
        ];
        assert_eq!(
            ProjectStandard::detect(&here, &standards).as_deref(),
            Some("rust")
        );
        assert_eq!(ProjectStandard::detect(&here, &standards[..1]), None);
    }

    #[test]
    fn a_marker_with_a_wildcard_matches_by_name() {
        let here = files(&["App.csproj", "src/App.sln"]);
        let standards = [
            standard("fsharp", &["*.fsproj"]),
            standard("csharp", &["*.csproj", "src/*.sln"]),
        ];
        assert_eq!(
            ProjectStandard::detect(&here, &standards).as_deref(),
            Some("csharp")
        );
    }

    #[test]
    fn wildcards_match_any_run_within_a_name() {
        assert!(matches("*.csproj", "App.csproj"));
        assert!(matches("App.*.json", "App.Debug.json"));
        assert!(matches("*", ""));
        assert!(!matches("*.csproj", "App.fsproj"));
        assert!(!matches("*.sln", "src/App.sln"));
        assert!(!matches("a*a", "a"));
    }
}
