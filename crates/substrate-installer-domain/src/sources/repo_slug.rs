//! The [`RepoSlug`].

use crate::texts::define_text;

define_text! {
    /// A GitHub repository as `owner/name`.
    RepoSlug, RepoSlug, repo_slug
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_keeps_its_text() {
        let value = RepoSlug::new("owner/name").unwrap();
        assert_eq!(value.as_str(), "owner/name");
        assert!(RepoSlug::new("").is_err());
    }
}
