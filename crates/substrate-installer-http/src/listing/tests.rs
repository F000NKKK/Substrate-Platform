//! The release source against a local server.

use std::sync::atomic::{AtomicUsize, Ordering};

use substrate_installer_app::ports::sources::{ListingError, ReleaseSource};
use substrate_installer_domain::sources::{CrateName, RepoSlug, Source, Toolchain};

use super::HttpReleaseSource;
use crate::testing::{MockServer, Reply};
use crate::{
    ClientConfig, Endpoints, GithubToken, HttpClient, HttpError, ReleaseReadError, is_rate_limited,
};

const GITHUB_JSON: &str = include_str!("fixtures/github_releases.json");
const CRATES_JSON: &str = include_str!("fixtures/crates_io.json");
const RELEASES_PATH: &str = "/repos/o/r/releases?per_page=30";

fn git_tag() -> Source {
    Source::GitTag {
        repo: RepoSlug::new("o/r").unwrap(),
    }
}

fn source_at(server: &MockServer, config: ClientConfig) -> HttpReleaseSource {
    let endpoints = Endpoints::new(&server.url(""), &server.url("/api"));
    HttpReleaseSource::new(HttpClient::new(config.with_endpoints(endpoints)))
}

fn versions(source: &HttpReleaseSource, what: &Source) -> Vec<String> {
    let releases = source.releases(what).unwrap();
    releases.iter().map(|r| r.version.to_string()).collect()
}

#[test]
fn github_releases_come_through_the_token_and_the_page_size() {
    let server = MockServer::start(|_| Reply::ok(GITHUB_JSON));
    let token = GithubToken::new("secret").unwrap();
    let source = source_at(&server, ClientConfig::new("test").with_github_token(token));
    assert_eq!(versions(&source, &git_tag()), ["2.1.0", "2.0.0"]);
    let request = &server.requests()[0];
    assert_eq!(request.path, RELEASES_PATH);
    assert_eq!(request.headers["authorization"], "Bearer secret");
    assert_eq!(request.headers["accept"], "application/vnd.github+json");
}

#[test]
fn crates_io_lists_the_unyanked_versions() {
    let server = MockServer::start(|_| Reply::ok(CRATES_JSON));
    let source = source_at(&server, ClientConfig::new("test"));
    let what = Source::CratesIo {
        name: CrateName::new("demo").unwrap(),
    };
    assert_eq!(versions(&source, &what), ["0.10.0", "0.9.0", "0.3.0"]);
    assert_eq!(server.requests()[0].path, "/api/crates/demo");
}

#[test]
fn a_toolchain_is_its_own_one_release_and_asks_nobody() {
    let server = MockServer::start(|_| Reply::status(500));
    let source = source_at(&server, ClientConfig::new("test"));
    let what = Source::Rustup {
        toolchain: Toolchain::new("1.99").unwrap(),
    };
    assert_eq!(versions(&source, &what), ["1.99"]);
    assert!(server.requests().is_empty());
}

#[test]
fn an_unchanged_answer_is_asked_with_its_etag_and_read_from_the_cache() {
    let server = MockServer::start(|request| {
        if request.headers.get("if-none-match").map(String::as_str) == Some("\"v1\"") {
            Reply::status(304)
        } else {
            Reply::ok(GITHUB_JSON).header("ETag", "\"v1\"")
        }
    });
    let dir = tempfile::tempdir().unwrap();
    let source = source_at(
        &server,
        ClientConfig::new("test").with_cache_dir(dir.path()),
    );
    let first = versions(&source, &git_tag());
    let second = versions(&source, &git_tag());
    assert_eq!(first, second);
    let requests = server.requests();
    assert_eq!(requests.len(), 2);
    assert!(!requests[0].headers.contains_key("if-none-match"));
    assert_eq!(requests[1].headers["if-none-match"], "\"v1\"");
}

#[test]
fn a_rate_limit_is_stale_cache_when_there_is_one_and_a_named_error_when_not() {
    let calls = AtomicUsize::new(0);
    let server = MockServer::start(move |_| {
        if calls.fetch_add(1, Ordering::Relaxed) == 0 {
            Reply::ok(GITHUB_JSON).header("ETag", "\"v1\"")
        } else {
            Reply::status(429).header("Retry-After", "60")
        }
    });
    let dir = tempfile::tempdir().unwrap();
    let cached = source_at(
        &server,
        ClientConfig::new("test").with_cache_dir(dir.path()),
    );
    let first = versions(&cached, &git_tag());
    assert_eq!(versions(&cached, &git_tag()), first);
    let bare = source_at(&server, ClientConfig::new("test"));
    let error = bare.releases(&git_tag()).unwrap_err();
    assert!(is_rate_limited(&error), "{error:?}");
    let ListingError::Failed { cause, .. } = error else {
        panic!("not a failure");
    };
    assert!(matches!(
        cause.downcast_ref::<HttpError>(),
        Some(HttpError::RateLimited {
            retry_after_seconds: Some(60),
            ..
        })
    ));
}

#[test]
fn a_plain_refusal_is_not_a_rate_limit() {
    let server = MockServer::start(|_| Reply::status(403));
    let source = source_at(&server, ClientConfig::new("test"));
    let error = source.releases(&git_tag()).unwrap_err();
    assert!(!is_rate_limited(&error));
    assert!(matches!(error, ListingError::Failed { .. }));
}

#[test]
fn an_answer_that_is_not_json_names_the_address() {
    let server = MockServer::start(|_| Reply::ok("<html>"));
    let source = source_at(&server, ClientConfig::new("test"));
    let ListingError::Failed { cause, .. } = source.releases(&git_tag()).unwrap_err() else {
        panic!("not a failure");
    };
    assert!(matches!(
        cause.downcast_ref::<ReleaseReadError>(),
        Some(ReleaseReadError::Json { .. })
    ));
}
