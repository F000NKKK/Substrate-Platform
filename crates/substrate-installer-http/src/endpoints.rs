//! Where the APIs are.

/// GitHub's REST API.
const GITHUB_API: &str = "https://api.github.com/";
/// crates.io's API.
const CRATES_API: &str = "https://crates.io/api/v1/";

/// The base addresses of the APIs the listing asks. The defaults are the real
/// ones; a test or a mirror points them elsewhere.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoints {
    github: String,
    crates: String,
}

impl Endpoints {
    /// GitHub's and crates.io's own APIs.
    #[must_use]
    pub fn official() -> Self {
        Self::new(GITHUB_API, CRATES_API)
    }

    /// APIs at these base addresses (a missing trailing `/` is added).
    #[must_use]
    pub fn new(github: &str, crates: &str) -> Self {
        Self {
            github: with_slash(github),
            crates: with_slash(crates),
        }
    }

    /// GitHub's base address, ending with `/`.
    #[must_use]
    pub fn github(&self) -> &str {
        &self.github
    }

    /// crates.io's base address, ending with `/`.
    #[must_use]
    pub fn crates(&self) -> &str {
        &self.crates
    }
}

impl Default for Endpoints {
    fn default() -> Self {
        Self::official()
    }
}

fn with_slash(base: &str) -> String {
    if base.ends_with('/') {
        base.to_owned()
    } else {
        format!("{base}/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bases_end_with_a_slash() {
        let endpoints = Endpoints::new("http://127.0.0.1:1", "http://h/api/");
        assert_eq!(
            (endpoints.github(), endpoints.crates()),
            ("http://127.0.0.1:1/", "http://h/api/")
        );
        assert_eq!(Endpoints::default().github(), "https://api.github.com/");
    }
}
