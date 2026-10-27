//! The write operations hold the lock around their port call.

use std::path::Path;
use std::sync::Mutex;
use std::time::Duration;

use substrate_git_domain::GitError;
use substrate_git_domain::commits::{CommitMode, CommitResult, CommitScope};
use substrate_git_domain::status::{DiscardItem, RepoStatus};

use crate::ports::WorkingTree;
use crate::{WriteLock, changes};

/// A working tree that records when each write starts and ends.
#[derive(Default)]
struct RecordingTree {
    events: Mutex<Vec<&'static str>>,
}

impl RecordingTree {
    fn write(&self, name: &'static str) {
        self.events.lock().expect("events").push(name);
        std::thread::sleep(Duration::from_millis(30));
        self.events.lock().expect("events").push(name);
    }
}

impl WorkingTree for RecordingTree {
    fn status(&self, _: &Path) -> Result<Option<RepoStatus>, GitError> {
        Ok(None)
    }

    fn stage(&self, _: &Path, _: &[String]) -> Result<(), GitError> {
        self.write("stage");
        Ok(())
    }

    fn unstage(&self, _: &Path, _: &[String]) -> Result<(), GitError> {
        self.write("unstage");
        Ok(())
    }

    fn discard(&self, _: &Path, _: &[DiscardItem]) -> Result<(), GitError> {
        self.write("discard");
        Ok(())
    }

    fn commit(
        &self,
        _: &Path,
        message: &str,
        _: CommitMode,
        _: CommitScope,
    ) -> Result<CommitResult, GitError> {
        self.write("commit");
        Ok(CommitResult {
            hash: "abc".into(),
            subject: message.into(),
        })
    }
}

#[test]
fn two_writes_never_overlap() {
    let tree = RecordingTree::default();
    let lock = WriteLock::new();
    let root = Path::new(".");
    std::thread::scope(|scope| {
        scope.spawn(|| changes::stage(&tree, &lock, root, &[]).expect("stage"));
        scope.spawn(|| changes::unstage(&tree, &lock, root, &[]).expect("unstage"));
        scope.spawn(|| changes::discard(&tree, &lock, root, &[]).expect("discard"));
    });
    let events = tree.events.lock().expect("events").clone();
    // Each write's start is followed by its own end, never by another's start.
    assert_eq!(events.len(), 6);
    for pair in events.chunks(2) {
        assert_eq!(pair[0], pair[1]);
    }
}

#[test]
fn a_commit_answers_with_what_it_made() {
    let tree = RecordingTree::default();
    let result = changes::commit(
        &tree,
        &WriteLock::new(),
        Path::new("."),
        "subject",
        (CommitMode::New, CommitScope::Staged),
    )
    .expect("commit");
    assert_eq!(result.subject, "subject");
}
