//! How the HTTP client is made.

use std::path::PathBuf;

use crate::{Endpoints, GithubToken};

/// What an [`HttpClient`](crate::HttpClient) is made of. There is no global
/// state: the composition root builds one config and hands the client to the
/// adapters.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub(crate) user_agent: String,
    pub(crate) cache_dir: Option<PathBuf>,
    pub(crate) github_token: Option<GithubToken>,
    pub(crate) endpoints: Endpoints,
}

impl ClientConfig {
    /// A config for a client that introduces itself as `user_agent` (GitHub's
    /// API wants one), with no cache and no token.
    #[must_use]
    pub fn new(user_agent: impl Into<String>) -> Self {
        Self {
            user_agent: user_agent.into(),
            cache_dir: None,
            github_token: None,
            endpoints: Endpoints::official(),
        }
    }

    /// Keeps the JSON answers (with their `ETag`s) in `dir`, so a repeated
    /// question costs a `304` and a rate limit is answered from the cache.
    #[must_use]
    pub fn with_cache_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.cache_dir = Some(dir.into());
        self
    }

    /// Sends `token` to the GitHub API.
    #[must_use]
    pub fn with_github_token(mut self, token: GithubToken) -> Self {
        self.github_token = Some(token);
        self
    }

    /// Asks the APIs at `endpoints` instead of the official ones.
    #[must_use]
    pub fn with_endpoints(mut self, endpoints: Endpoints) -> Self {
        self.endpoints = endpoints;
        self
    }
}
