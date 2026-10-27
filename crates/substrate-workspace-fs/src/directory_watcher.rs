//! Watching folders for changes made outside the IDE — a terminal, git,
//! another editor, a build. Watches exactly the folders the host names
//! (not recursively: a file tree names the folders it has listed, an
//! editor the folders of its open files), so a huge build-output folder
//! nobody looks at costs nothing. Events are coalesced: a burst (a
//! checkout, a build writing many files) arrives as one batch of paths.

use std::collections::{BTreeSet, HashSet};
use std::fmt;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::Duration;

use notify::event::{AccessKind, AccessMode, EventKind};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use substrate_lock::Locked;

use crate::FsError;

/// How long a burst may stay quiet before its batch is delivered.
pub const DEFAULT_QUIET: Duration = Duration::from_millis(150);

/// Watches a set of folders; changed paths (files or folders directly in a
/// watched folder, or the folder itself) go to the callback in batches.
pub struct DirectoryWatcher {
    watcher: Mutex<RecommendedWatcher>,
    watched: Mutex<HashSet<PathBuf>>,
}

impl fmt::Debug for DirectoryWatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DirectoryWatcher")
            .field("watched", &*self.watched.locked())
            .finish_non_exhaustive()
    }
}

impl DirectoryWatcher {
    /// A watcher calling `on_change` (from its own thread) with each
    /// batch, sorted and without duplicates, once `quiet` passes without
    /// another change. Watches nothing until [`DirectoryWatcher::set`].
    ///
    /// # Errors
    ///
    /// The path can't be used.
    pub fn new(
        quiet: Duration,
        on_change: impl Fn(Vec<PathBuf>) + Send + 'static,
    ) -> Result<Self, FsError> {
        let (tx, rx) = mpsc::channel::<Vec<PathBuf>>();
        let watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(ev) = res
                && changes_content(&ev.kind)
            {
                tx.send(ev.paths).ok();
            }
        })?;
        std::thread::Builder::new()
            .name("substrate-watch".into())
            .spawn(move || {
                while let Ok(first) = rx.recv() {
                    let mut batch: BTreeSet<PathBuf> = first.into_iter().collect();
                    loop {
                        match rx.recv_timeout(quiet) {
                            Ok(more) => batch.extend(more),
                            Err(RecvTimeoutError::Timeout) => break,
                            Err(RecvTimeoutError::Disconnected) => return,
                        }
                    }
                    on_change(batch.into_iter().collect());
                }
            })
            .map_err(|e| FsError::Watch(notify::Error::generic(&e.to_string())))?;
        Ok(Self {
            watcher: Mutex::new(watcher),
            watched: Mutex::new(HashSet::new()),
        })
    }

    /// Watches exactly `folders` from now on: new ones are added, ones no
    /// longer named dropped. A folder that can't be watched (gone, no
    /// permission) is skipped.
    pub fn set(&self, folders: impl IntoIterator<Item = PathBuf>) {
        let wanted: HashSet<PathBuf> = folders.into_iter().collect();
        let mut watcher = self.watcher.locked();
        let mut watched = self.watched.locked();
        for gone in watched.difference(&wanted) {
            watcher.unwatch(gone).ok();
        }
        watched.retain(|p| wanted.contains(p));
        for new in wanted {
            if !watched.contains(&new) && watcher.watch(&new, RecursiveMode::NonRecursive).is_ok() {
                watched.insert(new);
            }
        }
    }

    /// The folders being watched now.
    pub fn watched(&self) -> Vec<PathBuf> {
        let watched = self.watched.locked();
        let mut out: Vec<PathBuf> = watched.iter().cloned().collect();
        out.sort();
        out
    }
}

/// Opening or reading a file changes nothing; closing it after writing does.
fn changes_content(kind: &EventKind) -> bool {
    match kind {
        EventKind::Access(AccessKind::Close(AccessMode::Write)) => true,
        EventKind::Access(_) => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::time::Instant;

    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("substrate-watch-{name}-{}", std::process::id()));
        std::fs::remove_dir_all(&d).ok();
        std::fs::create_dir_all(&d).unwrap();
        d.canonicalize().unwrap()
    }

    fn wait_for(
        batches: &Mutex<Vec<Vec<PathBuf>>>,
        what: impl Fn(&[Vec<PathBuf>]) -> bool,
    ) -> bool {
        let until = Instant::now() + Duration::from_secs(5);
        while Instant::now() < until {
            if what(&batches.locked()) {
                return true;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        false
    }

    #[test]
    fn reports_changes_in_watched_folders_only() {
        let dir = temp_dir("only");
        let other = dir.join("other");
        std::fs::create_dir(&other).unwrap();
        let batches = Arc::new(Mutex::new(Vec::new()));
        let sink = batches.clone();
        let w = DirectoryWatcher::new(Duration::from_millis(50), move |b| sink.locked().push(b))
            .unwrap();
        w.set([dir.clone()]);
        assert_eq!(w.watched(), std::slice::from_ref(&dir));

        std::fs::write(other.join("ignored.txt"), "x").unwrap();
        let file = dir.join("a.txt");
        std::fs::write(&file, "one").unwrap();
        std::fs::write(&file, "two").unwrap();
        assert!(wait_for(&batches, |b| b
            .iter()
            .flatten()
            .any(|p| p == &file)));
        assert!(
            batches
                .locked()
                .iter()
                .flatten()
                .all(|p| !p.starts_with(&other) || p == &other)
        );

        // Unwatched: no more batches for it.
        w.set([]);
        assert!(w.watched().is_empty());
        batches.locked().clear();
        std::fs::write(&file, "three").unwrap();
        std::thread::sleep(Duration::from_millis(300));
        assert!(batches.locked().is_empty());
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_burst_is_one_batch() {
        let dir = temp_dir("burst");
        let batches = Arc::new(Mutex::new(Vec::new()));
        let sink = batches.clone();
        let w = DirectoryWatcher::new(Duration::from_millis(200), move |b| sink.locked().push(b))
            .unwrap();
        w.set([dir.clone()]);
        for i in 0..20 {
            std::fs::write(dir.join(format!("f{i}.txt")), "x").unwrap();
        }
        assert!(wait_for(&batches, |b| b
            .iter()
            .flatten()
            .filter(|p| p.extension().is_some())
            .count()
            >= 20));
        assert_eq!(batches.locked().len(), 1);
        std::fs::remove_dir_all(&dir).ok();
    }
}
