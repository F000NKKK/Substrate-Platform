//! The JSON answers kept on disk with their `ETag`s.

use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use substrate_installer_domain::releases::Sha256 as DomainDigest;

use crate::cached_answer::CachedAnswer;

/// A folder of answers, one file per address. Best effort: a file that
/// cannot be read or written only means the question is asked again.
#[derive(Debug, Clone)]
pub(crate) struct ResponseCache {
    dir: PathBuf,
}

impl ResponseCache {
    pub(crate) fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    /// The answer kept for `url`.
    pub(crate) fn load(&self, url: &str) -> Option<CachedAnswer> {
        let bytes = fs::read(self.file(url)).ok()?;
        serde_json::from_slice(&bytes).ok()
    }

    /// Keeps `answer` for `url`; written beside and renamed, so a reader
    /// never sees half of it.
    pub(crate) fn store(&self, url: &str, answer: &CachedAnswer) {
        let file = self.file(url);
        let Ok(bytes) = serde_json::to_vec(answer) else {
            return;
        };
        let written = fs::create_dir_all(&self.dir).and_then(|()| write_atomically(&file, &bytes));
        if written.is_err() {
            // Nothing to do: the next question just goes to the server.
            drop(fs::remove_file(file.with_extension("tmp")));
        }
    }

    fn file(&self, url: &str) -> PathBuf {
        let digest = Sha256::digest(url.as_bytes());
        let name = DomainDigest::from_bytes(digest.into());
        self.dir.join(format!("{name}.json"))
    }
}

fn write_atomically(file: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let temporary = file.with_extension("tmp");
    fs::write(&temporary, bytes)?;
    fs::rename(temporary, file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_answer_comes_back_and_a_missing_one_does_not() {
        let dir = tempfile::tempdir().unwrap();
        let cache = ResponseCache::new(dir.path().join("http"));
        assert_eq!(cache.load("https://x.y/a"), None);
        let answer = CachedAnswer {
            etag: Some("\"v1\"".to_owned()),
            body: "[]".to_owned(),
        };
        cache.store("https://x.y/a", &answer);
        assert_eq!(cache.load("https://x.y/a"), Some(answer));
        assert_eq!(cache.load("https://x.y/b"), None);
    }

    #[test]
    fn a_damaged_file_is_a_miss() {
        let dir = tempfile::tempdir().unwrap();
        let cache = ResponseCache::new(dir.path().to_path_buf());
        fs::write(cache.file("https://x.y/a"), b"not json").unwrap();
        assert_eq!(cache.load("https://x.y/a"), None);
    }
}
