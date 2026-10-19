//! What Git says to the user (see [`crate::message`]).

crate::messages! {
    git {
        MISSING = "Git isn't installed, or isn't on PATH.",
        CANT_START = "Couldn't start git: {error}",
        AUTH_FAILED = "The remote asked for credentials, and this app can't prompt for them. Set up a Git credential helper (such as Git Credential Manager) or an SSH key, then try again.",
        NO_IDENTITY = "Git doesn't know who you are yet — set your name and email in Tools > Options > Source Control, then commit again.",
        NOT_A_REPOSITORY = "This folder isn't in a Git repository.",
        NO_PUSH_REMOTE = "This repository has no remote to push to.",
        NO_ORIGIN = "This branch has no upstream, and there's no \"origin\" remote to push it to.",
        DETACHED_PUSH = "A detached HEAD can't be pushed — check out a branch first.",
        NO_UPSTREAM = "The current branch has no upstream branch to pull from. Push it first.",
        NOTHING_IN_PROGRESS = "No merge or rebase is in progress.",
        CLONE_TARGET_NOT_EMPTY = "{path} already exists and isn't empty.",
        NOT_SETTABLE = "{key} can't be set from here.",
        /// The line a streamed run starts with.
        RUNNING = "> git {command}",
        /// The line a streamed run ends with when it fails.
        FAILED = "[error] git {command} failed (exit code {code})",
        /// … when a signal ended it.
        KILLED = "[error] git {command} was stopped by a signal",
    }
}
