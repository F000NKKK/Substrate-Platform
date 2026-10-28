//! The command line of a silent setup, decided without running anything.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// An NSIS setup's switch for a silent run.
const SILENT: &str = "/S";
/// Its switch for the folder to install into: it must come last and is not quoted.
const FOLDER: &str = "/D=";

/// A program to run and its arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SetupCommand {
    pub(super) program: PathBuf,
    pub(super) args: Vec<OsString>,
}

/// The command that runs `setup` silently, installing into `folder`.
pub(super) fn install_command(setup: &Path, folder: &Path) -> SetupCommand {
    let mut target = OsString::from(FOLDER);
    target.push(folder);
    SetupCommand {
        program: setup.to_path_buf(),
        args: vec![OsString::from(SILENT), target],
    }
}

/// The command that runs an installed `uninstaller` silently.
pub(super) fn uninstall_command(uninstaller: &Path) -> SetupCommand {
    SetupCommand {
        program: uninstaller.to_path_buf(),
        args: vec![OsString::from(SILENT)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_folder_comes_last_and_unquoted() {
        let command = install_command(Path::new("/dl/setup.exe"), Path::new("C:/Yog/p/1.0"));
        assert_eq!(command.program, Path::new("/dl/setup.exe"));
        assert_eq!(
            command.args,
            [OsString::from("/S"), OsString::from("/D=C:/Yog/p/1.0")]
        );
    }

    #[test]
    fn uninstalling_is_silent() {
        let command = uninstall_command(Path::new("/p/uninstall.exe"));
        assert_eq!(command.args, [OsString::from("/S")]);
    }
}
