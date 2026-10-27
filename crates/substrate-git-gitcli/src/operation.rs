//! Which operation waits to be finished.

use std::path::Path;

use substrate_git_domain::status::Operation;

/// What in `.git` says it's waiting, for each operation.
const MARKERS: [(&[&str], Operation); 4] = [
    (&["rebase-merge", "rebase-apply"], Operation::Rebase),
    (&["MERGE_HEAD"], Operation::Merge),
    (&["CHERRY_PICK_HEAD"], Operation::CherryPick),
    (&["REVERT_HEAD"], Operation::Revert),
];

/// The operation waiting in `git_dir`, if any.
pub(crate) fn in_progress(git_dir: &Path) -> Option<Operation> {
    MARKERS
        .iter()
        .find(|(files, _)| files.iter().any(|f| git_dir.join(f).exists()))
        .map(|(_, op)| *op)
}

/// The git command that continues or aborts `operation`.
pub(crate) fn command(operation: Operation) -> &'static str {
    match operation {
        Operation::Rebase => "rebase",
        Operation::Merge => "merge",
        Operation::CherryPick => "cherry-pick",
        Operation::Revert => "revert",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_marker_in_the_git_dir_names_the_operation() {
        let dir = std::env::temp_dir().join(format!("substrate-op-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        assert_eq!(in_progress(&dir), None);
        std::fs::write(dir.join("MERGE_HEAD"), "x").unwrap();
        assert_eq!(in_progress(&dir), Some(Operation::Merge));
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(command(Operation::CherryPick), "cherry-pick");
    }
}
