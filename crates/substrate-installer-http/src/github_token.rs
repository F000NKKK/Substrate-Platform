//! A GitHub token.

use std::fmt;

/// A GitHub token, which lifts the API's limit from 60 requests an hour to
/// 5000. The composition root finds it (`GITHUB_TOKEN`, `gh auth token`);
/// this crate only carries it, and sends it to the GitHub API alone, never to
/// a download link.
#[derive(Clone)]
pub struct GithubToken(String);

impl GithubToken {
    /// The token `text`; none when it is blank.
    #[must_use]
    pub fn new(text: &str) -> Option<Self> {
        let text = text.trim();
        (!text.is_empty()).then(|| Self(text.to_owned()))
    }

    /// The `Authorization` header's value.
    pub(crate) fn bearer(&self) -> String {
        format!("Bearer {}", self.0)
    }
}

impl fmt::Debug for GithubToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GithubToken(..)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_blank_token_is_none_and_a_real_one_stays_secret() {
        assert!(GithubToken::new("  \n").is_none());
        let token = GithubToken::new(" abc\n").unwrap();
        assert_eq!(token.bearer(), "Bearer abc");
        assert!(!format!("{token:?}").contains("abc"));
    }
}
