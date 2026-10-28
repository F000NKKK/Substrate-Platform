//! The agent, the cache and the token, in one value.

use std::time::Duration;

use ureq::http::Response;
use ureq::{Agent, Body};

use crate::rate_limit::{is_rate_limited, retry_after_seconds};
use crate::response_cache::{CachedAnswer, ResponseCache};
use crate::{ClientConfig, Endpoints, GithubToken, HttpError};

/// GitHub's API wants this media type.
const GITHUB_JSON: &str = "application/vnd.github+json";
/// How long to wait for a connection, and for the next bytes of an answer.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(30);
const READ_TIMEOUT: Duration = Duration::from_secs(60);
/// The most a JSON answer may weigh.
const JSON_LIMIT: u64 = 64 * 1024 * 1024;
const NOT_MODIFIED: u16 = 304;

/// Asks servers: JSON answers with an `ETag` cache (and the GitHub token, to
/// GitHub's API alone) and the raw responses of file downloads. Cheap to
/// clone; the clones share the agent's connections.
#[derive(Debug, Clone)]
pub struct HttpClient {
    agent: Agent,
    cache: Option<ResponseCache>,
    token: Option<GithubToken>,
    endpoints: Endpoints,
}

impl HttpClient {
    /// A client made as `config` says.
    #[must_use]
    pub fn new(config: ClientConfig) -> Self {
        let agent = Agent::config_builder()
            .user_agent(config.user_agent)
            // Statuses are read, not raised: a 304 and a rate limit are answers.
            .http_status_as_error(false)
            .timeout_connect(Some(CONNECT_TIMEOUT))
            .timeout_per_read(Some(READ_TIMEOUT))
            .build()
            .into();
        Self {
            agent,
            cache: config.cache_dir.map(ResponseCache::new),
            token: config.github_token,
            endpoints: config.endpoints,
        }
    }

    /// The APIs this client asks.
    pub(crate) fn endpoints(&self) -> &Endpoints {
        &self.endpoints
    }

    /// `url`'s JSON, as text. Asked again only if it changed (a `304` does
    /// not count against GitHub's limit); the kept answer is used when the
    /// server is rate limiting.
    pub(crate) fn json_text(&self, url: &str) -> Result<String, HttpError> {
        let cached = self.cache.as_ref().and_then(|cache| cache.load(url));
        let mut request = self.agent.get(url).header("Accept", GITHUB_JSON);
        if let Some(token) = self.token_for(url) {
            request = request.header("Authorization", token.bearer());
        }
        if let Some(etag) = cached.as_ref().and_then(|kept| kept.etag.as_deref()) {
            request = request.header("If-None-Match", etag);
        }
        let mut response = request.call().map_err(|source| transport(url, source))?;
        let status = response.status().as_u16();
        if let Some(kept) = cached
            && (status == NOT_MODIFIED || is_rate_limited(status, response.headers()))
        {
            return Ok(kept.body);
        }
        check_status(url, &response)?;
        self.read_and_keep(url, &mut response)
    }

    /// The response to a download of `url` from byte `from` on (a `Range`
    /// request when `from` is not 0). The status is the caller's to read.
    pub(crate) fn file(&self, url: &str, from: u64) -> Result<Response<Body>, HttpError> {
        let mut request = self.agent.get(url);
        if from > 0 {
            request = request.header("Range", format!("bytes={from}-"));
        }
        request.call().map_err(|source| transport(url, source))
    }

    fn token_for(&self, url: &str) -> Option<&GithubToken> {
        self.token
            .as_ref()
            .filter(|_| url.starts_with(self.endpoints.github()))
    }

    fn read_and_keep(&self, url: &str, response: &mut Response<Body>) -> Result<String, HttpError> {
        let etag = response
            .headers()
            .get("etag")
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let body = response
            .body_mut()
            .with_config()
            .limit(JSON_LIMIT)
            .read_to_string()
            .map_err(|source| transport(url, source))?;
        if let Some(cache) = &self.cache {
            let answer = CachedAnswer {
                etag,
                body: body.clone(),
            };
            cache.store(url, &answer);
        }
        Ok(body)
    }
}

/// A success, or the error its status stands for.
pub(crate) fn check_status(url: &str, response: &Response<Body>) -> Result<(), HttpError> {
    let status = response.status();
    if status.is_success() {
        Ok(())
    } else if is_rate_limited(status.as_u16(), response.headers()) {
        Err(HttpError::RateLimited {
            url: url.to_owned(),
            retry_after_seconds: retry_after_seconds(response.headers()),
        })
    } else {
        Err(HttpError::Status {
            url: url.to_owned(),
            status: status.as_u16(),
        })
    }
}

pub(crate) fn transport(url: &str, source: ureq::Error) -> HttpError {
    HttpError::Transport {
        url: url.to_owned(),
        source: Box::new(source),
    }
}
