//! Private release assets use the API and never send credentials to storage redirects.

use std::fs;

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::downloads::{DownloadRequest, Downloader, TransferControl};
use substrate_installer_app::ports::sources::ReleaseSource;
use substrate_installer_domain::releases::HttpUrl;
use substrate_installer_domain::sources::{RepoSlug, Source};

use super::support::{digest, serve_ranged};
use crate::testing::{MockServer, Reply, Request};
use crate::{ClientConfig, Endpoints, GithubToken, HttpClient, HttpDownloader, HttpReleaseSource};

#[test]
fn private_assets_resume_through_the_api_without_leaking_tokens_to_storage() {
    let body = b"verified release bytes";
    let storage = MockServer::start(serve_ranged(body.to_vec()));
    let storage_url = storage.url("/binary");
    let api = MockServer::start(move |request| private_api(request, &storage_url));
    let client = client(&api);
    let releases = HttpReleaseSource::new(client.clone())
        .releases(&Source::GitTag {
            repo: RepoSlug::new("o/r").unwrap(),
        })
        .unwrap();
    let asset = &releases[0].assets[0];
    assert_eq!(asset.url.as_str(), api.url("/repos/o/r/releases/assets/7"));
    let dir = tempfile::tempdir().unwrap();
    let destination = dir.path().join("file.bin");
    fs::write(dir.path().join("file.bin.part"), &body[..3]).unwrap();
    let cancel = CancelToken::new();
    let control = TransferControl {
        cancel: &cancel,
        progress: &|_| {},
    };
    let hash = digest(body);
    HttpDownloader::new(client)
        .download(
            &DownloadRequest {
                url: &asset.url,
                destination: &destination,
                sha256: Some(&hash),
            },
            &control,
        )
        .unwrap();
    assert_eq!(fs::read(destination).unwrap(), body);
    let asset_request = &api.requests()[1];
    assert_eq!(asset_request.headers["authorization"], "Bearer test-token");
    assert_eq!(asset_request.headers["accept"], "application/octet-stream");
    let stored = &storage.requests()[0];
    assert!(!stored.headers.contains_key("authorization"));
    assert_eq!(stored.headers["range"], "bytes=3-");
}

#[test]
fn failures_keep_the_http_status_and_direct_external_downloads_get_no_token() {
    let api = MockServer::start(|_| Reply::status(404));
    let outside = MockServer::start(|_| Reply::status(404));
    let dir = tempfile::tempdir().unwrap();
    let cancel = CancelToken::new();
    let control = TransferControl {
        cancel: &cancel,
        progress: &|_| {},
    };
    let error = HttpDownloader::new(client(&api))
        .download(
            &DownloadRequest {
                url: &HttpUrl::new(outside.url("/binary")).unwrap(),
                destination: &dir.path().join("file.bin"),
                sha256: None,
            },
            &control,
        )
        .unwrap_err();
    assert!(error.to_string().contains("404"));
    assert!(!outside.requests()[0].headers.contains_key("authorization"));
}

fn client(api: &MockServer) -> HttpClient {
    HttpClient::new(
        ClientConfig::new("test")
            .with_endpoints(Endpoints::new(&api.url(""), "https://crates.io/api/v1/"))
            .with_github_token(GithubToken::new("test-token").unwrap()),
    )
}

fn private_api(request: &Request, storage: &str) -> Reply {
    if request.headers.get("authorization").map(String::as_str) != Some("Bearer test-token") {
        return Reply::status(404);
    }
    if request.path == "/repos/o/r/releases/assets/7" {
        if request.headers.get("accept").map(String::as_str) != Some("application/octet-stream") {
            return Reply::status(406);
        }
        return Reply::status(302).header("Location", storage);
    }
    let url = format!(
        "http://{}/repos/o/r/releases/assets/7",
        request.headers["host"]
    );
    Reply::ok(serde_json::json!([{
        "tag_name":"v1.0.0", "draft":false, "prerelease":false,
        "assets":[{"name":"file.bin", "url":url, "browser_download_url":"https://github.invalid/private/file.bin"}]
    }]).to_string())
}

#[test]
#[ignore = "explicit network smoke using an authorized release token"]
fn a_published_release_asset_is_downloaded_and_verified() {
    let repo = RepoSlug::new(std::env::var("SUBSTRATE_RELEASE_REPO").expect("release repository"))
        .unwrap();
    let tag = substrate_installer_domain::versions::Version::new(
        &std::env::var("SUBSTRATE_RELEASE_TAG").expect("release tag"),
    )
    .unwrap();
    let name = std::env::var("SUBSTRATE_RELEASE_ASSET").expect("release asset");
    let token =
        GithubToken::new(&std::env::var("GITHUB_TOKEN").expect("authorized GitHub token")).unwrap();
    let client = HttpClient::new(
        ClientConfig::new("substrate-installer-http/smoke").with_github_token(token),
    );
    let releases = HttpReleaseSource::new(client.clone())
        .releases(&Source::GitTag { repo })
        .expect("release listing");
    let release = releases
        .iter()
        .find(|release| release.version == tag)
        .expect("requested release");
    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name.as_str() == name)
        .expect("requested asset");
    let hash = asset.sha256.as_ref().expect("published SHA-256");
    let dir = tempfile::tempdir().unwrap();
    let destination = dir.path().join("verified.bin");
    let cancel = CancelToken::new();
    let control = TransferControl {
        cancel: &cancel,
        progress: &|_| {},
    };
    HttpDownloader::new(client)
        .download(
            &DownloadRequest {
                url: &asset.url,
                destination: &destination,
                sha256: Some(hash),
            },
            &control,
        )
        .expect("authenticated binary download and checksum verification");
    let size = fs::metadata(destination).unwrap().len();
    assert_eq!(asset.size, Some(size));
    println!("Verified release asset: {name}, {size} bytes, SHA-256 matched");
}
