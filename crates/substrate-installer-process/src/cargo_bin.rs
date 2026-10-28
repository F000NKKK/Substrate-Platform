//! Where cargo and rustup live.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use substrate_installer_app::ports::tools::ToolProgram;
use substrate_installer_domain::sources::CrateName;

use crate::executable::executable;

/// cargo's `bin` folder: where `rustup` and `cargo` live and where
/// `cargo install` puts what it builds (`$CARGO_HOME/bin`, else
/// `~/.cargo/bin`). A program missing from the folder is looked for in the
/// folders of a search path, when the value has one (see
/// [`from_environment`](Self::from_environment) and
/// [`with_path`](Self::with_path)).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoBin {
    dir: PathBuf,
    path: Option<OsString>,
}

impl CargoBin {
    /// The folder `dir`, with no search path: only programs in it are found.
    #[must_use]
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self {
            dir: dir.into(),
            path: None,
        }
    }

    /// The same folder, with `path` (a `PATH`-style list of folders) to look
    /// in for what the folder lacks.
    #[must_use]
    pub fn with_path(mut self, path: impl Into<OsString>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// The folder this machine's environment says (`CARGO_HOME`, else the
    /// home folder's `.cargo`), searching the environment's `PATH` for what it
    /// lacks; none when neither folder is known.
    #[must_use]
    pub fn from_environment() -> Option<Self> {
        let bin = Self::resolve(std::env::var_os("CARGO_HOME"), std::env::home_dir())?;
        Some(match std::env::var_os("PATH") {
            Some(path) => bin.with_path(path),
            None => bin,
        })
    }

    /// The folder for a `CARGO_HOME` value and a home folder; an empty
    /// `CARGO_HOME` counts as unset.
    #[must_use]
    pub fn resolve(cargo_home: Option<OsString>, home: Option<PathBuf>) -> Option<Self> {
        let root = cargo_home
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
            .or_else(|| home.map(|home| home.join(".cargo")))?;
        Some(Self::new(root.join("bin")))
    }

    /// The folder.
    #[must_use]
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Where `program` is: in this folder, else in the search path.
    #[must_use]
    pub fn locate(&self, program: ToolProgram) -> Option<PathBuf> {
        let file = executable(&program.to_string());
        let own = self.dir.join(&file);
        if own.is_file() {
            return Some(own);
        }
        find_on_path(&file, self.path.as_deref()?)
    }

    /// The binary `cargo install` puts for the crate `name`.
    pub(crate) fn crate_binary(&self, name: &CrateName) -> PathBuf {
        self.dir.join(executable(name.as_str()))
    }

    /// Where `rustup` would be if it were installed here.
    pub(crate) fn rustup_path(&self) -> PathBuf {
        self.dir.join(executable(&ToolProgram::Rustup.to_string()))
    }
}

/// The first folder of `path` that has the file `file`.
fn find_on_path(file: &str, path: &OsStr) -> Option<PathBuf> {
    std::env::split_paths(path)
        .map(|folder| folder.join(file))
        .find(|candidate| candidate.is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cargo_home_wins_over_the_home_folder() {
        let bin = CargoBin::resolve(Some("/opt/cargo".into()), Some("/home/u".into()));
        assert_eq!(bin.unwrap().dir(), Path::new("/opt/cargo/bin"));
    }

    #[test]
    fn without_cargo_home_it_is_dot_cargo_in_the_home_folder() {
        let bin = CargoBin::resolve(None, Some("/home/u".into())).unwrap();
        assert_eq!(bin.dir(), Path::new("/home/u/.cargo/bin"));
        let empty = CargoBin::resolve(Some(OsString::new()), Some("/home/u".into()));
        assert_eq!(empty, Some(bin));
    }

    #[test]
    fn with_neither_there_is_no_folder() {
        assert_eq!(CargoBin::resolve(None, None), None);
    }

    #[test]
    fn a_program_in_the_folder_is_found_there() {
        let dir = tempfile::tempdir().unwrap();
        let bin = CargoBin::new(dir.path());
        std::fs::write(dir.path().join(executable("rustup")), "").unwrap();
        assert_eq!(bin.locate(ToolProgram::Rustup), Some(bin.rustup_path()));
    }

    #[test]
    fn the_search_path_is_used_only_when_given() {
        let dir = tempfile::tempdir().unwrap();
        let file = executable("cargo");
        std::fs::write(dir.path().join(&file), "").unwrap();
        let bare = CargoBin::new("/nonexistent");
        assert_eq!(bare.locate(ToolProgram::Cargo), None);
        let searching = bare.with_path(dir.path());
        assert_eq!(
            searching.locate(ToolProgram::Cargo),
            Some(dir.path().join(file))
        );
    }

    #[test]
    fn a_program_is_found_on_the_path() {
        let dir = tempfile::tempdir().unwrap();
        let file = executable("tool");
        std::fs::write(dir.path().join(&file), "").unwrap();
        let path = std::env::join_paths([Path::new("/nonexistent"), dir.path()]).unwrap();
        assert_eq!(find_on_path(&file, &path), Some(dir.path().join(&file)));
        assert_eq!(find_on_path("nothing-like-this", &path), None);
    }

    #[test]
    fn a_crates_binary_is_in_the_folder() {
        let bin = CargoBin::new("/c/bin");
        let name = CrateName::new("cargo-zigbuild").unwrap();
        assert_eq!(
            bin.crate_binary(&name),
            Path::new("/c/bin").join(executable("cargo-zigbuild"))
        );
    }
}
