//! `git diff --name-status -z` as [`FileChange`]s.

use substrate_git_domain::status::{ChangeKind, FileChange};

/// Parses `git diff --name-status -z` (also `diff-tree`'s): `M\0path\0`,
/// `R100\0old\0new\0`.
#[must_use]
pub fn parse_name_status(raw: &str) -> Vec<FileChange> {
    let mut files = Vec::new();
    let mut fields = raw.split('\0').filter(|f| !f.is_empty());
    while let Some(code) = fields.next() {
        let Some(kind) = code.chars().next().and_then(ChangeKind::from_code) else {
            continue;
        };
        let Some(first) = fields.next() else { break };
        if matches!(kind, ChangeKind::Renamed | ChangeKind::Copied) {
            let Some(second) = fields.next() else { break };
            files.push(FileChange::new(second, Some(first), kind));
        } else {
            files.push(FileChange::new(first, None, kind));
        }
    }
    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_name_status() {
        let raw = "M\0src/a.rs\0R087\0old.rs\0new.rs\0A\0added file.txt\0D\0gone.rs\0";
        assert_eq!(
            parse_name_status(raw),
            [
                FileChange::new("src/a.rs", None, ChangeKind::Modified),
                FileChange::new("new.rs", Some("old.rs"), ChangeKind::Renamed),
                FileChange::new("added file.txt", None, ChangeKind::Added),
                FileChange::new("gone.rs", None, ChangeKind::Deleted),
            ]
        );
    }
}
