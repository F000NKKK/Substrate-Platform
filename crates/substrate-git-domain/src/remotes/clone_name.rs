//! The folder a clone goes into.

/// The folder a clone of `url` goes into by default — its last path segment, minus `.git`.
#[must_use]
pub fn clone_directory_name(url: &str) -> String {
    let trimmed = url.trim().trim_end_matches(['/', '\\']);
    let last = trimmed.rsplit(['/', '\\', ':']).next().unwrap_or(trimmed);
    let name = last.strip_suffix(".git").unwrap_or(last);
    if name.is_empty() {
        "repository".to_string()
    } else {
        name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_clone_folders() {
        assert_eq!(
            clone_directory_name("https://github.com/owner/Some-App.git"),
            "Some-App"
        );
        assert_eq!(
            clone_directory_name("git@github.com:owner/other.git"),
            "other"
        );
        assert_eq!(clone_directory_name("git@host:repo"), "repo");
        assert_eq!(clone_directory_name("C:\\repos\\local\\"), "local");
        assert_eq!(clone_directory_name(""), "repository");
    }
}
